// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a subscription filter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubscriptionFilter {
    /// <p>The name of the subscription filter.</p>
    pub filter_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the log group.</p>
    pub log_group_name: ::std::option::Option<::std::string::String>,
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub filter_pattern: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub destination_arn: ::std::option::Option<::std::string::String>,
    /// <p></p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The method used to distribute log data to the destination, which can be either random or grouped by log stream.</p>
    pub distribution: ::std::option::Option<crate::types::Distribution>,
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub creation_time: ::std::option::Option<i64>,
}
impl SubscriptionFilter {
    /// <p>The name of the subscription filter.</p>
    pub fn filter_name(&self) -> ::std::option::Option<&str> {
        self.filter_name.as_deref()
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(&self) -> ::std::option::Option<&str> {
        self.log_group_name.as_deref()
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn filter_pattern(&self) -> ::std::option::Option<&str> {
        self.filter_pattern.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn destination_arn(&self) -> ::std::option::Option<&str> {
        self.destination_arn.as_deref()
    }
    /// <p></p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The method used to distribute log data to the destination, which can be either random or grouped by log stream.</p>
    pub fn distribution(&self) -> ::std::option::Option<&crate::types::Distribution> {
        self.distribution.as_ref()
    }
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn creation_time(&self) -> ::std::option::Option<i64> {
        self.creation_time
    }
}
impl SubscriptionFilter {
    /// Creates a new builder-style object to manufacture [`SubscriptionFilter`](crate::types::SubscriptionFilter).
    pub fn builder() -> crate::types::builders::SubscriptionFilterBuilder {
        crate::types::builders::SubscriptionFilterBuilder::default()
    }
}

/// A builder for [`SubscriptionFilter`](crate::types::SubscriptionFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SubscriptionFilterBuilder {
    pub(crate) filter_name: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) filter_pattern: ::std::option::Option<::std::string::String>,
    pub(crate) destination_arn: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) distribution: ::std::option::Option<crate::types::Distribution>,
    pub(crate) creation_time: ::std::option::Option<i64>,
}
impl SubscriptionFilterBuilder {
    /// <p>The name of the subscription filter.</p>
    pub fn filter_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filter_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the subscription filter.</p>
    pub fn set_filter_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filter_name = input;
        self
    }
    /// <p>The name of the subscription filter.</p>
    pub fn get_filter_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.filter_name
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_name = input;
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_name
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn filter_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filter_pattern = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn set_filter_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filter_pattern = input;
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn get_filter_pattern(&self) -> &::std::option::Option<::std::string::String> {
        &self.filter_pattern
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn destination_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn set_destination_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn get_destination_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_arn
    }
    /// <p></p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p></p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p></p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>The method used to distribute log data to the destination, which can be either random or grouped by log stream.</p>
    pub fn distribution(mut self, input: crate::types::Distribution) -> Self {
        self.distribution = ::std::option::Option::Some(input);
        self
    }
    /// <p>The method used to distribute log data to the destination, which can be either random or grouped by log stream.</p>
    pub fn set_distribution(mut self, input: ::std::option::Option<crate::types::Distribution>) -> Self {
        self.distribution = input;
        self
    }
    /// <p>The method used to distribute log data to the destination, which can be either random or grouped by log stream.</p>
    pub fn get_distribution(&self) -> &::std::option::Option<crate::types::Distribution> {
        &self.distribution
    }
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn creation_time(mut self, input: i64) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<i64> {
        &self.creation_time
    }
    /// Consumes the builder and constructs a [`SubscriptionFilter`](crate::types::SubscriptionFilter).
    pub fn build(self) -> crate::types::SubscriptionFilter {
        crate::types::SubscriptionFilter {
            filter_name: self.filter_name,
            log_group_name: self.log_group_name,
            filter_pattern: self.filter_pattern,
            destination_arn: self.destination_arn,
            role_arn: self.role_arn,
            distribution: self.distribution,
            creation_time: self.creation_time,
        }
    }
}
