// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_test_metric_filter_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::test_metric_filter::TestMetricFilterInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter_pattern {
        object.key("filterPattern").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_event_messages {
        let mut array_3 = object.key("logEventMessages").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}
