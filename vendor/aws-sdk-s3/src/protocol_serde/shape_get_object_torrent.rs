// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_torrent_http_response(
    response: &mut ::aws_smithy_runtime_api::http::Response,
) -> std::result::Result<crate::operation::get_object_torrent::GetObjectTorrentOutput, crate::operation::get_object_torrent::GetObjectTorrentError> {
    let mut _response_body = ::aws_smithy_types::body::SdkBody::taken();
    std::mem::swap(&mut _response_body, response.body_mut());
    let _response_body = &mut _response_body;

    let _response_status = response.status().as_u16();
    let _response_headers = response.headers();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_object_torrent::builders::GetObjectTorrentOutputBuilder::default();
        output = output.set_body(Some(crate::protocol_serde::shape_get_object_torrent_output::de_body_payload(
            _response_body,
        )?));
        output = output.set_request_charged(
            crate::protocol_serde::shape_get_object_torrent_output::de_request_charged_header(_response_headers).map_err(|_| {
                crate::operation::get_object_torrent::GetObjectTorrentError::unhandled(
                    "Failed to parse RequestCharged from header `x-amz-request-charged",
                )
            })?,
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_torrent_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_object_torrent::GetObjectTorrentOutput, crate::operation::get_object_torrent::GetObjectTorrentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_object_torrent::GetObjectTorrentError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_object_torrent::GetObjectTorrentError::generic(generic))
}

pub fn ser_get_object_torrent_headers(
    input: &crate::operation::get_object_torrent::GetObjectTorrentInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.request_payer {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "request_payer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-request-payer", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.expected_bucket_owner {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    Ok(builder)
}
