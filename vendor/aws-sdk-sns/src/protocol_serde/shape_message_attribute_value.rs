// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_message_attribute_value(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::MessageAttributeValue,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DataType");
    {
        scope_1.string(&input.data_type);
    }
    #[allow(unused_mut)]
    let mut scope_2 = writer.prefix("StringValue");
    if let Some(var_3) = &input.string_value {
        scope_2.string(var_3);
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("BinaryValue");
    if let Some(var_5) = &input.binary_value {
        scope_4.string(&::aws_smithy_types::base64::encode(var_5));
    }
    Ok(())
}
