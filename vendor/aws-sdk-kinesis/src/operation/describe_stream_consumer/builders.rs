// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_stream_consumer::_describe_stream_consumer_output::DescribeStreamConsumerOutputBuilder;

pub use crate::operation::describe_stream_consumer::_describe_stream_consumer_input::DescribeStreamConsumerInputBuilder;

impl crate::operation::describe_stream_consumer::builders::DescribeStreamConsumerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_stream_consumer::DescribeStreamConsumerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stream_consumer::DescribeStreamConsumerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_stream_consumer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeStreamConsumer`.
///
/// <p>To get the description of a registered consumer, provide the ARN of the consumer. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as they don't conflict with each other. If you don't know the name or ARN of the consumer that you want to describe, you can use the <code>ListStreamConsumers</code> operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream.</p>
/// <p>This operation has a limit of 20 transactions per second per stream.</p><note>
/// <p>When making a cross-account call with <code>DescribeStreamConsumer</code>, make sure to provide the ARN of the consumer.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeStreamConsumerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_stream_consumer::builders::DescribeStreamConsumerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_stream_consumer::DescribeStreamConsumerOutput,
        crate::operation::describe_stream_consumer::DescribeStreamConsumerError,
    > for DescribeStreamConsumerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_stream_consumer::DescribeStreamConsumerOutput,
            crate::operation::describe_stream_consumer::DescribeStreamConsumerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeStreamConsumerFluentBuilder {
    /// Creates a new `DescribeStreamConsumerFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeStreamConsumer as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_stream_consumer::builders::DescribeStreamConsumerInputBuilder {
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
        crate::operation::describe_stream_consumer::DescribeStreamConsumerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stream_consumer::DescribeStreamConsumerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_stream_consumer::DescribeStreamConsumer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_stream_consumer::DescribeStreamConsumer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_stream_consumer::DescribeStreamConsumerOutput,
        crate::operation::describe_stream_consumer::DescribeStreamConsumerError,
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
    /// <p>The ARN of the Kinesis data stream that the consumer is registered with. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the Kinesis data stream that the consumer is registered with. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The ARN of the Kinesis data stream that the consumer is registered with. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_arn()
    }
    /// <p>The name that you gave to the consumer.</p>
    pub fn consumer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumer_name(input.into());
        self
    }
    /// <p>The name that you gave to the consumer.</p>
    pub fn set_consumer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumer_name(input);
        self
    }
    /// <p>The name that you gave to the consumer.</p>
    pub fn get_consumer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumer_name()
    }
    /// <p>The ARN returned by Kinesis Data Streams when you registered the consumer.</p>
    pub fn consumer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumer_arn(input.into());
        self
    }
    /// <p>The ARN returned by Kinesis Data Streams when you registered the consumer.</p>
    pub fn set_consumer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumer_arn(input);
        self
    }
    /// <p>The ARN returned by Kinesis Data Streams when you registered the consumer.</p>
    pub fn get_consumer_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumer_arn()
    }
}
