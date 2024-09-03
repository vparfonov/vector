// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicateSecretToRegionsOutput {
    /// <p>The ARN of the primary secret.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The status of replication.</p>
    pub replication_status: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>,
    _request_id: Option<String>,
}
impl ReplicateSecretToRegionsOutput {
    /// <p>The ARN of the primary secret.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The status of replication.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.replication_status.is_none()`.
    pub fn replication_status(&self) -> &[crate::types::ReplicationStatusType] {
        self.replication_status.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ReplicateSecretToRegionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ReplicateSecretToRegionsOutput {
    /// Creates a new builder-style object to manufacture [`ReplicateSecretToRegionsOutput`](crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput).
    pub fn builder() -> crate::operation::replicate_secret_to_regions::builders::ReplicateSecretToRegionsOutputBuilder {
        crate::operation::replicate_secret_to_regions::builders::ReplicateSecretToRegionsOutputBuilder::default()
    }
}

/// A builder for [`ReplicateSecretToRegionsOutput`](crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ReplicateSecretToRegionsOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) replication_status: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>,
    _request_id: Option<String>,
}
impl ReplicateSecretToRegionsOutputBuilder {
    /// <p>The ARN of the primary secret.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the primary secret.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the primary secret.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Appends an item to `replication_status`.
    ///
    /// To override the contents of this collection use [`set_replication_status`](Self::set_replication_status).
    ///
    /// <p>The status of replication.</p>
    pub fn replication_status(mut self, input: crate::types::ReplicationStatusType) -> Self {
        let mut v = self.replication_status.unwrap_or_default();
        v.push(input);
        self.replication_status = ::std::option::Option::Some(v);
        self
    }
    /// <p>The status of replication.</p>
    pub fn set_replication_status(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>>) -> Self {
        self.replication_status = input;
        self
    }
    /// <p>The status of replication.</p>
    pub fn get_replication_status(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReplicationStatusType>> {
        &self.replication_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ReplicateSecretToRegionsOutput`](crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput).
    pub fn build(self) -> crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput {
        crate::operation::replicate_secret_to_regions::ReplicateSecretToRegionsOutput {
            arn: self.arn,
            replication_status: self.replication_status,
            _request_id: self._request_id,
        }
    }
}
