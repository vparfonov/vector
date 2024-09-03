// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_metric_streams::_start_metric_streams_output::StartMetricStreamsOutputBuilder;

pub use crate::operation::start_metric_streams::_start_metric_streams_input::StartMetricStreamsInputBuilder;

impl crate::operation::start_metric_streams::builders::StartMetricStreamsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_metric_streams::StartMetricStreamsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_metric_streams::StartMetricStreamsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_metric_streams();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartMetricStreams`.
///
/// <p>Starts the streaming of metrics for one or more of your metric streams.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartMetricStreamsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_metric_streams::builders::StartMetricStreamsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_metric_streams::StartMetricStreamsOutput,
        crate::operation::start_metric_streams::StartMetricStreamsError,
    > for StartMetricStreamsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_metric_streams::StartMetricStreamsOutput,
            crate::operation::start_metric_streams::StartMetricStreamsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartMetricStreamsFluentBuilder {
    /// Creates a new `StartMetricStreamsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartMetricStreams as a reference.
    pub fn as_input(&self) -> &crate::operation::start_metric_streams::builders::StartMetricStreamsInputBuilder {
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
        crate::operation::start_metric_streams::StartMetricStreamsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_metric_streams::StartMetricStreamsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_metric_streams::StartMetricStreams::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_metric_streams::StartMetricStreams::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_metric_streams::StartMetricStreamsOutput,
        crate::operation::start_metric_streams::StartMetricStreamsError,
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
    ///
    /// Appends an item to `Names`.
    ///
    /// To override the contents of this collection use [`set_names`](Self::set_names).
    ///
    /// <p>The array of the names of metric streams to start streaming.</p>
    /// <p>This is an "all or nothing" operation. If you do not have permission to access all of the metric streams that you list here, then none of the streams that you list in the operation will start streaming.</p>
    pub fn names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.names(input.into());
        self
    }
    /// <p>The array of the names of metric streams to start streaming.</p>
    /// <p>This is an "all or nothing" operation. If you do not have permission to access all of the metric streams that you list here, then none of the streams that you list in the operation will start streaming.</p>
    pub fn set_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_names(input);
        self
    }
    /// <p>The array of the names of metric streams to start streaming.</p>
    /// <p>This is an "all or nothing" operation. If you do not have permission to access all of the metric streams that you list here, then none of the streams that you list in the operation will start streaming.</p>
    pub fn get_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_names()
    }
}
