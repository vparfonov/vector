// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_head_object_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::head_object::HeadObjectOutput, crate::operation::head_object::HeadObjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::head_object::HeadObjectError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::head_object::HeadObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NotFound" => crate::operation::head_object::HeadObjectError::NotFound({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundBuilder::default();
                output = crate::protocol_serde::shape_not_found::de_not_found_xml_err(_response_body, output)
                    .map_err(crate::operation::head_object::HeadObjectError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::head_object::HeadObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_head_object_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::head_object::HeadObjectOutput, crate::operation::head_object::HeadObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::head_object::builders::HeadObjectOutputBuilder::default();
        output = output.set_accept_ranges(
            crate::protocol_serde::shape_head_object_output::de_accept_ranges_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse AcceptRanges from header `accept-ranges"))?,
        );
        output = output.set_archive_status(
            crate::protocol_serde::shape_head_object_output::de_archive_status_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ArchiveStatus from header `x-amz-archive-status")
            })?,
        );
        output = output.set_bucket_key_enabled(
            crate::protocol_serde::shape_head_object_output::de_bucket_key_enabled_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse BucketKeyEnabled from header `x-amz-server-side-encryption-bucket-key-enabled",
                )
            })?,
        );
        output = output.set_cache_control(
            crate::protocol_serde::shape_head_object_output::de_cache_control_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse CacheControl from header `Cache-Control"))?,
        );
        output = output.set_checksum_crc32(
            crate::protocol_serde::shape_head_object_output::de_checksum_crc32_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ChecksumCRC32 from header `x-amz-checksum-crc32")
            })?,
        );
        output = output.set_checksum_crc32_c(
            crate::protocol_serde::shape_head_object_output::de_checksum_crc32_c_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ChecksumCRC32C from header `x-amz-checksum-crc32c")
            })?,
        );
        output = output.set_checksum_sha1(
            crate::protocol_serde::shape_head_object_output::de_checksum_sha1_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ChecksumSHA1 from header `x-amz-checksum-sha1")
            })?,
        );
        output = output.set_checksum_sha256(
            crate::protocol_serde::shape_head_object_output::de_checksum_sha256_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ChecksumSHA256 from header `x-amz-checksum-sha256")
            })?,
        );
        output = output.set_content_disposition(
            crate::protocol_serde::shape_head_object_output::de_content_disposition_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentDisposition from header `Content-Disposition")
            })?,
        );
        output = output.set_content_encoding(
            crate::protocol_serde::shape_head_object_output::de_content_encoding_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentEncoding from header `Content-Encoding")
            })?,
        );
        output = output.set_content_language(
            crate::protocol_serde::shape_head_object_output::de_content_language_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentLanguage from header `Content-Language")
            })?,
        );
        output = output.set_content_length(
            crate::protocol_serde::shape_head_object_output::de_content_length_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentLength from header `Content-Length")
            })?,
        );
        output = output.set_content_type(
            crate::protocol_serde::shape_head_object_output::de_content_type_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentType from header `Content-Type"))?,
        );
        output = output.set_delete_marker(
            crate::protocol_serde::shape_head_object_output::de_delete_marker_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse DeleteMarker from header `x-amz-delete-marker")
            })?,
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_head_object_output::de_e_tag_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ETag from header `ETag"))?,
        );
        output = output.set_expiration(
            crate::protocol_serde::shape_head_object_output::de_expiration_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse Expiration from header `x-amz-expiration"))?,
        );
        output = output.set_expires(
            crate::protocol_serde::shape_head_object_output::de_expires_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse Expires from header `Expires"))?,
        );
        output = output.set_expires_string(
            crate::protocol_serde::shape_head_object_output::de_expires_string_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ExpiresString from header `ExpiresString"))?,
        );
        output = output.set_last_modified(
            crate::protocol_serde::shape_head_object_output::de_last_modified_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse LastModified from header `Last-Modified"))?,
        );
        output = output.set_metadata(
            crate::protocol_serde::shape_head_object_output::de_metadata_prefix_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse Metadata from prefix header `x-amz-meta-"))?,
        );
        output = output.set_missing_meta(
            crate::protocol_serde::shape_head_object_output::de_missing_meta_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse MissingMeta from header `x-amz-missing-meta")
            })?,
        );
        output = output.set_object_lock_legal_hold_status(
            crate::protocol_serde::shape_head_object_output::de_object_lock_legal_hold_status_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse ObjectLockLegalHoldStatus from header `x-amz-object-lock-legal-hold",
                )
            })?,
        );
        output = output.set_object_lock_mode(
            crate::protocol_serde::shape_head_object_output::de_object_lock_mode_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ObjectLockMode from header `x-amz-object-lock-mode")
            })?,
        );
        output = output.set_object_lock_retain_until_date(
            crate::protocol_serde::shape_head_object_output::de_object_lock_retain_until_date_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse ObjectLockRetainUntilDate from header `x-amz-object-lock-retain-until-date",
                )
            })?,
        );
        output = output.set_parts_count(
            crate::protocol_serde::shape_head_object_output::de_parts_count_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse PartsCount from header `x-amz-mp-parts-count")
            })?,
        );
        output = output.set_replication_status(
            crate::protocol_serde::shape_head_object_output::de_replication_status_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ReplicationStatus from header `x-amz-replication-status")
            })?,
        );
        output = output.set_request_charged(
            crate::protocol_serde::shape_head_object_output::de_request_charged_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged")
            })?,
        );
        output = output.set_restore(
            crate::protocol_serde::shape_head_object_output::de_restore_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse Restore from header `x-amz-restore"))?,
        );
        output = output.set_sse_customer_algorithm(
            crate::protocol_serde::shape_head_object_output::de_sse_customer_algorithm_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse SSECustomerAlgorithm from header `x-amz-server-side-encryption-customer-algorithm",
                )
            })?,
        );
        output = output.set_sse_customer_key_md5(
            crate::protocol_serde::shape_head_object_output::de_sse_customer_key_md5_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse SSECustomerKeyMD5 from header `x-amz-server-side-encryption-customer-key-MD5",
                )
            })?,
        );
        output = output.set_ssekms_key_id(
            crate::protocol_serde::shape_head_object_output::de_ssekms_key_id_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse SSEKMSKeyId from header `x-amz-server-side-encryption-aws-kms-key-id",
                )
            })?,
        );
        output = output.set_server_side_encryption(
            crate::protocol_serde::shape_head_object_output::de_server_side_encryption_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse ServerSideEncryption from header `x-amz-server-side-encryption",
                )
            })?,
        );
        output = output.set_storage_class(
            crate::protocol_serde::shape_head_object_output::de_storage_class_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled("Failed to parse StorageClass from header `x-amz-storage-class")
            })?,
        );
        output = output.set_version_id(
            crate::protocol_serde::shape_head_object_output::de_version_id_header(_response_headers)
                .map_err(|_| crate::operation::head_object::HeadObjectError::unhandled("Failed to parse VersionId from header `x-amz-version-id"))?,
        );
        output = output.set_website_redirect_location(
            crate::protocol_serde::shape_head_object_output::de_website_redirect_location_header(_response_headers).map_err(|_| {
                crate::operation::head_object::HeadObjectError::unhandled(
                    "Failed to parse WebsiteRedirectLocation from header `x-amz-website-redirect-location",
                )
            })?,
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_head_object_headers(
    input: &crate::operation::head_object::HeadObjectInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.if_match {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "if_match",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("If-Match", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.if_modified_since {
        let formatted_4 = inner_3.fmt(::aws_smithy_types::date_time::Format::HttpDate)?;
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "if_modified_since",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("If-Modified-Since", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.if_none_match {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "if_none_match",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("If-None-Match", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.if_unmodified_since {
        let formatted_8 = inner_7.fmt(::aws_smithy_types::date_time::Format::HttpDate)?;
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "if_unmodified_since",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("If-Unmodified-Since", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_9) = &input.range {
        let formatted_10 = inner_9.as_str();
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "range",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("Range", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_11) = &input.sse_customer_algorithm {
        let formatted_12 = inner_11.as_str();
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_algorithm",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_13) = &input.sse_customer_key {
        let formatted_14 = inner_13.as_str();
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key",
                    format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_15) = &input.sse_customer_key_md5 {
        let formatted_16 = inner_15.as_str();
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key_md5",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_17) = &input.request_payer {
        let formatted_18 = inner_17.as_str();
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "request_payer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-request-payer", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_19) = &input.expected_bucket_owner {
        let formatted_20 = inner_19.as_str();
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_21) = &input.checksum_mode {
        let formatted_22 = inner_21.as_str();
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_mode",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-checksum-mode", header_value);
        }
    }
    Ok(builder)
}
