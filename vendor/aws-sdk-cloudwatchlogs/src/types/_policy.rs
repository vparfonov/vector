// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that contains information about one delivery destination policy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Policy {
    /// <p>The contents of the delivery destination policy.</p>
    pub delivery_destination_policy: ::std::option::Option<::std::string::String>,
}
impl Policy {
    /// <p>The contents of the delivery destination policy.</p>
    pub fn delivery_destination_policy(&self) -> ::std::option::Option<&str> {
        self.delivery_destination_policy.as_deref()
    }
}
impl Policy {
    /// Creates a new builder-style object to manufacture [`Policy`](crate::types::Policy).
    pub fn builder() -> crate::types::builders::PolicyBuilder {
        crate::types::builders::PolicyBuilder::default()
    }
}

/// A builder for [`Policy`](crate::types::Policy).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PolicyBuilder {
    pub(crate) delivery_destination_policy: ::std::option::Option<::std::string::String>,
}
impl PolicyBuilder {
    /// <p>The contents of the delivery destination policy.</p>
    pub fn delivery_destination_policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.delivery_destination_policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The contents of the delivery destination policy.</p>
    pub fn set_delivery_destination_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.delivery_destination_policy = input;
        self
    }
    /// <p>The contents of the delivery destination policy.</p>
    pub fn get_delivery_destination_policy(&self) -> &::std::option::Option<::std::string::String> {
        &self.delivery_destination_policy
    }
    /// Consumes the builder and constructs a [`Policy`](crate::types::Policy).
    pub fn build(self) -> crate::types::Policy {
        crate::types::Policy {
            delivery_destination_policy: self.delivery_destination_policy,
        }
    }
}
