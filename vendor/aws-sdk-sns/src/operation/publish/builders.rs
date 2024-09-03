// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::publish::_publish_output::PublishOutputBuilder;

pub use crate::operation::publish::_publish_input::PublishInputBuilder;

impl crate::operation::publish::builders::PublishInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::publish::PublishOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::publish::PublishError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.publish();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Publish`.
///
/// <p>Sends a message to an Amazon SNS topic, a text message (SMS message) directly to a phone number, or a message to a mobile platform endpoint (when you specify the <code>TargetArn</code>).</p>
/// <p>If you send a message to a topic, Amazon SNS delivers the message to each endpoint that is subscribed to the topic. The format of the message depends on the notification protocol for each subscribed endpoint.</p>
/// <p>When a <code>messageId</code> is returned, the message is saved and Amazon SNS immediately delivers it to subscribers.</p>
/// <p>To use the <code>Publish</code> action for publishing a message to a mobile endpoint, such as an app on a Kindle device or mobile phone, you must specify the EndpointArn for the TargetArn parameter. The EndpointArn is returned when making a call with the <code>CreatePlatformEndpoint</code> action.</p>
/// <p>For more information about formatting messages, see <a href="https://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-custommessage.html">Send Custom Platform-Specific Payloads in Messages to Mobile Devices</a>.</p><important>
/// <p>You can publish messages only to topics and endpoints in the same Amazon Web Services Region.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PublishFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::publish::builders::PublishInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::publish::PublishOutput, crate::operation::publish::PublishError>
    for PublishFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::publish::PublishOutput, crate::operation::publish::PublishError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PublishFluentBuilder {
    /// Creates a new `PublishFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Publish as a reference.
    pub fn as_input(&self) -> &crate::operation::publish::builders::PublishInputBuilder {
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
        crate::operation::publish::PublishOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::publish::PublishError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::publish::Publish::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::publish::Publish::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::publish::PublishOutput, crate::operation::publish::PublishError, Self>
    {
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
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_arn(input.into());
        self
    }
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_arn(input);
        self
    }
    /// <p>The topic you want to publish to.</p>
    /// <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_arn()
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_arn()
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number(input.into());
        self
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number(input);
        self
    }
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p>
    /// <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_phone_number()
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message(input.into());
        self
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message(input);
        self
    }
    /// <p>The message you want to send.</p>
    /// <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter.</p>
    /// <p></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p></li>
    /// <li>
    /// <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p>
    /// <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren't truncated mid-word but are cut off at whole-word boundaries.</p>
    /// <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p></li>
    /// </ul>
    /// <p>JSON-specific constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p></li>
    /// <li>
    /// <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p></li>
    /// <li>
    /// <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p></li>
    /// <li>
    /// <p>Values have a minimum length of 0 (the empty string, "", is allowed).</p></li>
    /// <li>
    /// <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p></li>
    /// <li>
    /// <p>Non-string values will cause the key to be ignored.</p></li>
    /// <li>
    /// <p>Keys that do not correspond to supported transport protocols are ignored.</p></li>
    /// <li>
    /// <p>Duplicate keys are not allowed.</p></li>
    /// <li>
    /// <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p></li>
    /// </ul>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message()
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be UTF-8 text with no line breaks or control characters, and less than 100 characters long.</p>
    pub fn subject(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subject(input.into());
        self
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be UTF-8 text with no line breaks or control characters, and less than 100 characters long.</p>
    pub fn set_subject(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subject(input);
        self
    }
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p>
    /// <p>Constraints: Subjects must be UTF-8 text with no line breaks or control characters, and less than 100 characters long.</p>
    pub fn get_subject(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subject()
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn message_structure(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_structure(input.into());
        self
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn set_message_structure(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_structure(input);
        self
    }
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must:</p>
    /// <ul>
    /// <li>
    /// <p>be a syntactically valid JSON object; and</p></li>
    /// <li>
    /// <p>contain at least a top-level JSON key of "default" with a value that is a string.</p></li>
    /// </ul>
    /// <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p>
    /// <p>Valid value: <code>json</code></p>
    pub fn get_message_structure(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_structure()
    }
    ///
    /// Adds a key-value pair to `MessageAttributes`.
    ///
    /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
    ///
    /// <p>Message attributes for Publish action.</p>
    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::MessageAttributeValue) -> Self {
        self.inner = self.inner.message_attributes(k.into(), v);
        self
    }
    /// <p>Message attributes for Publish action.</p>
    pub fn set_message_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    ) -> Self {
        self.inner = self.inner.set_message_attributes(input);
        self
    }
    /// <p>Message attributes for Publish action.</p>
    pub fn get_message_attributes(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        self.inner.get_message_attributes()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn message_deduplication_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_deduplication_id(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn set_message_deduplication_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_deduplication_id(input);
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageDeduplicationId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>, which is a token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any message sent with the same <code>MessageDeduplicationId</code> during the 5-minute deduplication interval is treated as a duplicate.</p>
    /// <p>If the topic has <code>ContentBasedDeduplication</code> set, the system generates a <code>MessageDeduplicationId</code> based on the contents of the message. Your <code>MessageDeduplicationId</code> overrides the generated one.</p>
    pub fn get_message_deduplication_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_deduplication_id()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn message_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_group_id(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn set_message_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_group_id(input);
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics. The <code>MessageGroupId</code> can contain up to 128 alphanumeric characters <code>(a-z, A-Z, 0-9)</code> and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>The <code>MessageGroupId</code> is a tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). Every message must include a <code>MessageGroupId</code>.</p>
    pub fn get_message_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_group_id()
    }
}
