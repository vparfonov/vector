// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_metric_data::_put_metric_data_output::PutMetricDataOutputBuilder;

pub use crate::operation::put_metric_data::_put_metric_data_input::PutMetricDataInputBuilder;

impl crate::operation::put_metric_data::builders::PutMetricDataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_metric_data::PutMetricDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_metric_data::PutMetricDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_metric_data();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutMetricData`.
///
/// <p>Publishes metric data points to Amazon CloudWatch. CloudWatch associates the data points with the specified metric. If the specified metric does not exist, CloudWatch creates the metric. When CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_ListMetrics.html">ListMetrics</a>.</p>
/// <p>You can publish either individual data points in the <code>Value</code> field, or arrays of values and the number of times each value occurred during the period by using the <code>Values</code> and <code>Counts</code> fields in the <code>MetricData</code> structure. Using the <code>Values</code> and <code>Counts</code> method enables you to publish up to 150 values per metric with one <code>PutMetricData</code> request, and supports retrieving percentile statistics on this data.</p>
/// <p>Each <code>PutMetricData</code> request is limited to 1 MB in size for HTTP POST requests. You can send a payload compressed by gzip. Each request is also limited to no more than 1000 different metrics.</p>
/// <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of -2^360 to 2^360. In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p>
/// <p>You can use up to 30 dimensions per metric to further clarify what data the metric collects. Each dimension consists of a Name and Value pair. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
/// <p>You specify the time stamp to be associated with each data point. You can specify time stamps that are as much as two weeks before the current date, and as much as 2 hours after the current day and time.</p>
/// <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricData.html">GetMetricData</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html">GetMetricStatistics</a> from the time they are submitted. Data points with time stamps between 3 and 24 hours ago can take as much as 2 hours to become available for for <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricData.html">GetMetricData</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html">GetMetricStatistics</a>.</p>
/// <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p>
/// <ul>
/// <li>
/// <p>The <code>SampleCount</code> value of the statistic set is 1 and <code>Min</code>, <code>Max</code>, and <code>Sum</code> are all equal.</p></li>
/// <li>
/// <p>The <code>Min</code> and <code>Max</code> are equal, and <code>Sum</code> is equal to <code>Min</code> multiplied by <code>SampleCount</code>.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutMetricDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_metric_data::builders::PutMetricDataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_metric_data::PutMetricDataOutput,
        crate::operation::put_metric_data::PutMetricDataError,
    > for PutMetricDataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_metric_data::PutMetricDataOutput,
            crate::operation::put_metric_data::PutMetricDataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutMetricDataFluentBuilder {
    /// Creates a new `PutMetricDataFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutMetricData as a reference.
    pub fn as_input(&self) -> &crate::operation::put_metric_data::builders::PutMetricDataInputBuilder {
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
        crate::operation::put_metric_data::PutMetricDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_metric_data::PutMetricDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_metric_data::PutMetricData::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_metric_data::PutMetricData::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_metric_data::PutMetricDataOutput,
        crate::operation::put_metric_data::PutMetricDataError,
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
    /// <p>The namespace for the metric data. You can use ASCII characters for the namespace, except for control characters which are not supported.</p>
    /// <p>To avoid conflicts with Amazon Web Services service namespaces, you should not specify a namespace that begins with <code>AWS/</code></p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The namespace for the metric data. You can use ASCII characters for the namespace, except for control characters which are not supported.</p>
    /// <p>To avoid conflicts with Amazon Web Services service namespaces, you should not specify a namespace that begins with <code>AWS/</code></p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The namespace for the metric data. You can use ASCII characters for the namespace, except for control characters which are not supported.</p>
    /// <p>To avoid conflicts with Amazon Web Services service namespaces, you should not specify a namespace that begins with <code>AWS/</code></p>
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace()
    }
    ///
    /// Appends an item to `MetricData`.
    ///
    /// To override the contents of this collection use [`set_metric_data`](Self::set_metric_data).
    ///
    /// <p>The data for the metric. The array can include no more than 1000 metrics per call.</p>
    pub fn metric_data(mut self, input: crate::types::MetricDatum) -> Self {
        self.inner = self.inner.metric_data(input);
        self
    }
    /// <p>The data for the metric. The array can include no more than 1000 metrics per call.</p>
    pub fn set_metric_data(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricDatum>>) -> Self {
        self.inner = self.inner.set_metric_data(input);
        self
    }
    /// <p>The data for the metric. The array can include no more than 1000 metrics per call.</p>
    pub fn get_metric_data(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricDatum>> {
        self.inner.get_metric_data()
    }
}
