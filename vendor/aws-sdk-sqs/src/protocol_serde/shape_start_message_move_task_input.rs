// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_message_move_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_message_move_task::StartMessageMoveTaskInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_arn {
        object.key("SourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_arn {
        object.key("DestinationArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_number_of_messages_per_second {
        object.key("MaxNumberOfMessagesPerSecond").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    Ok(())
}
