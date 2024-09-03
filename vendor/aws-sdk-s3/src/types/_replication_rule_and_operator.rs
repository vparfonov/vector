// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for specifying rule filters. The filters determine the subset of objects to which the rule applies. This element is required only if you specify more than one filter.</p>
/// <p>For example:</p>
/// <ul>
/// <li>
/// <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> tag.</p></li>
/// <li>
/// <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> tag.</p></li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p>
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>An array of tags containing key and value pairs.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl ReplicationRuleAndOperator {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>An array of tags containing key and value pairs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl ReplicationRuleAndOperator {
    /// Creates a new builder-style object to manufacture [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
    pub fn builder() -> crate::types::builders::ReplicationRuleAndOperatorBuilder {
        crate::types::builders::ReplicationRuleAndOperatorBuilder::default()
    }
}

/// A builder for [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ReplicationRuleAndOperatorBuilder {
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl ReplicationRuleAndOperatorBuilder {
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>An object key name prefix that identifies the subset of objects to which the rule applies.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of tags containing key and value pairs.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of tags containing key and value pairs.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>An array of tags containing key and value pairs.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`ReplicationRuleAndOperator`](crate::types::ReplicationRuleAndOperator).
    pub fn build(self) -> crate::types::ReplicationRuleAndOperator {
        crate::types::ReplicationRuleAndOperator {
            prefix: self.prefix,
            tags: self.tags,
        }
    }
}
