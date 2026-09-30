use std::io::{self, Read};

use bytes::{Buf, Bytes};
use flate2::read::{MultiGzDecoder, ZlibDecoder};
use snap::raw::Decoder as SnappyDecoder;
use warp::http::StatusCode;

use super::super::decompression::{LimitedReader, max_decompressed_size};
use crate::{common::http::ErrorMessage, internal_events::HttpDecompressError};

/// Decompresses the body based on the Content-Encoding header.
///
/// Supports gzip, deflate, snappy, zstd, and identity (no compression).
/// Caps decompressed output at 100 MiB to prevent decompression bomb attacks.
pub fn decompress_body(header: Option<&str>, mut body: Bytes) -> Result<Bytes, ErrorMessage> {
    let limit = max_decompressed_size();

    // Check compressed body size first to prevent buffering oversized payloads
    if body.len() > limit {
        return Err(ErrorMessage::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("Request body exceeds limit of {} bytes", limit),
        ));
    }

    if let Some(encodings) = header {
        for encoding in encodings.rsplit(',').map(str::trim) {
            body = match encoding {
                "identity" => body,
                "gzip" => {
                    let mut decoded = Vec::new();
                    let decoder = MultiGzDecoder::new(body.reader());
                    let mut limited = LimitedReader::new(decoder, max_decompressed_size());
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| emit_decompress_error(encoding, error))?;
                    decoded.into()
                }
                "deflate" => {
                    let mut decoded = Vec::new();
                    let decoder = ZlibDecoder::new(body.reader());
                    let mut limited = LimitedReader::new(decoder, max_decompressed_size());
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| emit_decompress_error(encoding, error))?;
                    decoded.into()
                }
                "snappy" => {
                    // Check the declared uncompressed size before allocating
                    let limit = max_decompressed_size();
                    let declared_len = snap::raw::decompress_len(&body).map_err(|error| {
                        emit_decompress_error(
                            encoding,
                            io::Error::new(io::ErrorKind::InvalidData, error),
                        )
                    })?;
                    if declared_len > limit {
                        return Err(ErrorMessage::new(
                            StatusCode::PAYLOAD_TOO_LARGE,
                            format!("Decompressed snappy body exceeds limit of {} bytes", limit),
                        ));
                    }
                    let decompressed =
                        SnappyDecoder::new()
                            .decompress_vec(&body)
                            .map_err(|error| {
                                emit_decompress_error(
                                    encoding,
                                    io::Error::new(io::ErrorKind::InvalidData, error),
                                )
                            })?;
                    decompressed.into()
                }
                "zstd" => {
                    let mut decoded = Vec::new();
                    let limit = max_decompressed_size();
                    // Create decoder and cap the window to prevent large window allocation.
                    // For HTTP Content-Encoding: zstd, RFC 9659 recommends max 8 MB (log 23).
                    let mut decoder = zstd::stream::read::Decoder::new(body.reader())
                        .map_err(|error| emit_decompress_error(encoding, error))?;
                    decoder
                        .window_log_max(23) // 8 MB window max per RFC 9659
                        .map_err(|error| emit_decompress_error(encoding, error))?;
                    let mut limited = LimitedReader::new(decoder, limit);
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| emit_decompress_error(encoding, error))?;
                    decoded.into()
                }
                encoding => {
                    return Err(ErrorMessage::new(
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        format!("Unsupported encoding {encoding}"),
                    ));
                }
            }
        }
    }

    // Final check for identity/no-encoding case
    let limit = max_decompressed_size();
    if body.len() > limit {
        return Err(ErrorMessage::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("Payload exceeds limit of {} bytes", limit),
        ));
    }

    Ok(body)
}

pub fn emit_decompress_error(encoding: &str, error: impl std::error::Error) -> ErrorMessage {
    emit!(HttpDecompressError {
        encoding,
        error: &error
    });
    ErrorMessage::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("Failed decompressing payload with {encoding} decoder."),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    #[test]
    fn test_decompress_bomb_protection_gzip() {
        // Create a decompression bomb: compress 150 MB of zeros into a small payload
        let bomb_size = 150 * 1024 * 1024; // 150 MiB
        let zeros = vec![0u8; bomb_size];

        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&zeros).unwrap();
        let compressed = encoder.finish().unwrap();

        // The compressed size should be very small (zeros compress well)
        assert!(
            compressed.len() < 1024 * 1024,
            "Compressed size should be < 1 MiB"
        );

        // Try to decompress - should fail due to size limit
        let result = decompress_body(Some("gzip"), Bytes::from(compressed));

        assert!(result.is_err(), "Should reject decompression bomb");
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_decompress_normal_payload_gzip() {
        // Create a normal-sized compressed payload (1 MiB)
        let normal_size = 1024 * 1024;
        let data = vec![b'A'; normal_size];

        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&data).unwrap();
        let compressed = encoder.finish().unwrap();

        // Should decompress successfully
        let result = decompress_body(Some("gzip"), Bytes::from(compressed));

        assert!(result.is_ok(), "Should accept normal-sized payload");
        let decompressed = result.unwrap();
        assert_eq!(decompressed.len(), normal_size);
    }

    #[test]
    fn test_decompress_bomb_protection_zstd() {
        // Create a zstd decompression bomb
        let bomb_size = 150 * 1024 * 1024; // 150 MiB
        let zeros = vec![0u8; bomb_size];

        let compressed = zstd::encode_all(&zeros[..], 3).unwrap();

        // The compressed size should be very small
        assert!(
            compressed.len() < 1024 * 1024,
            "Compressed size should be < 1 MiB"
        );

        // Try to decompress - should fail due to size limit
        let result = decompress_body(Some("zstd"), Bytes::from(compressed));

        assert!(result.is_err(), "Should reject decompression bomb");
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_decompress_bomb_protection_deflate() {
        // Create a deflate decompression bomb
        let bomb_size = 150 * 1024 * 1024; // 150 MiB
        let zeros = vec![0u8; bomb_size];

        use flate2::write::ZlibEncoder;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&zeros).unwrap();
        let compressed = encoder.finish().unwrap();

        assert!(
            compressed.len() < 1024 * 1024,
            "Compressed size should be < 1 MiB"
        );

        let result = decompress_body(Some("deflate"), Bytes::from(compressed));

        assert!(result.is_err(), "Should reject decompression bomb");
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_decompress_bomb_protection_snappy() {
        // Create a snappy decompression bomb
        let bomb_size = 150 * 1024 * 1024; // 150 MiB
        let zeros = vec![0u8; bomb_size];

        let compressed = snap::raw::Encoder::new().compress_vec(&zeros).unwrap();

        // Snappy doesn't compress zeros as well as gzip, but should still fail
        let result = decompress_body(Some("snappy"), Bytes::from(compressed));

        assert!(result.is_err(), "Should reject decompression bomb");
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[test]
    fn test_identity_payload_exceeds_limit() {
        // Create an uncompressed payload that exceeds the limit
        let oversized = vec![b'A'; 150 * 1024 * 1024];

        let result = decompress_body(Some("identity"), Bytes::from(oversized));

        assert!(result.is_err(), "Should reject oversized identity payload");
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[test]
    fn test_no_encoding_header_exceeds_limit() {
        // No encoding header with oversized payload
        let oversized = vec![b'B'; 150 * 1024 * 1024];

        let result = decompress_body(None, Bytes::from(oversized));

        assert!(
            result.is_err(),
            "Should reject oversized payload with no encoding"
        );
        let err = result.unwrap_err();
        assert_eq!(err.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[test]
    fn test_boundary_at_limit() {
        // Exactly at the limit should succeed
        use crate::sources::util::decompression::DEFAULT_MAX_DECOMPRESSED_SIZE;
        let at_limit = vec![b'C'; DEFAULT_MAX_DECOMPRESSED_SIZE];

        let result = decompress_body(None, Bytes::from(at_limit));
        assert!(result.is_ok(), "Should accept payload exactly at limit");
    }

    #[test]
    fn test_boundary_one_over_limit() {
        // One byte over the limit should fail
        use crate::sources::util::decompression::DEFAULT_MAX_DECOMPRESSED_SIZE;
        let over_limit = vec![b'D'; DEFAULT_MAX_DECOMPRESSED_SIZE + 1];

        let result = decompress_body(None, Bytes::from(over_limit));
        assert!(result.is_err(), "Should reject payload one byte over limit");
    }
}
