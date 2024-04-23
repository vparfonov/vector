// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPublicAccessBlockInput {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetPublicAccessBlockInput {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl GetPublicAccessBlockInput {
    /// Creates a new builder-style object to manufacture [`GetPublicAccessBlockInput`](crate::operation::get_public_access_block::GetPublicAccessBlockInput).
    pub fn builder() -> crate::operation::get_public_access_block::builders::GetPublicAccessBlockInputBuilder {
        crate::operation::get_public_access_block::builders::GetPublicAccessBlockInputBuilder::default()
    }
}

/// A builder for [`GetPublicAccessBlockInput`](crate::operation::get_public_access_block::GetPublicAccessBlockInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetPublicAccessBlockInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetPublicAccessBlockInputBuilder {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_bucket_owner
    }
    /// Consumes the builder and constructs a [`GetPublicAccessBlockInput`](crate::operation::get_public_access_block::GetPublicAccessBlockInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_public_access_block::GetPublicAccessBlockInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_public_access_block::GetPublicAccessBlockInput {
            bucket: self.bucket,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}
