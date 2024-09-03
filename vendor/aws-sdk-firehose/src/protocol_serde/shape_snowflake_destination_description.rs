// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_snowflake_destination_description<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::SnowflakeDestinationDescription>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SnowflakeDestinationDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AccountUrl" => {
                            builder = builder.set_account_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "User" => {
                            builder = builder.set_user(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Database" => {
                            builder = builder.set_database(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Schema" => {
                            builder = builder.set_schema(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Table" => {
                            builder = builder.set_table(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SnowflakeRoleConfiguration" => {
                            builder = builder.set_snowflake_role_configuration(
                                crate::protocol_serde::shape_snowflake_role_configuration::de_snowflake_role_configuration(tokens)?,
                            );
                        }
                        "DataLoadingOption" => {
                            builder = builder.set_data_loading_option(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SnowflakeDataLoadingOption::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "MetaDataColumnName" => {
                            builder = builder.set_meta_data_column_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ContentColumnName" => {
                            builder = builder.set_content_column_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SnowflakeVpcConfiguration" => {
                            builder = builder.set_snowflake_vpc_configuration(
                                crate::protocol_serde::shape_snowflake_vpc_configuration::de_snowflake_vpc_configuration(tokens)?,
                            );
                        }
                        "CloudWatchLoggingOptions" => {
                            builder = builder.set_cloud_watch_logging_options(
                                crate::protocol_serde::shape_cloud_watch_logging_options::de_cloud_watch_logging_options(tokens)?,
                            );
                        }
                        "ProcessingConfiguration" => {
                            builder = builder.set_processing_configuration(
                                crate::protocol_serde::shape_processing_configuration::de_processing_configuration(tokens)?,
                            );
                        }
                        "RoleARN" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RetryOptions" => {
                            builder =
                                builder.set_retry_options(crate::protocol_serde::shape_snowflake_retry_options::de_snowflake_retry_options(tokens)?);
                        }
                        "S3BackupMode" => {
                            builder = builder.set_s3_backup_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SnowflakeS3BackupMode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "S3DestinationDescription" => {
                            builder = builder.set_s3_destination_description(
                                crate::protocol_serde::shape_s3_destination_description::de_s3_destination_description(tokens)?,
                            );
                        }
                        "SecretsManagerConfiguration" => {
                            builder = builder.set_secrets_manager_configuration(
                                crate::protocol_serde::shape_secrets_manager_configuration::de_secrets_manager_configuration(tokens)?,
                            );
                        }
                        "BufferingHints" => {
                            builder = builder.set_buffering_hints(
                                crate::protocol_serde::shape_snowflake_buffering_hints::de_snowflake_buffering_hints(tokens)?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
