// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container element that identifies who initiated the multipart upload.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Initiator {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p><note>
    /// <p><b>Directory buckets</b> - If the principal is an Amazon Web Services account, it provides the Amazon Web Services account ID. If the principal is an IAM User, it provides a user ARN value.</p>
    /// </note>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>Name of the Principal.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub display_name: ::std::option::Option<::std::string::String>,
}
impl Initiator {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p><note>
    /// <p><b>Directory buckets</b> - If the principal is an Amazon Web Services account, it provides the Amazon Web Services account ID. If the principal is an IAM User, it provides a user ARN value.</p>
    /// </note>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Name of the Principal.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
}
impl Initiator {
    /// Creates a new builder-style object to manufacture [`Initiator`](crate::types::Initiator).
    pub fn builder() -> crate::types::builders::InitiatorBuilder {
        crate::types::builders::InitiatorBuilder::default()
    }
}

/// A builder for [`Initiator`](crate::types::Initiator).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InitiatorBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
}
impl InitiatorBuilder {
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p><note>
    /// <p><b>Directory buckets</b> - If the principal is an Amazon Web Services account, it provides the Amazon Web Services account ID. If the principal is an IAM User, it provides a user ARN value.</p>
    /// </note>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p><note>
    /// <p><b>Directory buckets</b> - If the principal is an Amazon Web Services account, it provides the Amazon Web Services account ID. If the principal is an IAM User, it provides a user ARN value.</p>
    /// </note>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>If the principal is an Amazon Web Services account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p><note>
    /// <p><b>Directory buckets</b> - If the principal is an Amazon Web Services account, it provides the Amazon Web Services account ID. If the principal is an IAM User, it provides a user ARN value.</p>
    /// </note>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>Name of the Principal.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the Principal.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>Name of the Principal.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.display_name
    }
    /// Consumes the builder and constructs a [`Initiator`](crate::types::Initiator).
    pub fn build(self) -> crate::types::Initiator {
        crate::types::Initiator {
            id: self.id,
            display_name: self.display_name,
        }
    }
}
