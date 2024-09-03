// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the versioning state of an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTVersioningStatus.html">PUT Bucket versioning</a> in the <i>Amazon S3 API Reference</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: ::std::option::Option<crate::types::MfaDelete>,
    /// <p>The versioning state of the bucket.</p>
    pub status: ::std::option::Option<crate::types::BucketVersioningStatus>,
}
impl VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub fn mfa_delete(&self) -> ::std::option::Option<&crate::types::MfaDelete> {
        self.mfa_delete.as_ref()
    }
    /// <p>The versioning state of the bucket.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::BucketVersioningStatus> {
        self.status.as_ref()
    }
}
impl VersioningConfiguration {
    /// Creates a new builder-style object to manufacture [`VersioningConfiguration`](crate::types::VersioningConfiguration).
    pub fn builder() -> crate::types::builders::VersioningConfigurationBuilder {
        crate::types::builders::VersioningConfigurationBuilder::default()
    }
}

/// A builder for [`VersioningConfiguration`](crate::types::VersioningConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VersioningConfigurationBuilder {
    pub(crate) mfa_delete: ::std::option::Option<crate::types::MfaDelete>,
    pub(crate) status: ::std::option::Option<crate::types::BucketVersioningStatus>,
}
impl VersioningConfigurationBuilder {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub fn mfa_delete(mut self, input: crate::types::MfaDelete) -> Self {
        self.mfa_delete = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub fn set_mfa_delete(mut self, input: ::std::option::Option<crate::types::MfaDelete>) -> Self {
        self.mfa_delete = input;
        self
    }
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub fn get_mfa_delete(&self) -> &::std::option::Option<crate::types::MfaDelete> {
        &self.mfa_delete
    }
    /// <p>The versioning state of the bucket.</p>
    pub fn status(mut self, input: crate::types::BucketVersioningStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The versioning state of the bucket.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::BucketVersioningStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The versioning state of the bucket.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::BucketVersioningStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`VersioningConfiguration`](crate::types::VersioningConfiguration).
    pub fn build(self) -> crate::types::VersioningConfiguration {
        crate::types::VersioningConfiguration {
            mfa_delete: self.mfa_delete,
            status: self.status,
        }
    }
}
