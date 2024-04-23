// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_destination_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::S3DestinationUpdate,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.role_arn {
        object.key("RoleARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.bucket_arn {
        object.key("BucketARN").string(var_2.as_str());
    }
    if let Some(var_3) = &input.prefix {
        object.key("Prefix").string(var_3.as_str());
    }
    if let Some(var_4) = &input.error_output_prefix {
        object.key("ErrorOutputPrefix").string(var_4.as_str());
    }
    if let Some(var_5) = &input.buffering_hints {
        #[allow(unused_mut)]
        let mut object_6 = object.key("BufferingHints").start_object();
        crate::protocol_serde::shape_buffering_hints::ser_buffering_hints(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.compression_format {
        object.key("CompressionFormat").string(var_7.as_str());
    }
    if let Some(var_8) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("EncryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_11 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}
