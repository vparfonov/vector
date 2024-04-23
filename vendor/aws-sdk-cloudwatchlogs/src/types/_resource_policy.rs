// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A policy enabling one or more entities to put logs to a log group in this account.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourcePolicy {
    /// <p>The name of the resource policy.</p>
    pub policy_name: ::std::option::Option<::std::string::String>,
    /// <p>The details of the policy.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub last_updated_time: ::std::option::Option<i64>,
}
impl ResourcePolicy {
    /// <p>The name of the resource policy.</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>The details of the policy.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<i64> {
        self.last_updated_time
    }
}
impl ResourcePolicy {
    /// Creates a new builder-style object to manufacture [`ResourcePolicy`](crate::types::ResourcePolicy).
    pub fn builder() -> crate::types::builders::ResourcePolicyBuilder {
        crate::types::builders::ResourcePolicyBuilder::default()
    }
}

/// A builder for [`ResourcePolicy`](crate::types::ResourcePolicy).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResourcePolicyBuilder {
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) last_updated_time: ::std::option::Option<i64>,
}
impl ResourcePolicyBuilder {
    /// <p>The name of the resource policy.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the resource policy.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>The name of the resource policy.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_name
    }
    /// <p>The details of the policy.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The details of the policy.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>The details of the policy.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn last_updated_time(mut self, input: i64) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn set_last_updated_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn get_last_updated_time(&self) -> &::std::option::Option<i64> {
        &self.last_updated_time
    }
    /// Consumes the builder and constructs a [`ResourcePolicy`](crate::types::ResourcePolicy).
    pub fn build(self) -> crate::types::ResourcePolicy {
        crate::types::ResourcePolicy {
            policy_name: self.policy_name,
            policy_document: self.policy_document,
            last_updated_time: self.last_updated_time,
        }
    }
}
