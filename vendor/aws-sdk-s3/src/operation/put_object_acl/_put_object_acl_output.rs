// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutObjectAclOutput {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutObjectAclOutput {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_charged(&self) -> ::std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for PutObjectAclOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for PutObjectAclOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutObjectAclOutput {
    /// Creates a new builder-style object to manufacture [`PutObjectAclOutput`](crate::operation::put_object_acl::PutObjectAclOutput).
    pub fn builder() -> crate::operation::put_object_acl::builders::PutObjectAclOutputBuilder {
        crate::operation::put_object_acl::builders::PutObjectAclOutputBuilder::default()
    }
}

/// A builder for [`PutObjectAclOutput`](crate::operation::put_object_acl::PutObjectAclOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutObjectAclOutputBuilder {
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutObjectAclOutputBuilder {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = ::std::option::Option::Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_request_charged(mut self, input: ::std::option::Option<crate::types::RequestCharged>) -> Self {
        self.request_charged = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_request_charged(&self) -> &::std::option::Option<crate::types::RequestCharged> {
        &self.request_charged
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutObjectAclOutput`](crate::operation::put_object_acl::PutObjectAclOutput).
    pub fn build(self) -> crate::operation::put_object_acl::PutObjectAclOutput {
        crate::operation::put_object_acl::PutObjectAclOutput {
            request_charged: self.request_charged,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
