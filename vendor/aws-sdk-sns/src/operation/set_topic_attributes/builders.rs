// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_topic_attributes::_set_topic_attributes_output::SetTopicAttributesOutputBuilder;

pub use crate::operation::set_topic_attributes::_set_topic_attributes_input::SetTopicAttributesInputBuilder;

impl crate::operation::set_topic_attributes::builders::SetTopicAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_topic_attributes::SetTopicAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_topic_attributes::SetTopicAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_topic_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetTopicAttributes`.
///
/// <p>Allows a topic owner to set an attribute of the topic to a new value.</p><note>
/// <p>To remove the ability to change topic permissions, you must deny permissions to the <code>AddPermission</code>, <code>RemovePermission</code>, and <code>SetTopicAttributes</code> actions in your IAM policy.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetTopicAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_topic_attributes::builders::SetTopicAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_topic_attributes::SetTopicAttributesOutput,
        crate::operation::set_topic_attributes::SetTopicAttributesError,
    > for SetTopicAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_topic_attributes::SetTopicAttributesOutput,
            crate::operation::set_topic_attributes::SetTopicAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetTopicAttributesFluentBuilder {
    /// Creates a new `SetTopicAttributesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetTopicAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::set_topic_attributes::builders::SetTopicAttributesInputBuilder {
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
        crate::operation::set_topic_attributes::SetTopicAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_topic_attributes::SetTopicAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_topic_attributes::SetTopicAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_topic_attributes::SetTopicAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_topic_attributes::SetTopicAttributesOutput,
        crate::operation::set_topic_attributes::SetTopicAttributesError,
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
    /// <p>The ARN of the topic to modify.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_arn(input.into());
        self
    }
    /// <p>The ARN of the topic to modify.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_arn(input);
        self
    }
    /// <p>The ARN of the topic to modify.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_arn()
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>
    /// <li>
    /// <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>
    /// <li>
    /// <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>
    /// <li>
    /// <p>HTTP</p>
    /// <ul>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Amazon Kinesis Data Firehose</p>
    /// <ul>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Lambda</p>
    /// <ul>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Platform application endpoint</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// </ul><note>
    /// <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>
    /// <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>
    /// </note></li>
    /// <li>
    /// <p>Amazon SQS</p>
    /// <ul>
    /// <li>
    /// <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// </ul></li>
    /// </ul><note>
    /// <p>The <endpoint>
    /// SuccessFeedbackRoleArn and
    /// <endpoint>
    /// FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The
    /// <endpoint>
    /// SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the
    /// <endpoint>
    /// FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.
    /// </endpoint>
    /// </endpoint>
    /// </endpoint>
    /// </endpoint></p>
    /// </note>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>
    /// <li>
    /// <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>
    /// <ul>
    /// <li>
    /// <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>
    /// <li>
    /// <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>
    /// <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute_name(input.into());
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>
    /// <li>
    /// <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>
    /// <li>
    /// <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>
    /// <li>
    /// <p>HTTP</p>
    /// <ul>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Amazon Kinesis Data Firehose</p>
    /// <ul>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Lambda</p>
    /// <ul>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Platform application endpoint</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// </ul><note>
    /// <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>
    /// <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>
    /// </note></li>
    /// <li>
    /// <p>Amazon SQS</p>
    /// <ul>
    /// <li>
    /// <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// </ul></li>
    /// </ul><note>
    /// <p>The <endpoint>
    /// SuccessFeedbackRoleArn and
    /// <endpoint>
    /// FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The
    /// <endpoint>
    /// SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the
    /// <endpoint>
    /// FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.
    /// </endpoint>
    /// </endpoint>
    /// </endpoint>
    /// </endpoint></p>
    /// </note>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>
    /// <li>
    /// <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>
    /// <ul>
    /// <li>
    /// <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>
    /// <li>
    /// <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>
    /// <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn set_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attribute_name(input);
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to a platform application endpoint.</p></li>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p></li>
    /// <li>
    /// <p><code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p></li>
    /// <li>
    /// <p><code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic passes through the tracing header it receives from an Amazon SNS publisher to its subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. This is only supported on standard topics.</p></li>
    /// <li>
    /// <p>HTTP</p>
    /// <ul>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// <li>
    /// <p><code>HTTPFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an HTTP endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Amazon Kinesis Data Firehose</p>
    /// <ul>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// <li>
    /// <p><code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Kinesis Data Firehose endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Lambda</p>
    /// <ul>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// <li>
    /// <p><code>LambdaFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Lambda endpoint.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Platform application endpoint</p>
    /// <ul>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// <li>
    /// <p><code>ApplicationFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon Web Services application endpoint.</p></li>
    /// </ul><note>
    /// <p>In addition to being able to configure topic attributes for message delivery status of notification messages sent to Amazon SNS application endpoints, you can also configure application attributes for the delivery status of push notification messages sent to push notification services.</p>
    /// <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>.</p>
    /// </note></li>
    /// <li>
    /// <p>Amazon SQS</p>
    /// <ul>
    /// <li>
    /// <p><code>SQSSuccessFeedbackRoleArn</code> – Indicates successful message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSSuccessFeedbackSampleRate</code> – Indicates percentage of successful messages to sample for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// <li>
    /// <p><code>SQSFailureFeedbackRoleArn</code> – Indicates failed message delivery status for an Amazon SNS topic that is subscribed to an Amazon SQS endpoint.</p></li>
    /// </ul></li>
    /// </ul><note>
    /// <p>The <endpoint>
    /// SuccessFeedbackRoleArn and
    /// <endpoint>
    /// FailureFeedbackRoleArn attributes are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The
    /// <endpoint>
    /// SuccessFeedbackSampleRate attribute is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the
    /// <endpoint>
    /// FailureFeedbackRoleArn attribute, then all failed message deliveries generate CloudWatch Logs.
    /// </endpoint>
    /// </endpoint>
    /// </endpoint>
    /// </endpoint></p>
    /// </note>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p></li>
    /// <li>
    /// <p><code>SignatureVersion</code> – The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS. By default, <code>SignatureVersion</code> is set to <code>1</code>.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ContentBasedDeduplication</code> – Enables content-based deduplication for FIFO topics.</p>
    /// <ul>
    /// <li>
    /// <p>By default, <code>ContentBasedDeduplication</code> is set to <code>false</code>. If you create a FIFO topic and this attribute is <code>false</code>, you must specify a value for the <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action.</p></li>
    /// <li>
    /// <p>When you set <code>ContentBasedDeduplication</code> to <code>true</code>, Amazon SNS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p>
    /// <p>(Optional) To override the generated value, you can specify a value for the <code>MessageDeduplicationId</code> parameter for the <code>Publish</code> action.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn get_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attribute_name()
    }
    /// <p>The new value for the attribute.</p>
    pub fn attribute_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute_value(input.into());
        self
    }
    /// <p>The new value for the attribute.</p>
    pub fn set_attribute_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attribute_value(input);
        self
    }
    /// <p>The new value for the attribute.</p>
    pub fn get_attribute_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attribute_value()
    }
}
