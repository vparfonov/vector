// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_regions_from_replication_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput,
    crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceError" => crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::InternalServiceError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error::de_internal_service_error_json_err(_response_body, output)
                    .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => {
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                            .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_regions_from_replication_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput,
    crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationOutputBuilder::default();
        output = crate::protocol_serde::shape_remove_regions_from_replication::de_remove_regions_from_replication(_response_body, output)
            .map_err(crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_remove_regions_from_replication_input(
    input: &crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_regions_from_replication_input::ser_remove_regions_from_replication_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_remove_regions_from_replication(
    value: &[u8],
    mut builder: crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationOutputBuilder,
) -> Result<
    crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ARN" => {
                    builder = builder.set_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ReplicationStatus" => {
                    builder = builder
                        .set_replication_status(crate::protocol_serde::shape_replication_status_list_type::de_replication_status_list_type(tokens)?);
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
