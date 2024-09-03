// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::subscribe::_subscribe_output::SubscribeOutputBuilder;

pub use crate::operation::subscribe::_subscribe_input::SubscribeInputBuilder;

impl crate::operation::subscribe::builders::SubscribeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::subscribe::SubscribeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::subscribe::SubscribeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.subscribe();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Subscribe`.
///
/// <p>Subscribes an endpoint to an Amazon SNS topic. If the endpoint type is HTTP/S or email, or if the endpoint and the topic are not in the same Amazon Web Services account, the endpoint owner must run the <code>ConfirmSubscription</code> action to confirm the subscription.</p>
/// <p>You call the <code>ConfirmSubscription</code> action with the token from the subscription response. Confirmation tokens are valid for two days.</p>
/// <p>This action is throttled at 100 transactions per second (TPS).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SubscribeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::subscribe::builders::SubscribeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::subscribe::SubscribeOutput, crate::operation::subscribe::SubscribeError>
    for SubscribeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::subscribe::SubscribeOutput, crate::operation::subscribe::SubscribeError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SubscribeFluentBuilder {
    /// Creates a new `SubscribeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Subscribe as a reference.
    pub fn as_input(&self) -> &crate::operation::subscribe::builders::SubscribeInputBuilder {
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
        crate::operation::subscribe::SubscribeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::subscribe::SubscribeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::subscribe::Subscribe::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::subscribe::Subscribe::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::subscribe::SubscribeOutput,
        crate::operation::subscribe::SubscribeError,
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
    /// <p>The ARN of the topic you want to subscribe to.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_arn(input.into());
        self
    }
    /// <p>The ARN of the topic you want to subscribe to.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_arn(input);
        self
    }
    /// <p>The ARN of the topic you want to subscribe to.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_arn()
    }
    /// <p>The protocol that you want to use. Supported protocols include:</p>
    /// <ul>
    /// <li>
    /// <p><code>http</code> – delivery of JSON-encoded message via HTTP POST</p></li>
    /// <li>
    /// <p><code>https</code> – delivery of JSON-encoded message via HTTPS POST</p></li>
    /// <li>
    /// <p><code>email</code> – delivery of message via SMTP</p></li>
    /// <li>
    /// <p><code>email-json</code> – delivery of JSON-encoded message via SMTP</p></li>
    /// <li>
    /// <p><code>sms</code> – delivery of message via SMS</p></li>
    /// <li>
    /// <p><code>sqs</code> – delivery of JSON-encoded message to an Amazon SQS queue</p></li>
    /// <li>
    /// <p><code>application</code> – delivery of JSON-encoded message to an EndpointArn for a mobile app and device</p></li>
    /// <li>
    /// <p><code>lambda</code> – delivery of JSON-encoded message to an Lambda function</p></li>
    /// <li>
    /// <p><code>firehose</code> – delivery of JSON-encoded message to an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.protocol(input.into());
        self
    }
    /// <p>The protocol that you want to use. Supported protocols include:</p>
    /// <ul>
    /// <li>
    /// <p><code>http</code> – delivery of JSON-encoded message via HTTP POST</p></li>
    /// <li>
    /// <p><code>https</code> – delivery of JSON-encoded message via HTTPS POST</p></li>
    /// <li>
    /// <p><code>email</code> – delivery of message via SMTP</p></li>
    /// <li>
    /// <p><code>email-json</code> – delivery of JSON-encoded message via SMTP</p></li>
    /// <li>
    /// <p><code>sms</code> – delivery of message via SMS</p></li>
    /// <li>
    /// <p><code>sqs</code> – delivery of JSON-encoded message to an Amazon SQS queue</p></li>
    /// <li>
    /// <p><code>application</code> – delivery of JSON-encoded message to an EndpointArn for a mobile app and device</p></li>
    /// <li>
    /// <p><code>lambda</code> – delivery of JSON-encoded message to an Lambda function</p></li>
    /// <li>
    /// <p><code>firehose</code> – delivery of JSON-encoded message to an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn set_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The protocol that you want to use. Supported protocols include:</p>
    /// <ul>
    /// <li>
    /// <p><code>http</code> – delivery of JSON-encoded message via HTTP POST</p></li>
    /// <li>
    /// <p><code>https</code> – delivery of JSON-encoded message via HTTPS POST</p></li>
    /// <li>
    /// <p><code>email</code> – delivery of message via SMTP</p></li>
    /// <li>
    /// <p><code>email-json</code> – delivery of JSON-encoded message via SMTP</p></li>
    /// <li>
    /// <p><code>sms</code> – delivery of message via SMS</p></li>
    /// <li>
    /// <p><code>sqs</code> – delivery of JSON-encoded message to an Amazon SQS queue</p></li>
    /// <li>
    /// <p><code>application</code> – delivery of JSON-encoded message to an EndpointArn for a mobile app and device</p></li>
    /// <li>
    /// <p><code>lambda</code> – delivery of JSON-encoded message to an Lambda function</p></li>
    /// <li>
    /// <p><code>firehose</code> – delivery of JSON-encoded message to an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn get_protocol(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_protocol()
    }
    /// <p>The endpoint that you want to receive notifications. Endpoints vary by protocol:</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>http</code> protocol, the (public) endpoint is a URL beginning with <code>http://</code>.</p></li>
    /// <li>
    /// <p>For the <code>https</code> protocol, the (public) endpoint is a URL beginning with <code>https://</code>.</p></li>
    /// <li>
    /// <p>For the <code>email</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>email-json</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>sms</code> protocol, the endpoint is a phone number of an SMS-enabled device.</p></li>
    /// <li>
    /// <p>For the <code>sqs</code> protocol, the endpoint is the ARN of an Amazon SQS queue.</p></li>
    /// <li>
    /// <p>For the <code>application</code> protocol, the endpoint is the EndpointArn of a mobile app and device.</p></li>
    /// <li>
    /// <p>For the <code>lambda</code> protocol, the endpoint is the ARN of an Lambda function.</p></li>
    /// <li>
    /// <p>For the <code>firehose</code> protocol, the endpoint is the ARN of an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.endpoint(input.into());
        self
    }
    /// <p>The endpoint that you want to receive notifications. Endpoints vary by protocol:</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>http</code> protocol, the (public) endpoint is a URL beginning with <code>http://</code>.</p></li>
    /// <li>
    /// <p>For the <code>https</code> protocol, the (public) endpoint is a URL beginning with <code>https://</code>.</p></li>
    /// <li>
    /// <p>For the <code>email</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>email-json</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>sms</code> protocol, the endpoint is a phone number of an SMS-enabled device.</p></li>
    /// <li>
    /// <p>For the <code>sqs</code> protocol, the endpoint is the ARN of an Amazon SQS queue.</p></li>
    /// <li>
    /// <p>For the <code>application</code> protocol, the endpoint is the EndpointArn of a mobile app and device.</p></li>
    /// <li>
    /// <p>For the <code>lambda</code> protocol, the endpoint is the ARN of an Lambda function.</p></li>
    /// <li>
    /// <p>For the <code>firehose</code> protocol, the endpoint is the ARN of an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint(input);
        self
    }
    /// <p>The endpoint that you want to receive notifications. Endpoints vary by protocol:</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>http</code> protocol, the (public) endpoint is a URL beginning with <code>http://</code>.</p></li>
    /// <li>
    /// <p>For the <code>https</code> protocol, the (public) endpoint is a URL beginning with <code>https://</code>.</p></li>
    /// <li>
    /// <p>For the <code>email</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>email-json</code> protocol, the endpoint is an email address.</p></li>
    /// <li>
    /// <p>For the <code>sms</code> protocol, the endpoint is a phone number of an SMS-enabled device.</p></li>
    /// <li>
    /// <p>For the <code>sqs</code> protocol, the endpoint is the ARN of an Amazon SQS queue.</p></li>
    /// <li>
    /// <p>For the <code>application</code> protocol, the endpoint is the EndpointArn of a mobile app and device.</p></li>
    /// <li>
    /// <p>For the <code>lambda</code> protocol, the endpoint is the ARN of an Lambda function.</p></li>
    /// <li>
    /// <p>For the <code>firehose</code> protocol, the endpoint is the ARN of an Amazon Kinesis Data Firehose delivery stream.</p></li>
    /// </ul>
    pub fn get_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_endpoint()
    }
    ///
    /// Adds a key-value pair to `Attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>Subscribe</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>FilterPolicy</code> – The simple JSON object that lets your subscriber receive only a subset of messages, rather than receiving every message published to the topic.</p></li>
    /// <li>
    /// <p><code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li>
    /// <p><code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p></li>
    /// <li>
    /// <p><code>MessageBody</code> – The filter is applied on the message body.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>RawMessageDelivery</code> – When set to <code>true</code>, enables raw message delivery to Amazon SQS or HTTP/S endpoints. This eliminates the need for the endpoints to process JSON formatting, which is otherwise created for Amazon SNS metadata.</p></li>
    /// <li>
    /// <p><code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li>
    /// <p><code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li>
    /// <p>Permission to write to the Firehose delivery stream</p></li>
    /// <li>
    /// <p>Amazon SNS listed as a trusted entity</p></li>
    /// </ul>
    /// <p>Specifying a valid ARN for this attribute is required for Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p></li>
    /// </ul>
    /// <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ReplayPolicy</code> – Adds or updates an inline policy document for a subscription to replay messages stored in the specified Amazon SNS topic.</p></li>
    /// <li>
    /// <p><code>ReplayStatus</code> – Retrieves the status of the subscription message replay, which can be one of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>Completed</code> – The replay has successfully redelivered all messages, and is now delivering newly published messages. If an ending point was specified in the <code>ReplayPolicy</code> then the subscription will no longer receive newly published messages.</p></li>
    /// <li>
    /// <p><code>In progress</code> – The replay is currently replaying the selected messages.</p></li>
    /// <li>
    /// <p><code>Failed</code> – The replay was unable to complete.</p></li>
    /// <li>
    /// <p><code>Pending</code> – The default state while the replay initiates.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attributes(k.into(), v.into());
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>Subscribe</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>FilterPolicy</code> – The simple JSON object that lets your subscriber receive only a subset of messages, rather than receiving every message published to the topic.</p></li>
    /// <li>
    /// <p><code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li>
    /// <p><code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p></li>
    /// <li>
    /// <p><code>MessageBody</code> – The filter is applied on the message body.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>RawMessageDelivery</code> – When set to <code>true</code>, enables raw message delivery to Amazon SQS or HTTP/S endpoints. This eliminates the need for the endpoints to process JSON formatting, which is otherwise created for Amazon SNS metadata.</p></li>
    /// <li>
    /// <p><code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li>
    /// <p><code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li>
    /// <p>Permission to write to the Firehose delivery stream</p></li>
    /// <li>
    /// <p>Amazon SNS listed as a trusted entity</p></li>
    /// </ul>
    /// <p>Specifying a valid ARN for this attribute is required for Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p></li>
    /// </ul>
    /// <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ReplayPolicy</code> – Adds or updates an inline policy document for a subscription to replay messages stored in the specified Amazon SNS topic.</p></li>
    /// <li>
    /// <p><code>ReplayStatus</code> – Retrieves the status of the subscription message replay, which can be one of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>Completed</code> – The replay has successfully redelivered all messages, and is now delivering newly published messages. If an ending point was specified in the <code>ReplayPolicy</code> then the subscription will no longer receive newly published messages.</p></li>
    /// <li>
    /// <p><code>In progress</code> – The replay is currently replaying the selected messages.</p></li>
    /// <li>
    /// <p><code>Failed</code> – The replay was unable to complete.</p></li>
    /// <li>
    /// <p><code>Pending</code> – The default state while the replay initiates.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that the <code>Subscribe</code> action uses:</p>
    /// <ul>
    /// <li>
    /// <p><code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p></li>
    /// <li>
    /// <p><code>FilterPolicy</code> – The simple JSON object that lets your subscriber receive only a subset of messages, rather than receiving every message published to the topic.</p></li>
    /// <li>
    /// <p><code>FilterPolicyScope</code> – This attribute lets you choose the filtering scope by using one of the following string value types:</p>
    /// <ul>
    /// <li>
    /// <p><code>MessageAttributes</code> (default) – The filter is applied on the message attributes.</p></li>
    /// <li>
    /// <p><code>MessageBody</code> – The filter is applied on the message body.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><code>RawMessageDelivery</code> – When set to <code>true</code>, enables raw message delivery to Amazon SQS or HTTP/S endpoints. This eliminates the need for the endpoints to process JSON formatting, which is otherwise created for Amazon SNS metadata.</p></li>
    /// <li>
    /// <p><code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p></li>
    /// </ul>
    /// <p>The following attribute applies only to Amazon Data Firehose delivery stream subscriptions:</p>
    /// <ul>
    /// <li>
    /// <p><code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
    /// <ul>
    /// <li>
    /// <p>Permission to write to the Firehose delivery stream</p></li>
    /// <li>
    /// <p>Amazon SNS listed as a trusted entity</p></li>
    /// </ul>
    /// <p>Specifying a valid ARN for this attribute is required for Firehose delivery stream subscriptions. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout to Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p></li>
    /// </ul>
    /// <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
    /// <ul>
    /// <li>
    /// <p><code>ReplayPolicy</code> – Adds or updates an inline policy document for a subscription to replay messages stored in the specified Amazon SNS topic.</p></li>
    /// <li>
    /// <p><code>ReplayStatus</code> – Retrieves the status of the subscription message replay, which can be one of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>Completed</code> – The replay has successfully redelivered all messages, and is now delivering newly published messages. If an ending point was specified in the <code>ReplayPolicy</code> then the subscription will no longer receive newly published messages.</p></li>
    /// <li>
    /// <p><code>In progress</code> – The replay is currently replaying the selected messages.</p></li>
    /// <li>
    /// <p><code>Failed</code> – The replay was unable to complete.</p></li>
    /// <li>
    /// <p><code>Pending</code> – The default state while the replay initiates.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_attributes()
    }
    /// <p>Sets whether the response from the <code>Subscribe</code> request includes the subscription ARN, even if the subscription is not yet confirmed.</p>
    /// <p>If you set this parameter to <code>true</code>, the response includes the ARN in all cases, even if the subscription is not yet confirmed. In addition to the ARN for confirmed subscriptions, the response also includes the <code>pending subscription</code> ARN value for subscriptions that aren't yet confirmed. A subscription becomes confirmed when the subscriber calls the <code>ConfirmSubscription</code> action with a confirmation token.</p>
    /// <p></p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn return_subscription_arn(mut self, input: bool) -> Self {
        self.inner = self.inner.return_subscription_arn(input);
        self
    }
    /// <p>Sets whether the response from the <code>Subscribe</code> request includes the subscription ARN, even if the subscription is not yet confirmed.</p>
    /// <p>If you set this parameter to <code>true</code>, the response includes the ARN in all cases, even if the subscription is not yet confirmed. In addition to the ARN for confirmed subscriptions, the response also includes the <code>pending subscription</code> ARN value for subscriptions that aren't yet confirmed. A subscription becomes confirmed when the subscriber calls the <code>ConfirmSubscription</code> action with a confirmation token.</p>
    /// <p></p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn set_return_subscription_arn(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_return_subscription_arn(input);
        self
    }
    /// <p>Sets whether the response from the <code>Subscribe</code> request includes the subscription ARN, even if the subscription is not yet confirmed.</p>
    /// <p>If you set this parameter to <code>true</code>, the response includes the ARN in all cases, even if the subscription is not yet confirmed. In addition to the ARN for confirmed subscriptions, the response also includes the <code>pending subscription</code> ARN value for subscriptions that aren't yet confirmed. A subscription becomes confirmed when the subscriber calls the <code>ConfirmSubscription</code> action with a confirmation token.</p>
    /// <p></p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn get_return_subscription_arn(&self) -> &::std::option::Option<bool> {
        self.inner.get_return_subscription_arn()
    }
}
