// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for SetSubscriptionAttributes action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetSubscriptionAttributesInput {
    /// <p>The ARN of the subscription to modify.</p>
    pub subscription_arn: ::std::option::Option<::std::string::String>,
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that this action uses:</p>
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
    pub attribute_name: ::std::option::Option<::std::string::String>,
    /// <p>The new value for the attribute in JSON format.</p>
    pub attribute_value: ::std::option::Option<::std::string::String>,
}
impl SetSubscriptionAttributesInput {
    /// <p>The ARN of the subscription to modify.</p>
    pub fn subscription_arn(&self) -> ::std::option::Option<&str> {
        self.subscription_arn.as_deref()
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that this action uses:</p>
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
    pub fn attribute_name(&self) -> ::std::option::Option<&str> {
        self.attribute_name.as_deref()
    }
    /// <p>The new value for the attribute in JSON format.</p>
    pub fn attribute_value(&self) -> ::std::option::Option<&str> {
        self.attribute_value.as_deref()
    }
}
impl SetSubscriptionAttributesInput {
    /// Creates a new builder-style object to manufacture [`SetSubscriptionAttributesInput`](crate::operation::set_subscription_attributes::SetSubscriptionAttributesInput).
    pub fn builder() -> crate::operation::set_subscription_attributes::builders::SetSubscriptionAttributesInputBuilder {
        crate::operation::set_subscription_attributes::builders::SetSubscriptionAttributesInputBuilder::default()
    }
}

/// A builder for [`SetSubscriptionAttributesInput`](crate::operation::set_subscription_attributes::SetSubscriptionAttributesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SetSubscriptionAttributesInputBuilder {
    pub(crate) subscription_arn: ::std::option::Option<::std::string::String>,
    pub(crate) attribute_name: ::std::option::Option<::std::string::String>,
    pub(crate) attribute_value: ::std::option::Option<::std::string::String>,
}
impl SetSubscriptionAttributesInputBuilder {
    /// <p>The ARN of the subscription to modify.</p>
    /// This field is required.
    pub fn subscription_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subscription_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the subscription to modify.</p>
    pub fn set_subscription_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subscription_arn = input;
        self
    }
    /// <p>The ARN of the subscription to modify.</p>
    pub fn get_subscription_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.subscription_arn
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that this action uses:</p>
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
    /// This field is required.
    pub fn attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attribute_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that this action uses:</p>
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
    pub fn set_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attribute_name = input;
        self
    }
    /// <p>A map of attributes with their corresponding values.</p>
    /// <p>The following lists the names, descriptions, and values of the special request parameters that this action uses:</p>
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
    pub fn get_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.attribute_name
    }
    /// <p>The new value for the attribute in JSON format.</p>
    pub fn attribute_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attribute_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new value for the attribute in JSON format.</p>
    pub fn set_attribute_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attribute_value = input;
        self
    }
    /// <p>The new value for the attribute in JSON format.</p>
    pub fn get_attribute_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.attribute_value
    }
    /// Consumes the builder and constructs a [`SetSubscriptionAttributesInput`](crate::operation::set_subscription_attributes::SetSubscriptionAttributesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_subscription_attributes::SetSubscriptionAttributesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::set_subscription_attributes::SetSubscriptionAttributesInput {
            subscription_arn: self.subscription_arn,
            attribute_name: self.attribute_name,
            attribute_value: self.attribute_value,
        })
    }
}
