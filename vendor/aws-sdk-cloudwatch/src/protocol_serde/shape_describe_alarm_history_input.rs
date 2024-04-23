// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_alarm_history_input_input_input(
    input: &crate::operation::describe_alarm_history::DescribeAlarmHistoryInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeAlarmHistory", "2010-08-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AlarmName");
    if let Some(var_2) = &input.alarm_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AlarmTypes");
    if let Some(var_4) = &input.alarm_types {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5.as_str());
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("HistoryItemType");
    if let Some(var_9) = &input.history_item_type {
        scope_8.string(var_9.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("StartDate");
    if let Some(var_11) = &input.start_date {
        scope_10.date_time(var_11, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("EndDate");
    if let Some(var_13) = &input.end_date {
        scope_12.date_time(var_13, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("MaxRecords");
    if let Some(var_15) = &input.max_records {
        scope_14.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("NextToken");
    if let Some(var_17) = &input.next_token {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("ScanBy");
    if let Some(var_19) = &input.scan_by {
        scope_18.string(var_19.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
