// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_elasticsearch_domain_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainOutput,
    crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::BaseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalException" => crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_elasticsearch_domain_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainOutput,
    crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_elasticsearch_domain::builders::DescribeElasticsearchDomainOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_elasticsearch_domain::de_describe_elasticsearch_domain(_response_body, output)
            .map_err(crate::operation::describe_elasticsearch_domain::DescribeElasticsearchDomainError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_elasticsearch_domain_output_output_correct_errors(output).build()
    })
}

pub(crate) fn de_describe_elasticsearch_domain(
    value: &[u8],
    mut builder: crate::operation::describe_elasticsearch_domain::builders::DescribeElasticsearchDomainOutputBuilder,
) -> Result<
    crate::operation::describe_elasticsearch_domain::builders::DescribeElasticsearchDomainOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DomainStatus" => {
                    builder = builder.set_domain_status(crate::protocol_serde::shape_elasticsearch_domain_status::de_elasticsearch_domain_status(
                        tokens,
                    )?);
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
