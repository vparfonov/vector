// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBucketLifecycleInput {
    /// <p>The bucket name of the lifecycle to delete.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl DeleteBucketLifecycleInput {
    /// <p>The bucket name of the lifecycle to delete.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl DeleteBucketLifecycleInput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
    pub fn builder() -> crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleInputBuilder {
        crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleInputBuilder::default()
    }
}

/// A builder for [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteBucketLifecycleInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl DeleteBucketLifecycleInputBuilder {
    /// <p>The bucket name of the lifecycle to delete.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bucket name of the lifecycle to delete.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The bucket name of the lifecycle to delete.</p>
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
    /// Consumes the builder and constructs a [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput {
            bucket: self.bucket,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}
