// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_log_events_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_log_events::PutLogEventsOutput, crate::operation::put_log_events::PutLogEventsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::put_log_events::PutLogEventsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::put_log_events::PutLogEventsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::put_log_events::PutLogEventsError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::put_log_events::PutLogEventsError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                        .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnrecognizedClientException" => crate::operation::put_log_events::PutLogEventsError::UnrecognizedClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnrecognizedClientExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unrecognized_client_exception::de_unrecognized_client_exception_json_err(_response_body, output)
                        .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSequenceTokenException" => crate::operation::put_log_events::PutLogEventsError::InvalidSequenceTokenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSequenceTokenExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_sequence_token_exception::de_invalid_sequence_token_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DataAlreadyAcceptedException" => crate::operation::put_log_events::PutLogEventsError::DataAlreadyAcceptedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DataAlreadyAcceptedExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_data_already_accepted_exception::de_data_already_accepted_exception_json_err(_response_body, output)
                        .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::put_log_events::PutLogEventsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_log_events_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_log_events::PutLogEventsOutput, crate::operation::put_log_events::PutLogEventsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_log_events::builders::PutLogEventsOutputBuilder::default();
        output = crate::protocol_serde::shape_put_log_events::de_put_log_events(_response_body, output)
            .map_err(crate::operation::put_log_events::PutLogEventsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_put_log_events_input(
    input: &crate::operation::put_log_events::PutLogEventsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_log_events_input::ser_put_log_events_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_put_log_events(
    value: &[u8],
    mut builder: crate::operation::put_log_events::builders::PutLogEventsOutputBuilder,
) -> Result<crate::operation::put_log_events::builders::PutLogEventsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "nextSequenceToken" => {
                    builder = builder.set_next_sequence_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "rejectedLogEventsInfo" => {
                    builder = builder.set_rejected_log_events_info(
                        crate::protocol_serde::shape_rejected_log_events_info::de_rejected_log_events_info(tokens)?,
                    );
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
