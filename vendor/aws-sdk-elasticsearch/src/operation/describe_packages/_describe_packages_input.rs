// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for request parameters to <code> <code>DescribePackage</code> </code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePackagesInput {
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::DescribePackagesFilter>>,
    /// <p>Limits results to a maximum number of packages.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribePackagesInput {
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::DescribePackagesFilter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>Limits results to a maximum number of packages.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribePackagesInput {
    /// Creates a new builder-style object to manufacture [`DescribePackagesInput`](crate::operation::describe_packages::DescribePackagesInput).
    pub fn builder() -> crate::operation::describe_packages::builders::DescribePackagesInputBuilder {
        crate::operation::describe_packages::builders::DescribePackagesInputBuilder::default()
    }
}

/// A builder for [`DescribePackagesInput`](crate::operation::describe_packages::DescribePackagesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribePackagesInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::DescribePackagesFilter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribePackagesInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    pub fn filters(mut self, input: crate::types::DescribePackagesFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DescribePackagesFilter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DescribePackagesFilter>> {
        &self.filters
    }
    /// <p>Limits results to a maximum number of packages.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Limits results to a maximum number of packages.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Limits results to a maximum number of packages.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`DescribePackagesInput`](crate::operation::describe_packages::DescribePackagesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_packages::DescribePackagesInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_packages::DescribePackagesInput {
            filters: self.filters,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
