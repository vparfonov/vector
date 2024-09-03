// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_http_endpoint_destination_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::HttpEndpointDestinationUpdate,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.endpoint_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("EndpointConfiguration").start_object();
        crate::protocol_serde::shape_http_endpoint_configuration::ser_http_endpoint_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.buffering_hints {
        #[allow(unused_mut)]
        let mut object_4 = object.key("BufferingHints").start_object();
        crate::protocol_serde::shape_http_endpoint_buffering_hints::ser_http_endpoint_buffering_hints(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.request_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("RequestConfiguration").start_object();
        crate::protocol_serde::shape_http_endpoint_request_configuration::ser_http_endpoint_request_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.processing_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("ProcessingConfiguration").start_object();
        crate::protocol_serde::shape_processing_configuration::ser_processing_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.role_arn {
        object.key("RoleARN").string(var_11.as_str());
    }
    if let Some(var_12) = &input.retry_options {
        #[allow(unused_mut)]
        let mut object_13 = object.key("RetryOptions").start_object();
        crate::protocol_serde::shape_http_endpoint_retry_options::ser_http_endpoint_retry_options(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.s3_backup_mode {
        object.key("S3BackupMode").string(var_14.as_str());
    }
    if let Some(var_15) = &input.s3_update {
        #[allow(unused_mut)]
        let mut object_16 = object.key("S3Update").start_object();
        crate::protocol_serde::shape_s3_destination_update::ser_s3_destination_update(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.secrets_manager_configuration {
        #[allow(unused_mut)]
        let mut object_18 = object.key("SecretsManagerConfiguration").start_object();
        crate::protocol_serde::shape_secrets_manager_configuration::ser_secrets_manager_configuration(&mut object_18, var_17)?;
        object_18.finish();
    }
    Ok(())
}
