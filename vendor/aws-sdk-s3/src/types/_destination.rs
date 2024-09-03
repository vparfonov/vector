// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies information about where to publish analysis or configuration results for an Amazon S3 bucket and S3 Replication Time Control (S3 RTC).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Destination {
    /// <p>The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    pub bucket: ::std::string::String,
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub account: ::std::option::Option<::std::string::String>,
    /// <p>The storage class to use when replicating objects, such as S3 Standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica.</p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub storage_class: ::std::option::Option<crate::types::StorageClass>,
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the Amazon Web Services account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub access_control_translation: ::std::option::Option<crate::types::AccessControlTranslation>,
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub encryption_configuration: ::std::option::Option<crate::types::EncryptionConfiguration>,
    /// <p>A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. Must be specified together with a <code>Metrics</code> block.</p>
    pub replication_time: ::std::option::Option<crate::types::ReplicationTime>,
    /// <p>A container specifying replication metrics-related settings enabling replication metrics and events.</p>
    pub metrics: ::std::option::Option<crate::types::Metrics>,
}
impl Destination {
    /// <p>The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    pub fn bucket(&self) -> &str {
        use std::ops::Deref;
        self.bucket.deref()
    }
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn account(&self) -> ::std::option::Option<&str> {
        self.account.as_deref()
    }
    /// <p>The storage class to use when replicating objects, such as S3 Standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica.</p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub fn storage_class(&self) -> ::std::option::Option<&crate::types::StorageClass> {
        self.storage_class.as_ref()
    }
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the Amazon Web Services account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub fn access_control_translation(&self) -> ::std::option::Option<&crate::types::AccessControlTranslation> {
        self.access_control_translation.as_ref()
    }
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub fn encryption_configuration(&self) -> ::std::option::Option<&crate::types::EncryptionConfiguration> {
        self.encryption_configuration.as_ref()
    }
    /// <p>A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. Must be specified together with a <code>Metrics</code> block.</p>
    pub fn replication_time(&self) -> ::std::option::Option<&crate::types::ReplicationTime> {
        self.replication_time.as_ref()
    }
    /// <p>A container specifying replication metrics-related settings enabling replication metrics and events.</p>
    pub fn metrics(&self) -> ::std::option::Option<&crate::types::Metrics> {
        self.metrics.as_ref()
    }
}
impl Destination {
    /// Creates a new builder-style object to manufacture [`Destination`](crate::types::Destination).
    pub fn builder() -> crate::types::builders::DestinationBuilder {
        crate::types::builders::DestinationBuilder::default()
    }
}

/// A builder for [`Destination`](crate::types::Destination).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DestinationBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) account: ::std::option::Option<::std::string::String>,
    pub(crate) storage_class: ::std::option::Option<crate::types::StorageClass>,
    pub(crate) access_control_translation: ::std::option::Option<crate::types::AccessControlTranslation>,
    pub(crate) encryption_configuration: ::std::option::Option<crate::types::EncryptionConfiguration>,
    pub(crate) replication_time: ::std::option::Option<crate::types::ReplicationTime>,
    pub(crate) metrics: ::std::option::Option<crate::types::Metrics>,
}
impl DestinationBuilder {
    /// <p>The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the results.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account = input;
        self
    }
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the Amazon Web Services account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-change-owner.html">Replication Additional Configuration: Changing the Replica Owner</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_account(&self) -> &::std::option::Option<::std::string::String> {
        &self.account
    }
    /// <p>The storage class to use when replicating objects, such as S3 Standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica.</p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub fn storage_class(mut self, input: crate::types::StorageClass) -> Self {
        self.storage_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>The storage class to use when replicating objects, such as S3 Standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica.</p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub fn set_storage_class(mut self, input: ::std::option::Option<crate::types::StorageClass>) -> Self {
        self.storage_class = input;
        self
    }
    /// <p>The storage class to use when replicating objects, such as S3 Standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica.</p>
    /// <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon S3 API Reference</i>.</p>
    pub fn get_storage_class(&self) -> &::std::option::Option<crate::types::StorageClass> {
        &self.storage_class
    }
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the Amazon Web Services account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub fn access_control_translation(mut self, input: crate::types::AccessControlTranslation) -> Self {
        self.access_control_translation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the Amazon Web Services account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub fn set_access_control_translation(mut self, input: ::std::option::Option<crate::types::AccessControlTranslation>) -> Self {
        self.access_control_translation = input;
        self
    }
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the Amazon Web Services account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same Amazon Web Services account that owns the source object.</p>
    pub fn get_access_control_translation(&self) -> &::std::option::Option<crate::types::AccessControlTranslation> {
        &self.access_control_translation
    }
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub fn encryption_configuration(mut self, input: crate::types::EncryptionConfiguration) -> Self {
        self.encryption_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub fn set_encryption_configuration(mut self, input: ::std::option::Option<crate::types::EncryptionConfiguration>) -> Self {
        self.encryption_configuration = input;
        self
    }
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub fn get_encryption_configuration(&self) -> &::std::option::Option<crate::types::EncryptionConfiguration> {
        &self.encryption_configuration
    }
    /// <p>A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. Must be specified together with a <code>Metrics</code> block.</p>
    pub fn replication_time(mut self, input: crate::types::ReplicationTime) -> Self {
        self.replication_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. Must be specified together with a <code>Metrics</code> block.</p>
    pub fn set_replication_time(mut self, input: ::std::option::Option<crate::types::ReplicationTime>) -> Self {
        self.replication_time = input;
        self
    }
    /// <p>A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. Must be specified together with a <code>Metrics</code> block.</p>
    pub fn get_replication_time(&self) -> &::std::option::Option<crate::types::ReplicationTime> {
        &self.replication_time
    }
    /// <p>A container specifying replication metrics-related settings enabling replication metrics and events.</p>
    pub fn metrics(mut self, input: crate::types::Metrics) -> Self {
        self.metrics = ::std::option::Option::Some(input);
        self
    }
    /// <p>A container specifying replication metrics-related settings enabling replication metrics and events.</p>
    pub fn set_metrics(mut self, input: ::std::option::Option<crate::types::Metrics>) -> Self {
        self.metrics = input;
        self
    }
    /// <p>A container specifying replication metrics-related settings enabling replication metrics and events.</p>
    pub fn get_metrics(&self) -> &::std::option::Option<crate::types::Metrics> {
        &self.metrics
    }
    /// Consumes the builder and constructs a [`Destination`](crate::types::Destination).
    /// This method will fail if any of the following fields are not set:
    /// - [`bucket`](crate::types::builders::DestinationBuilder::bucket)
    pub fn build(self) -> ::std::result::Result<crate::types::Destination, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Destination {
            bucket: self.bucket.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "bucket",
                    "bucket was not specified but it is required when building Destination",
                )
            })?,
            account: self.account,
            storage_class: self.storage_class,
            access_control_translation: self.access_control_translation,
            encryption_configuration: self.encryption_configuration,
            replication_time: self.replication_time,
            metrics: self.metrics,
        })
    }
}
