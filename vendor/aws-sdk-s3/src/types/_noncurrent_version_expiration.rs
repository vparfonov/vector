// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NoncurrentVersionExpiration {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub noncurrent_days: ::std::option::Option<i32>,
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100 noncurrent versions to retain. Amazon S3 will permanently delete any additional noncurrent versions beyond the specified number to retain. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub newer_noncurrent_versions: ::std::option::Option<i32>,
}
impl NoncurrentVersionExpiration {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn noncurrent_days(&self) -> ::std::option::Option<i32> {
        self.noncurrent_days
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100 noncurrent versions to retain. Amazon S3 will permanently delete any additional noncurrent versions beyond the specified number to retain. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn newer_noncurrent_versions(&self) -> ::std::option::Option<i32> {
        self.newer_noncurrent_versions
    }
}
impl NoncurrentVersionExpiration {
    /// Creates a new builder-style object to manufacture [`NoncurrentVersionExpiration`](crate::types::NoncurrentVersionExpiration).
    pub fn builder() -> crate::types::builders::NoncurrentVersionExpirationBuilder {
        crate::types::builders::NoncurrentVersionExpirationBuilder::default()
    }
}

/// A builder for [`NoncurrentVersionExpiration`](crate::types::NoncurrentVersionExpiration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct NoncurrentVersionExpirationBuilder {
    pub(crate) noncurrent_days: ::std::option::Option<i32>,
    pub(crate) newer_noncurrent_versions: ::std::option::Option<i32>,
}
impl NoncurrentVersionExpirationBuilder {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn noncurrent_days(mut self, input: i32) -> Self {
        self.noncurrent_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_noncurrent_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.noncurrent_days = input;
        self
    }
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. The value must be a non-zero positive integer. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_noncurrent_days(&self) -> &::std::option::Option<i32> {
        &self.noncurrent_days
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100 noncurrent versions to retain. Amazon S3 will permanently delete any additional noncurrent versions beyond the specified number to retain. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn newer_noncurrent_versions(mut self, input: i32) -> Self {
        self.newer_noncurrent_versions = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100 noncurrent versions to retain. Amazon S3 will permanently delete any additional noncurrent versions beyond the specified number to retain. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_newer_noncurrent_versions(mut self, input: ::std::option::Option<i32>) -> Self {
        self.newer_noncurrent_versions = input;
        self
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. You can specify up to 100 noncurrent versions to retain. Amazon S3 will permanently delete any additional noncurrent versions beyond the specified number to retain. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_newer_noncurrent_versions(&self) -> &::std::option::Option<i32> {
        &self.newer_noncurrent_versions
    }
    /// Consumes the builder and constructs a [`NoncurrentVersionExpiration`](crate::types::NoncurrentVersionExpiration).
    pub fn build(self) -> crate::types::NoncurrentVersionExpiration {
        crate::types::NoncurrentVersionExpiration {
            noncurrent_days: self.noncurrent_days,
            newer_noncurrent_versions: self.newer_noncurrent_versions,
        }
    }
}
