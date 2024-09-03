// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_secret_value_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_secret_value::GetSecretValueInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.secret_id {
        object.key("SecretId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version_id {
        object.key("VersionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.version_stage {
        object.key("VersionStage").string(var_3.as_str());
    }
    Ok(())
}
