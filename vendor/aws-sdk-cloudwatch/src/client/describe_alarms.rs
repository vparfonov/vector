// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAlarms`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`alarm_names(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::alarm_names) / [`set_alarm_names(Option<Vec::<String>>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_alarm_names):<br>required: **false**<br><p>The names of the alarms to retrieve information about.</p><br>
    ///   - [`alarm_name_prefix(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::alarm_name_prefix) / [`set_alarm_name_prefix(Option<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_alarm_name_prefix):<br>required: **false**<br><p>An alarm name prefix. If you specify this parameter, you receive information about all alarms that have names that start with this prefix.</p> <p>If this parameter is specified, you cannot specify <code>AlarmNames</code>.</p><br>
    ///   - [`alarm_types(AlarmType)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::alarm_types) / [`set_alarm_types(Option<Vec::<AlarmType>>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_alarm_types):<br>required: **false**<br><p>Use this parameter to specify whether you want the operation to return metric alarms or composite alarms. If you omit this parameter, only metric alarms are returned, even if composite alarms exist in the account.</p> <p>For example, if you omit this parameter or specify <code>MetricAlarms</code>, the operation returns only a list of metric alarms. It does not return any composite alarms, even if composite alarms exist in the account.</p> <p>If you specify <code>CompositeAlarms</code>, the operation returns only a list of composite alarms, and does not return any metric alarms.</p><br>
    ///   - [`children_of_alarm_name(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::children_of_alarm_name) / [`set_children_of_alarm_name(Option<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_children_of_alarm_name):<br>required: **false**<br><p>If you use this parameter and specify the name of a composite alarm, the operation returns information about the "children" alarms of the alarm you specify. These are the metric alarms and composite alarms referenced in the <code>AlarmRule</code> field of the composite alarm that you specify in <code>ChildrenOfAlarmName</code>. Information about the composite alarm that you name in <code>ChildrenOfAlarmName</code> is not returned.</p> <p>If you specify <code>ChildrenOfAlarmName</code>, you cannot specify any other parameters in the request except for <code>MaxRecords</code> and <code>NextToken</code>. If you do so, you receive a validation error.</p><note>  <p>Only the <code>Alarm Name</code>, <code>ARN</code>, <code>StateValue</code> (OK/ALARM/INSUFFICIENT_DATA), and <code>StateUpdatedTimestamp</code> information are returned by this operation when you use this parameter. To get complete information about these alarms, perform another <code>DescribeAlarms</code> operation and specify the parent alarm names in the <code>AlarmNames</code> parameter.</p> </note><br>
    ///   - [`parents_of_alarm_name(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::parents_of_alarm_name) / [`set_parents_of_alarm_name(Option<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_parents_of_alarm_name):<br>required: **false**<br><p>If you use this parameter and specify the name of a metric or composite alarm, the operation returns information about the "parent" alarms of the alarm you specify. These are the composite alarms that have <code>AlarmRule</code> parameters that reference the alarm named in <code>ParentsOfAlarmName</code>. Information about the alarm that you specify in <code>ParentsOfAlarmName</code> is not returned.</p> <p>If you specify <code>ParentsOfAlarmName</code>, you cannot specify any other parameters in the request except for <code>MaxRecords</code> and <code>NextToken</code>. If you do so, you receive a validation error.</p><note>  <p>Only the Alarm Name and ARN are returned by this operation when you use this parameter. To get complete information about these alarms, perform another <code>DescribeAlarms</code> operation and specify the parent alarm names in the <code>AlarmNames</code> parameter.</p> </note><br>
    ///   - [`state_value(StateValue)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::state_value) / [`set_state_value(Option<StateValue>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_state_value):<br>required: **false**<br><p>Specify this parameter to receive information only about alarms that are currently in the state that you specify.</p><br>
    ///   - [`action_prefix(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::action_prefix) / [`set_action_prefix(Option<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_action_prefix):<br>required: **false**<br><p>Use this parameter to filter the results of the operation to only those alarms that use a certain alarm action. For example, you could specify the ARN of an SNS topic to find all alarms that send notifications to that topic.</p><br>
    ///   - [`max_records(i32)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_max_records):<br>required: **false**<br><p>The maximum number of alarm descriptions to retrieve.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned by a previous call to indicate that there is more data available.</p><br>
    /// - On success, responds with [`DescribeAlarmsOutput`](crate::operation::describe_alarms::DescribeAlarmsOutput) with field(s):
    ///   - [`composite_alarms(Option<Vec::<CompositeAlarm>>)`](crate::operation::describe_alarms::DescribeAlarmsOutput::composite_alarms): <p>The information about any composite alarms returned by the operation.</p>
    ///   - [`metric_alarms(Option<Vec::<MetricAlarm>>)`](crate::operation::describe_alarms::DescribeAlarmsOutput::metric_alarms): <p>The information about any metric alarms returned by the operation.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_alarms::DescribeAlarmsOutput::next_token): <p>The token that marks the start of the next batch of returned results.</p>
    /// - On failure, responds with [`SdkError<DescribeAlarmsError>`](crate::operation::describe_alarms::DescribeAlarmsError)
    pub fn describe_alarms(&self) -> crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder {
        crate::operation::describe_alarms::builders::DescribeAlarmsFluentBuilder::new(self.handle.clone())
    }
}
