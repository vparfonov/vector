// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::decrease_stream_retention_period::_decrease_stream_retention_period_output::DecreaseStreamRetentionPeriodOutputBuilder;

pub use crate::operation::decrease_stream_retention_period::_decrease_stream_retention_period_input::DecreaseStreamRetentionPeriodInputBuilder;

impl DecreaseStreamRetentionPeriodInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.decrease_stream_retention_period();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DecreaseStreamRetentionPeriod`.
///
/// <p>Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <note>
/// <p>When invoking this API, you must use either the <code>StreamARN</code> or the <code>StreamName</code> parameter, or both. It is recommended that you use the <code>StreamARN</code> input parameter when you invoke this API.</p>
/// </note>
/// <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DecreaseStreamRetentionPeriodFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput,
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError,
    > for DecreaseStreamRetentionPeriodFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput,
            crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DecreaseStreamRetentionPeriodFluentBuilder {
    /// Creates a new `DecreaseStreamRetentionPeriod`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DecreaseStreamRetentionPeriod as a reference.
    pub fn as_input(&self) -> &crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodInputBuilder {
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
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriod::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriod::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput,
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the stream to modify.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream to modify.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the stream to modify.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    /// <p>The new retention period of the stream, in hours. Must be less than the current retention period.</p>
    pub fn retention_period_hours(mut self, input: i32) -> Self {
        self.inner = self.inner.retention_period_hours(input);
        self
    }
    /// <p>The new retention period of the stream, in hours. Must be less than the current retention period.</p>
    pub fn set_retention_period_hours(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_retention_period_hours(input);
        self
    }
    /// <p>The new retention period of the stream, in hours. Must be less than the current retention period.</p>
    pub fn get_retention_period_hours(&self) -> &::std::option::Option<i32> {
        self.inner.get_retention_period_hours()
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
