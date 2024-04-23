// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_metric_stream_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_metric_stream::PutMetricStreamOutput, crate::operation::put_metric_stream::PutMetricStreamError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::operation::put_metric_stream::PutMetricStreamError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceError" => crate::operation::put_metric_stream::PutMetricStreamError::InternalServiceFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceFaultBuilder::default();
                output = crate::protocol_serde::shape_internal_service_fault::de_internal_service_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterCombination" => crate::operation::put_metric_stream::PutMetricStreamError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValue" => crate::operation::put_metric_stream::PutMetricStreamError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingParameter" => crate::operation::put_metric_stream::PutMetricStreamError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MissingRequiredParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::put_metric_stream::PutMetricStreamError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_metric_stream_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_metric_stream::PutMetricStreamOutput, crate::operation::put_metric_stream::PutMetricStreamError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_metric_stream::builders::PutMetricStreamOutputBuilder::default();
        output = crate::protocol_serde::shape_put_metric_stream::de_put_metric_stream(_response_body, output)
            .map_err(crate::operation::put_metric_stream::PutMetricStreamError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_put_metric_stream(
    inp: &[u8],
    mut builder: crate::operation::put_metric_stream::builders::PutMetricStreamOutputBuilder,
) -> Result<crate::operation::put_metric_stream::builders::PutMetricStreamOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("PutMetricStreamResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected PutMetricStreamResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("PutMetricStreamResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected PutMetricStreamResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Arn") /* Arn com.amazonaws.cloudwatch.synthetic#PutMetricStreamOutput$Arn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected PutMetricStreamResult tag"));
    };
    Ok(builder)
}
