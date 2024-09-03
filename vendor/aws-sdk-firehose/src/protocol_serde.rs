// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_delivery_stream;

pub(crate) mod shape_delete_delivery_stream;

pub(crate) mod shape_describe_delivery_stream;

pub(crate) mod shape_list_delivery_streams;

pub(crate) mod shape_list_tags_for_delivery_stream;

pub(crate) mod shape_put_record;

pub(crate) mod shape_put_record_batch;

pub(crate) mod shape_start_delivery_stream_encryption;

pub(crate) mod shape_stop_delivery_stream_encryption;

pub(crate) mod shape_tag_delivery_stream;

pub(crate) mod shape_untag_delivery_stream;

pub(crate) mod shape_update_destination;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_create_delivery_stream_input;

pub(crate) mod shape_delete_delivery_stream_input;

pub(crate) mod shape_describe_delivery_stream_input;

pub(crate) mod shape_invalid_argument_exception;

pub(crate) mod shape_invalid_kms_resource_exception;

pub(crate) mod shape_invalid_source_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_delivery_streams_input;

pub(crate) mod shape_list_tags_for_delivery_stream_input;

pub(crate) mod shape_put_record_batch_input;

pub(crate) mod shape_put_record_input;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_start_delivery_stream_encryption_input;

pub(crate) mod shape_stop_delivery_stream_encryption_input;

pub(crate) mod shape_tag_delivery_stream_input;

pub(crate) mod shape_untag_delivery_stream_input;

pub(crate) mod shape_update_destination_input;

pub(crate) mod shape_amazon_open_search_serverless_destination_configuration;

pub(crate) mod shape_amazon_open_search_serverless_destination_update;

pub(crate) mod shape_amazonopensearchservice_destination_configuration;

pub(crate) mod shape_amazonopensearchservice_destination_update;

pub(crate) mod shape_delivery_stream_description;

pub(crate) mod shape_delivery_stream_encryption_configuration_input;

pub(crate) mod shape_delivery_stream_name_list;

pub(crate) mod shape_elasticsearch_destination_configuration;

pub(crate) mod shape_elasticsearch_destination_update;

pub(crate) mod shape_extended_s3_destination_configuration;

pub(crate) mod shape_extended_s3_destination_update;

pub(crate) mod shape_http_endpoint_destination_configuration;

pub(crate) mod shape_http_endpoint_destination_update;

pub(crate) mod shape_iceberg_destination_configuration;

pub(crate) mod shape_iceberg_destination_update;

pub(crate) mod shape_kinesis_stream_source_configuration;

pub(crate) mod shape_list_tags_for_delivery_stream_output_tag_list;

pub(crate) mod shape_msk_source_configuration;

pub(crate) mod shape_put_record_batch_response_entry_list;

pub(crate) mod shape_record;

pub(crate) mod shape_redshift_destination_configuration;

pub(crate) mod shape_redshift_destination_update;

pub(crate) mod shape_s3_destination_configuration;

pub(crate) mod shape_s3_destination_update;

pub(crate) mod shape_snowflake_destination_configuration;

pub(crate) mod shape_snowflake_destination_update;

pub(crate) mod shape_splunk_destination_configuration;

pub(crate) mod shape_splunk_destination_update;

pub(crate) mod shape_tag;

pub(crate) mod shape_amazon_open_search_serverless_buffering_hints;

pub(crate) mod shape_amazon_open_search_serverless_retry_options;

pub(crate) mod shape_amazonopensearchservice_buffering_hints;

pub(crate) mod shape_amazonopensearchservice_retry_options;

pub(crate) mod shape_authentication_configuration;

pub(crate) mod shape_buffering_hints;

pub(crate) mod shape_catalog_configuration;

pub(crate) mod shape_cloud_watch_logging_options;

pub(crate) mod shape_copy_command;

pub(crate) mod shape_data_format_conversion_configuration;

pub(crate) mod shape_delivery_stream_encryption_configuration;

pub(crate) mod shape_destination_description_list;

pub(crate) mod shape_destination_table_configuration;

pub(crate) mod shape_document_id_options;

pub(crate) mod shape_dynamic_partitioning_configuration;

pub(crate) mod shape_elasticsearch_buffering_hints;

pub(crate) mod shape_elasticsearch_retry_options;

pub(crate) mod shape_encryption_configuration;

pub(crate) mod shape_failure_description;

pub(crate) mod shape_http_endpoint_buffering_hints;

pub(crate) mod shape_http_endpoint_configuration;

pub(crate) mod shape_http_endpoint_request_configuration;

pub(crate) mod shape_http_endpoint_retry_options;

pub(crate) mod shape_processing_configuration;

pub(crate) mod shape_put_record_batch_response_entry;

pub(crate) mod shape_redshift_retry_options;

pub(crate) mod shape_retry_options;

pub(crate) mod shape_secrets_manager_configuration;

pub(crate) mod shape_snowflake_buffering_hints;

pub(crate) mod shape_snowflake_retry_options;

pub(crate) mod shape_snowflake_role_configuration;

pub(crate) mod shape_snowflake_vpc_configuration;

pub(crate) mod shape_source_description;

pub(crate) mod shape_splunk_buffering_hints;

pub(crate) mod shape_splunk_retry_options;

pub(crate) mod shape_vpc_configuration;

pub(crate) mod shape_destination_description;

pub(crate) mod shape_http_endpoint_common_attribute;

pub(crate) mod shape_input_format_configuration;

pub(crate) mod shape_kinesis_stream_source_description;

pub(crate) mod shape_kms_encryption_config;

pub(crate) mod shape_msk_source_description;

pub(crate) mod shape_output_format_configuration;

pub(crate) mod shape_processor;

pub(crate) mod shape_schema_configuration;

pub(crate) mod shape_amazon_open_search_serverless_destination_description;

pub(crate) mod shape_amazonopensearchservice_destination_description;

pub(crate) mod shape_deserializer;

pub(crate) mod shape_elasticsearch_destination_description;

pub(crate) mod shape_extended_s3_destination_description;

pub(crate) mod shape_http_endpoint_destination_description;

pub(crate) mod shape_iceberg_destination_description;

pub(crate) mod shape_processor_parameter;

pub(crate) mod shape_redshift_destination_description;

pub(crate) mod shape_s3_destination_description;

pub(crate) mod shape_serializer;

pub(crate) mod shape_snowflake_destination_description;

pub(crate) mod shape_splunk_destination_description;

pub(crate) mod shape_destination_table_configuration_list;

pub(crate) mod shape_hive_json_ser_de;

pub(crate) mod shape_http_endpoint_description;

pub(crate) mod shape_open_x_json_ser_de;

pub(crate) mod shape_orc_ser_de;

pub(crate) mod shape_parquet_ser_de;

pub(crate) mod shape_vpc_configuration_description;

pub(crate) mod shape_http_endpoint_common_attributes_list;

pub(crate) mod shape_processor_list;

pub(crate) mod shape_security_group_id_list;

pub(crate) mod shape_subnet_id_list;

pub(crate) mod shape_list_of_non_empty_strings_without_whitespace;

pub(crate) mod shape_processor_parameter_list;

pub(crate) mod shape_column_to_json_key_mappings;

pub(crate) mod shape_list_of_non_empty_strings;
