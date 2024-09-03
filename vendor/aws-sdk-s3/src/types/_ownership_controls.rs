// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container element for a bucket's ownership controls.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OwnershipControls {
    /// <p>The container element for an ownership control rule.</p>
    pub rules: ::std::vec::Vec<crate::types::OwnershipControlsRule>,
}
impl OwnershipControls {
    /// <p>The container element for an ownership control rule.</p>
    pub fn rules(&self) -> &[crate::types::OwnershipControlsRule] {
        use std::ops::Deref;
        self.rules.deref()
    }
}
impl OwnershipControls {
    /// Creates a new builder-style object to manufacture [`OwnershipControls`](crate::types::OwnershipControls).
    pub fn builder() -> crate::types::builders::OwnershipControlsBuilder {
        crate::types::builders::OwnershipControlsBuilder::default()
    }
}

/// A builder for [`OwnershipControls`](crate::types::OwnershipControls).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OwnershipControlsBuilder {
    pub(crate) rules: ::std::option::Option<::std::vec::Vec<crate::types::OwnershipControlsRule>>,
}
impl OwnershipControlsBuilder {
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>The container element for an ownership control rule.</p>
    pub fn rules(mut self, input: crate::types::OwnershipControlsRule) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>The container element for an ownership control rule.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::OwnershipControlsRule>>) -> Self {
        self.rules = input;
        self
    }
    /// <p>The container element for an ownership control rule.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::OwnershipControlsRule>> {
        &self.rules
    }
    /// Consumes the builder and constructs a [`OwnershipControls`](crate::types::OwnershipControls).
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](crate::types::builders::OwnershipControlsBuilder::rules)
    pub fn build(self) -> ::std::result::Result<crate::types::OwnershipControls, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::OwnershipControls {
            rules: self.rules.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "rules",
                    "rules was not specified but it is required when building OwnershipControls",
                )
            })?,
        })
    }
}
