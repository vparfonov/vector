// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Subscription {
    /// <p>The subscription's ARN.</p>
    pub subscription_arn: ::std::option::Option<::std::string::String>,
    /// <p>The subscription's owner.</p>
    pub owner: ::std::option::Option<::std::string::String>,
    /// <p>The subscription's protocol.</p>
    pub protocol: ::std::option::Option<::std::string::String>,
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the subscription's topic.</p>
    pub topic_arn: ::std::option::Option<::std::string::String>,
}
impl Subscription {
    /// <p>The subscription's ARN.</p>
    pub fn subscription_arn(&self) -> ::std::option::Option<&str> {
        self.subscription_arn.as_deref()
    }
    /// <p>The subscription's owner.</p>
    pub fn owner(&self) -> ::std::option::Option<&str> {
        self.owner.as_deref()
    }
    /// <p>The subscription's protocol.</p>
    pub fn protocol(&self) -> ::std::option::Option<&str> {
        self.protocol.as_deref()
    }
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The ARN of the subscription's topic.</p>
    pub fn topic_arn(&self) -> ::std::option::Option<&str> {
        self.topic_arn.as_deref()
    }
}
impl Subscription {
    /// Creates a new builder-style object to manufacture [`Subscription`](crate::types::Subscription).
    pub fn builder() -> crate::types::builders::SubscriptionBuilder {
        crate::types::builders::SubscriptionBuilder::default()
    }
}

/// A builder for [`Subscription`](crate::types::Subscription).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SubscriptionBuilder {
    pub(crate) subscription_arn: ::std::option::Option<::std::string::String>,
    pub(crate) owner: ::std::option::Option<::std::string::String>,
    pub(crate) protocol: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) topic_arn: ::std::option::Option<::std::string::String>,
}
impl SubscriptionBuilder {
    /// <p>The subscription's ARN.</p>
    pub fn subscription_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subscription_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subscription's ARN.</p>
    pub fn set_subscription_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subscription_arn = input;
        self
    }
    /// <p>The subscription's ARN.</p>
    pub fn get_subscription_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.subscription_arn
    }
    /// <p>The subscription's owner.</p>
    pub fn owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subscription's owner.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner = input;
        self
    }
    /// <p>The subscription's owner.</p>
    pub fn get_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner
    }
    /// <p>The subscription's protocol.</p>
    pub fn protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subscription's protocol.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The subscription's protocol.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<::std::string::String> {
        &self.protocol
    }
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub fn get_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        &self.endpoint
    }
    /// <p>The ARN of the subscription's topic.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.topic_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the subscription's topic.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.topic_arn = input;
        self
    }
    /// <p>The ARN of the subscription's topic.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.topic_arn
    }
    /// Consumes the builder and constructs a [`Subscription`](crate::types::Subscription).
    pub fn build(self) -> crate::types::Subscription {
        crate::types::Subscription {
            subscription_arn: self.subscription_arn,
            owner: self.owner,
            protocol: self.protocol,
            endpoint: self.endpoint,
            topic_arn: self.topic_arn,
        }
    }
}
