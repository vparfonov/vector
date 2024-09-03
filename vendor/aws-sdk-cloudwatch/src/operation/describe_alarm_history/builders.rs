// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_alarm_history::_describe_alarm_history_output::DescribeAlarmHistoryOutputBuilder;

pub use crate::operation::describe_alarm_history::_describe_alarm_history_input::DescribeAlarmHistoryInputBuilder;

impl crate::operation::describe_alarm_history::builders::DescribeAlarmHistoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_alarm_history::DescribeAlarmHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_alarm_history::DescribeAlarmHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_alarm_history();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAlarmHistory`.
///
/// <p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for either all metric alarms or all composite alarms are returned.</p>
/// <p>CloudWatch retains the history of an alarm even if you delete the alarm.</p>
/// <p>To use this operation and return information about a composite alarm, you must be signed on with the <code>cloudwatch:DescribeAlarmHistory</code> permission that is scoped to <code>*</code>. You can't return information about composite alarms if your <code>cloudwatch:DescribeAlarmHistory</code> permission has a narrower scope.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAlarmHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_alarm_history::builders::DescribeAlarmHistoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_alarm_history::DescribeAlarmHistoryOutput,
        crate::operation::describe_alarm_history::DescribeAlarmHistoryError,
    > for DescribeAlarmHistoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_alarm_history::DescribeAlarmHistoryOutput,
            crate::operation::describe_alarm_history::DescribeAlarmHistoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAlarmHistoryFluentBuilder {
    /// Creates a new `DescribeAlarmHistoryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAlarmHistory as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_alarm_history::builders::DescribeAlarmHistoryInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_alarm_history::DescribeAlarmHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_alarm_history::DescribeAlarmHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_alarm_history::DescribeAlarmHistory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_alarm_history::DescribeAlarmHistory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_alarm_history::DescribeAlarmHistoryOutput,
        crate::operation::describe_alarm_history::DescribeAlarmHistoryError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_alarm_history::paginator::DescribeAlarmHistoryPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_alarm_history::paginator::DescribeAlarmHistoryPaginator {
        crate::operation::describe_alarm_history::paginator::DescribeAlarmHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the alarm.</p>
    pub fn alarm_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alarm_name(input.into());
        self
    }
    /// <p>The name of the alarm.</p>
    pub fn set_alarm_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alarm_name(input);
        self
    }
    /// <p>The name of the alarm.</p>
    pub fn get_alarm_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alarm_name()
    }
    ///
    /// Appends an item to `AlarmTypes`.
    ///
    /// To override the contents of this collection use [`set_alarm_types`](Self::set_alarm_types).
    ///
    /// <p>Use this parameter to specify whether you want the operation to return metric alarms or composite alarms. If you omit this parameter, only metric alarms are returned.</p>
    pub fn alarm_types(mut self, input: crate::types::AlarmType) -> Self {
        self.inner = self.inner.alarm_types(input);
        self
    }
    /// <p>Use this parameter to specify whether you want the operation to return metric alarms or composite alarms. If you omit this parameter, only metric alarms are returned.</p>
    pub fn set_alarm_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AlarmType>>) -> Self {
        self.inner = self.inner.set_alarm_types(input);
        self
    }
    /// <p>Use this parameter to specify whether you want the operation to return metric alarms or composite alarms. If you omit this parameter, only metric alarms are returned.</p>
    pub fn get_alarm_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AlarmType>> {
        self.inner.get_alarm_types()
    }
    /// <p>The type of alarm histories to retrieve.</p>
    pub fn history_item_type(mut self, input: crate::types::HistoryItemType) -> Self {
        self.inner = self.inner.history_item_type(input);
        self
    }
    /// <p>The type of alarm histories to retrieve.</p>
    pub fn set_history_item_type(mut self, input: ::std::option::Option<crate::types::HistoryItemType>) -> Self {
        self.inner = self.inner.set_history_item_type(input);
        self
    }
    /// <p>The type of alarm histories to retrieve.</p>
    pub fn get_history_item_type(&self) -> &::std::option::Option<crate::types::HistoryItemType> {
        self.inner.get_history_item_type()
    }
    /// <p>The starting date to retrieve alarm history.</p>
    pub fn start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_date(input);
        self
    }
    /// <p>The starting date to retrieve alarm history.</p>
    pub fn set_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_date(input);
        self
    }
    /// <p>The starting date to retrieve alarm history.</p>
    pub fn get_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_date()
    }
    /// <p>The ending date to retrieve alarm history.</p>
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The ending date to retrieve alarm history.</p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>The ending date to retrieve alarm history.</p>
    pub fn get_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_date()
    }
    /// <p>The maximum number of alarm history records to retrieve.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of alarm history records to retrieve.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of alarm history records to retrieve.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Specified whether to return the newest or oldest alarm history first. Specify <code>TimestampDescending</code> to have the newest event history returned first, and specify <code>TimestampAscending</code> to have the oldest history returned first.</p>
    pub fn scan_by(mut self, input: crate::types::ScanBy) -> Self {
        self.inner = self.inner.scan_by(input);
        self
    }
    /// <p>Specified whether to return the newest or oldest alarm history first. Specify <code>TimestampDescending</code> to have the newest event history returned first, and specify <code>TimestampAscending</code> to have the oldest history returned first.</p>
    pub fn set_scan_by(mut self, input: ::std::option::Option<crate::types::ScanBy>) -> Self {
        self.inner = self.inner.set_scan_by(input);
        self
    }
    /// <p>Specified whether to return the newest or oldest alarm history first. Specify <code>TimestampDescending</code> to have the newest event history returned first, and specify <code>TimestampAscending</code> to have the oldest history returned first.</p>
    pub fn get_scan_by(&self) -> &::std::option::Option<crate::types::ScanBy> {
        self.inner.get_scan_by()
    }
}
