// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_permission_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::remove_permission::RemovePermissionOutput, crate::operation::remove_permission::RemovePermissionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::remove_permission::RemovePermissionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidAddress" => crate::operation::remove_permission::RemovePermissionError::InvalidAddress({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidAddressBuilder::default();
                output = crate::protocol_serde::shape_invalid_address::de_invalid_address_json_err(_response_body, output)
                    .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSecurity" => crate::operation::remove_permission::RemovePermissionError::InvalidSecurity({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSecurityBuilder::default();
                output = crate::protocol_serde::shape_invalid_security::de_invalid_security_json_err(_response_body, output)
                    .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.NonExistentQueue" => crate::operation::remove_permission::RemovePermissionError::QueueDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::QueueDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RequestThrottled" => crate::operation::remove_permission::RemovePermissionError::RequestThrottled({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RequestThrottledBuilder::default();
                output = crate::protocol_serde::shape_request_throttled::de_request_throttled_json_err(_response_body, output)
                    .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.UnsupportedOperation" => crate::operation::remove_permission::RemovePermissionError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationBuilder::default();
                output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
                    .map_err(crate::operation::remove_permission::RemovePermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::remove_permission::RemovePermissionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_permission_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::remove_permission::RemovePermissionOutput, crate::operation::remove_permission::RemovePermissionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_permission::builders::RemovePermissionOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_remove_permission_input(
    input: &crate::operation::remove_permission::RemovePermissionInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_permission_input::ser_remove_permission_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
