// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_object_lock_configuration::_put_object_lock_configuration_output::PutObjectLockConfigurationOutputBuilder;

pub use crate::operation::put_object_lock_configuration::_put_object_lock_configuration_input::PutObjectLockConfigurationInputBuilder;

impl crate::operation::put_object_lock_configuration::builders::PutObjectLockConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_object_lock_configuration::PutObjectLockConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_object_lock_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutObjectLockConfiguration`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>Places an Object Lock configuration on the specified bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Locking Objects</a>.</p><note>
/// <ul>
/// <li>
/// <p>The <code>DefaultRetention</code> settings require both a mode and a period.</p></li>
/// <li>
/// <p>The <code>DefaultRetention</code> period can be either <code>Days</code> or <code>Years</code> but you must select one. You cannot specify <code>Days</code> and <code>Years</code> at the same time.</p></li>
/// <li>
/// <p>You can enable Object Lock for new or existing buckets. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock-configure.html">Configuring Object Lock</a>.</p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutObjectLockConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_object_lock_configuration::builders::PutObjectLockConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationOutput,
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationError,
    > for PutObjectLockConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_object_lock_configuration::PutObjectLockConfigurationOutput,
            crate::operation::put_object_lock_configuration::PutObjectLockConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutObjectLockConfigurationFluentBuilder {
    /// Creates a new `PutObjectLockConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutObjectLockConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_object_lock_configuration::builders::PutObjectLockConfigurationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_object_lock_configuration::PutObjectLockConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_object_lock_configuration::PutObjectLockConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_object_lock_configuration::PutObjectLockConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationOutput,
        crate::operation::put_object_lock_configuration::PutObjectLockConfigurationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The bucket whose Object Lock configuration you want to create or replace.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket whose Object Lock configuration you want to create or replace.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The bucket whose Object Lock configuration you want to create or replace.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The Object Lock configuration that you want to apply to the specified bucket.</p>
    pub fn object_lock_configuration(mut self, input: crate::types::ObjectLockConfiguration) -> Self {
        self.inner = self.inner.object_lock_configuration(input);
        self
    }
    /// <p>The Object Lock configuration that you want to apply to the specified bucket.</p>
    pub fn set_object_lock_configuration(mut self, input: ::std::option::Option<crate::types::ObjectLockConfiguration>) -> Self {
        self.inner = self.inner.set_object_lock_configuration(input);
        self
    }
    /// <p>The Object Lock configuration that you want to apply to the specified bucket.</p>
    pub fn get_object_lock_configuration(&self) -> &::std::option::Option<crate::types::ObjectLockConfiguration> {
        self.inner.get_object_lock_configuration()
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.inner = self.inner.request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_request_payer(mut self, input: ::std::option::Option<crate::types::RequestPayer>) -> Self {
        self.inner = self.inner.set_request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_request_payer(&self) -> &::std::option::Option<crate::types::RequestPayer> {
        self.inner.get_request_payer()
    }
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub fn token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.token(input.into());
        self
    }
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub fn set_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_token(input);
        self
    }
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub fn get_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_token()
    }
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>The MD5 hash for the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_content_md5()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        self.inner.get_checksum_algorithm()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}
