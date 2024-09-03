// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for request parameters to <code> <code>GetPackageVersionHistory</code> </code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPackageVersionHistoryInput {
    /// <p>Returns an audit history of versions of the package.</p>
    pub package_id: ::std::option::Option<::std::string::String>,
    /// <p>Limits results to a maximum number of versions.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl GetPackageVersionHistoryInput {
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn package_id(&self) -> ::std::option::Option<&str> {
        self.package_id.as_deref()
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl GetPackageVersionHistoryInput {
    /// Creates a new builder-style object to manufacture [`GetPackageVersionHistoryInput`](crate::operation::get_package_version_history::GetPackageVersionHistoryInput).
    pub fn builder() -> crate::operation::get_package_version_history::builders::GetPackageVersionHistoryInputBuilder {
        crate::operation::get_package_version_history::builders::GetPackageVersionHistoryInputBuilder::default()
    }
}

/// A builder for [`GetPackageVersionHistoryInput`](crate::operation::get_package_version_history::GetPackageVersionHistoryInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetPackageVersionHistoryInputBuilder {
    pub(crate) package_id: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl GetPackageVersionHistoryInputBuilder {
    /// <p>Returns an audit history of versions of the package.</p>
    /// This field is required.
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_id = input;
        self
    }
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn get_package_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.package_id
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Limits results to a maximum number of versions.</p>
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
    /// Consumes the builder and constructs a [`GetPackageVersionHistoryInput`](crate::operation::get_package_version_history::GetPackageVersionHistoryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_package_version_history::GetPackageVersionHistoryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_package_version_history::GetPackageVersionHistoryInput {
            package_id: self.package_id,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
