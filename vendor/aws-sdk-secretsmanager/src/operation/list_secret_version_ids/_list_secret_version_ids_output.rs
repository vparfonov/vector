// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSecretVersionIdsOutput {
    /// <p>A list of the versions of the secret.</p>
    pub versions: ::std::option::Option<::std::vec::Vec<crate::types::SecretVersionsListEntry>>,
    /// <p>Secrets Manager includes this value if there's more output available than what is included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a long list. To get the next results, call <code>ListSecretVersionIds</code> again with this value.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the secret.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the secret.</p>
    pub name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSecretVersionIdsOutput {
    /// <p>A list of the versions of the secret.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.versions.is_none()`.
    pub fn versions(&self) -> &[crate::types::SecretVersionsListEntry] {
        self.versions.as_deref().unwrap_or_default()
    }
    /// <p>Secrets Manager includes this value if there's more output available than what is included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a long list. To get the next results, call <code>ListSecretVersionIds</code> again with this value.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The ARN of the secret.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the secret.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListSecretVersionIdsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSecretVersionIdsOutput {
    /// Creates a new builder-style object to manufacture [`ListSecretVersionIdsOutput`](crate::operation::list_secret_version_ids::ListSecretVersionIdsOutput).
    pub fn builder() -> crate::operation::list_secret_version_ids::builders::ListSecretVersionIdsOutputBuilder {
        crate::operation::list_secret_version_ids::builders::ListSecretVersionIdsOutputBuilder::default()
    }
}

/// A builder for [`ListSecretVersionIdsOutput`](crate::operation::list_secret_version_ids::ListSecretVersionIdsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListSecretVersionIdsOutputBuilder {
    pub(crate) versions: ::std::option::Option<::std::vec::Vec<crate::types::SecretVersionsListEntry>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSecretVersionIdsOutputBuilder {
    /// Appends an item to `versions`.
    ///
    /// To override the contents of this collection use [`set_versions`](Self::set_versions).
    ///
    /// <p>A list of the versions of the secret.</p>
    pub fn versions(mut self, input: crate::types::SecretVersionsListEntry) -> Self {
        let mut v = self.versions.unwrap_or_default();
        v.push(input);
        self.versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the versions of the secret.</p>
    pub fn set_versions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SecretVersionsListEntry>>) -> Self {
        self.versions = input;
        self
    }
    /// <p>A list of the versions of the secret.</p>
    pub fn get_versions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SecretVersionsListEntry>> {
        &self.versions
    }
    /// <p>Secrets Manager includes this value if there's more output available than what is included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a long list. To get the next results, call <code>ListSecretVersionIds</code> again with this value.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Secrets Manager includes this value if there's more output available than what is included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a long list. To get the next results, call <code>ListSecretVersionIds</code> again with this value.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Secrets Manager includes this value if there's more output available than what is included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a long list. To get the next results, call <code>ListSecretVersionIds</code> again with this value.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The ARN of the secret.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name of the secret.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the secret.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the secret.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListSecretVersionIdsOutput`](crate::operation::list_secret_version_ids::ListSecretVersionIdsOutput).
    pub fn build(self) -> crate::operation::list_secret_version_ids::ListSecretVersionIdsOutput {
        crate::operation::list_secret_version_ids::ListSecretVersionIdsOutput {
            versions: self.versions,
            next_token: self.next_token,
            arn: self.arn,
            name: self.name,
            _request_id: self._request_id,
        }
    }
}
