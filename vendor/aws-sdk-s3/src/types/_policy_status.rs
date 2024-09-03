// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container element for a bucket's policy status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PolicyStatus {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub is_public: ::std::option::Option<bool>,
}
impl PolicyStatus {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub fn is_public(&self) -> ::std::option::Option<bool> {
        self.is_public
    }
}
impl PolicyStatus {
    /// Creates a new builder-style object to manufacture [`PolicyStatus`](crate::types::PolicyStatus).
    pub fn builder() -> crate::types::builders::PolicyStatusBuilder {
        crate::types::builders::PolicyStatusBuilder::default()
    }
}

/// A builder for [`PolicyStatus`](crate::types::PolicyStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PolicyStatusBuilder {
    pub(crate) is_public: ::std::option::Option<bool>,
}
impl PolicyStatusBuilder {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub fn is_public(mut self, input: bool) -> Self {
        self.is_public = ::std::option::Option::Some(input);
        self
    }
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub fn set_is_public(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_public = input;
        self
    }
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub fn get_is_public(&self) -> &::std::option::Option<bool> {
        &self.is_public
    }
    /// Consumes the builder and constructs a [`PolicyStatus`](crate::types::PolicyStatus).
    pub fn build(self) -> crate::types::PolicyStatus {
        crate::types::PolicyStatus { is_public: self.is_public }
    }
}
