// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_suppression_period(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SuppressionPeriod,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if input.value != 0 {
        object.key("value").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.value).into()),
        );
    }
    if let Some(var_1) = &input.suppression_unit {
        object.key("suppressionUnit").string(var_1.as_str());
    }
    Ok(())
}
