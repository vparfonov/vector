// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_metric_data::_get_metric_data_output::GetMetricDataOutputBuilder;

pub use crate::operation::get_metric_data::_get_metric_data_input::GetMetricDataInputBuilder;

impl crate::operation::get_metric_data::builders::GetMetricDataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_metric_data::GetMetricDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_data::GetMetricDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_metric_data();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMetricData`.
///
/// <p>You can use the <code>GetMetricData</code> API to retrieve CloudWatch metric values. The operation can also include a CloudWatch Metrics Insights query, and one or more metric math functions.</p>
/// <p>A <code>GetMetricData</code> operation that does not include a query can retrieve as many as 500 different metrics in a single request, with a total of as many as 100,800 data points. You can also optionally perform metric math expressions on the values of the returned statistics, to create new time series that represent new insights into your data. For example, using Lambda metrics, you could divide the Errors metric by the Invocations metric to get an error rate time series. For more information about metric math expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
/// <p>If you include a Metrics Insights query, each <code>GetMetricData</code> operation can include only one query. But the same <code>GetMetricData</code> operation can also retrieve other metrics. Metrics Insights queries can query only the most recent three hours of metric data. For more information about Metrics Insights, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/query_with_cloudwatch-metrics-insights.html">Query your metrics with CloudWatch Metrics Insights</a>.</p>
/// <p>Calls to the <code>GetMetricData</code> API have a different pricing structure than calls to <code>GetMetricStatistics</code>. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p>
/// <p>Amazon CloudWatch retains metric data as follows:</p>
/// <ul>
/// <li>
/// <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p></li>
/// <li>
/// <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p></li>
/// <li>
/// <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p></li>
/// <li>
/// <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p></li>
/// </ul>
/// <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p>
/// <p>If you omit <code>Unit</code> in your request, all data that was collected with any unit is returned, along with the corresponding units that were specified when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.</p>
/// <p><b>Using Metrics Insights queries with metric math</b></p>
/// <p>You can't mix a Metric Insights query and metric math syntax in the same expression, but you can reference results from a Metrics Insights query within other Metric math expressions. A Metrics Insights query without a GROUP BY clause returns a single time-series (TS), and can be used as input for a metric math expression that expects a single time series. A Metrics Insights query with a GROUP BY clause returns an array of time-series (TS\[\]), and can be used as input for a metric math expression that expects an array of time series.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMetricDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_metric_data::builders::GetMetricDataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_metric_data::GetMetricDataOutput,
        crate::operation::get_metric_data::GetMetricDataError,
    > for GetMetricDataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_metric_data::GetMetricDataOutput,
            crate::operation::get_metric_data::GetMetricDataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMetricDataFluentBuilder {
    /// Creates a new `GetMetricDataFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMetricData as a reference.
    pub fn as_input(&self) -> &crate::operation::get_metric_data::builders::GetMetricDataInputBuilder {
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
        crate::operation::get_metric_data::GetMetricDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_data::GetMetricDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_metric_data::GetMetricData::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_metric_data::GetMetricData::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_metric_data::GetMetricDataOutput,
        crate::operation::get_metric_data::GetMetricDataError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_metric_data::paginator::GetMetricDataPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_metric_data::paginator::GetMetricDataPaginator {
        crate::operation::get_metric_data::paginator::GetMetricDataPaginator::new(self.handle, self.inner)
    }
    ///
    /// Appends an item to `MetricDataQueries`.
    ///
    /// To override the contents of this collection use [`set_metric_data_queries`](Self::set_metric_data_queries).
    ///
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data.</p>
    pub fn metric_data_queries(mut self, input: crate::types::MetricDataQuery) -> Self {
        self.inner = self.inner.metric_data_queries(input);
        self
    }
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data.</p>
    pub fn set_metric_data_queries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>>) -> Self {
        self.inner = self.inner.set_metric_data_queries(input);
        self
    }
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data.</p>
    pub fn get_metric_data_queries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>> {
        self.inner.get_metric_data_queries()
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp.</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp.</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp.</p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>
    /// <li>
    /// <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>
    /// <li>
    /// <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    /// <p>If you omit this parameter, the default of <code>TimestampDescending</code> is used.</p>
    pub fn scan_by(mut self, input: crate::types::ScanBy) -> Self {
        self.inner = self.inner.scan_by(input);
        self
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    /// <p>If you omit this parameter, the default of <code>TimestampDescending</code> is used.</p>
    pub fn set_scan_by(mut self, input: ::std::option::Option<crate::types::ScanBy>) -> Self {
        self.inner = self.inner.set_scan_by(input);
        self
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    /// <p>If you omit this parameter, the default of <code>TimestampDescending</code> is used.</p>
    pub fn get_scan_by(&self) -> &::std::option::Option<crate::types::ScanBy> {
        self.inner.get_scan_by()
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn max_datapoints(mut self, input: i32) -> Self {
        self.inner = self.inner.max_datapoints(input);
        self
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn set_max_datapoints(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_datapoints(input);
        self
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn get_max_datapoints(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_datapoints()
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone.</p>
    pub fn label_options(mut self, input: crate::types::LabelOptions) -> Self {
        self.inner = self.inner.label_options(input);
        self
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone.</p>
    pub fn set_label_options(mut self, input: ::std::option::Option<crate::types::LabelOptions>) -> Self {
        self.inner = self.inner.set_label_options(input);
        self
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone.</p>
    pub fn get_label_options(&self) -> &::std::option::Option<crate::types::LabelOptions> {
        self.inner.get_label_options()
    }
}
