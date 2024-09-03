// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_enhanced_monitoring::_disable_enhanced_monitoring_output::DisableEnhancedMonitoringOutputBuilder;

pub use crate::operation::disable_enhanced_monitoring::_disable_enhanced_monitoring_input::DisableEnhancedMonitoringInputBuilder;

impl crate::operation::disable_enhanced_monitoring::builders::DisableEnhancedMonitoringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_enhanced_monitoring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableEnhancedMonitoring`.
///
/// <p>Disables enhanced monitoring.</p><note>
/// <p>When invoking this API, you must use either the <code>StreamARN</code> or the <code>StreamName</code> parameter, or both. It is recommended that you use the <code>StreamARN</code> input parameter when you invoke this API.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableEnhancedMonitoringFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_enhanced_monitoring::builders::DisableEnhancedMonitoringInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput,
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringError,
    > for DisableEnhancedMonitoringFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput,
            crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableEnhancedMonitoringFluentBuilder {
    /// Creates a new `DisableEnhancedMonitoringFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableEnhancedMonitoring as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_enhanced_monitoring::builders::DisableEnhancedMonitoringInputBuilder {
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
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoring::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoring::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringOutput,
        crate::operation::disable_enhanced_monitoring::DisableEnhancedMonitoringError,
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
    /// <p>The name of the Kinesis data stream for which to disable enhanced monitoring.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the Kinesis data stream for which to disable enhanced monitoring.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the Kinesis data stream for which to disable enhanced monitoring.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    ///
    /// Appends an item to `ShardLevelMetrics`.
    ///
    /// To override the contents of this collection use [`set_shard_level_metrics`](Self::set_shard_level_metrics).
    ///
    /// <p>List of shard-level metrics to disable.</p>
    /// <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" disables every metric.</p>
    /// <ul>
    /// <li>
    /// <p><code>IncomingBytes</code></p></li>
    /// <li>
    /// <p><code>IncomingRecords</code></p></li>
    /// <li>
    /// <p><code>OutgoingBytes</code></p></li>
    /// <li>
    /// <p><code>OutgoingRecords</code></p></li>
    /// <li>
    /// <p><code>WriteProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>ReadProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>IteratorAgeMilliseconds</code></p></li>
    /// <li>
    /// <p><code>ALL</code></p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    pub fn shard_level_metrics(mut self, input: crate::types::MetricsName) -> Self {
        self.inner = self.inner.shard_level_metrics(input);
        self
    }
    /// <p>List of shard-level metrics to disable.</p>
    /// <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" disables every metric.</p>
    /// <ul>
    /// <li>
    /// <p><code>IncomingBytes</code></p></li>
    /// <li>
    /// <p><code>IncomingRecords</code></p></li>
    /// <li>
    /// <p><code>OutgoingBytes</code></p></li>
    /// <li>
    /// <p><code>OutgoingRecords</code></p></li>
    /// <li>
    /// <p><code>WriteProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>ReadProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>IteratorAgeMilliseconds</code></p></li>
    /// <li>
    /// <p><code>ALL</code></p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    pub fn set_shard_level_metrics(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricsName>>) -> Self {
        self.inner = self.inner.set_shard_level_metrics(input);
        self
    }
    /// <p>List of shard-level metrics to disable.</p>
    /// <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" disables every metric.</p>
    /// <ul>
    /// <li>
    /// <p><code>IncomingBytes</code></p></li>
    /// <li>
    /// <p><code>IncomingRecords</code></p></li>
    /// <li>
    /// <p><code>OutgoingBytes</code></p></li>
    /// <li>
    /// <p><code>OutgoingRecords</code></p></li>
    /// <li>
    /// <p><code>WriteProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>ReadProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p><code>IteratorAgeMilliseconds</code></p></li>
    /// <li>
    /// <p><code>ALL</code></p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    pub fn get_shard_level_metrics(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricsName>> {
        self.inner.get_shard_level_metrics()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_arn()
    }
}
