// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_delivery_destination_policy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_delivery_destination_policy::PutDeliveryDestinationPolicyInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.delivery_destination_name {
        object.key("deliveryDestinationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.delivery_destination_policy {
        object.key("deliveryDestinationPolicy").string(var_2.as_str());
    }
    Ok(())
}
