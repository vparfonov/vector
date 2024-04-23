// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_policy::_put_bucket_policy_output::PutBucketPolicyOutputBuilder;

pub use crate::operation::put_bucket_policy::_put_bucket_policy_input::PutBucketPolicyInputBuilder;

impl PutBucketPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_policy::PutBucketPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_bucket_policy::PutBucketPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_bucket_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutBucketPolicy`.
///
/// <p>Applies an Amazon S3 bucket policy to an Amazon S3 bucket.</p> <note>
/// <p> <b>Directory buckets </b> - For directory buckets, you must make requests for this API operation to the Regional endpoint. These endpoints support path-style requests in the format <code>https://s3express-control.<i>region_code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-Regions-and-Zones.html">Regional and Zonal endpoints</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </note>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <p>If you are using an identity other than the root user of the Amazon Web Services account that owns the bucket, the calling identity must both have the <code>PutBucketPolicy</code> permissions on the specified bucket and belong to the bucket owner's account in order to use this operation.</p>
/// <p>If you don't have <code>PutBucketPolicy</code> permissions, Amazon S3 returns a <code>403 Access Denied</code> error. If you have the correct permissions, but you're not using an identity that belongs to the bucket owner's account, Amazon S3 returns a <code>405 Method Not Allowed</code> error.</p> <important>
/// <p>To ensure that bucket owners don't inadvertently lock themselves out of their own buckets, the root principal in a bucket owner's Amazon Web Services account can perform the <code>GetBucketPolicy</code>, <code>PutBucketPolicy</code>, and <code>DeleteBucketPolicy</code> API actions, even if their bucket policy explicitly denies the root principal's access. Bucket owner root principals can only be blocked from performing these API actions by VPC endpoint policies and Amazon Web Services Organizations policies.</p>
/// </important>
/// <ul>
/// <li> <p> <b>General purpose bucket permissions</b> - The <code>s3:PutBucketPolicy</code> permission is required in a policy. For more information about general purpose buckets bucket policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-iam-policies.html">Using Bucket Policies and User Policies</a> in the <i>Amazon S3 User Guide</i>.</p> </li>
/// <li> <p> <b>Directory bucket permissions</b> - To grant access to this API operation, you must have the <code>s3express:PutBucketPolicy</code> permission in an IAM identity-based policy instead of a bucket policy. Cross-account access to this API operation isn't supported. This operation can only be performed by the Amazon Web Services account that owns the resource. For more information about directory bucket policies and permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-security-iam.html">Amazon Web Services Identity and Access Management (IAM) for S3 Express One Zone</a> in the <i>Amazon S3 User Guide</i>.</p> </li>
/// </ul>
/// </dd>
/// <dt>
/// Example bucket policies
/// </dt>
/// <dd>
/// <p> <b>General purpose buckets example bucket policies</b> - See <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-bucket-policies.html">Bucket policy examples</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p> <b>Directory bucket example bucket policies</b> - See <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-security-iam-example-bucket-policies.html">Example bucket policies for S3 Express One Zone</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </dd>
/// <dt>
/// HTTP Host header syntax
/// </dt>
/// <dd>
/// <p> <b>Directory buckets </b> - The HTTP Host header syntax is <code>s3express-control.<i>region</i>.amazonaws.com</code>.</p>
/// </dd>
/// </dl>
/// <p>The following operations are related to <code>PutBucketPolicy</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutBucketPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_policy::builders::PutBucketPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_bucket_policy::PutBucketPolicyOutput,
        crate::operation::put_bucket_policy::PutBucketPolicyError,
    > for PutBucketPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_bucket_policy::PutBucketPolicyOutput,
            crate::operation::put_bucket_policy::PutBucketPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutBucketPolicyFluentBuilder {
    /// Creates a new `PutBucketPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutBucketPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::put_bucket_policy::builders::PutBucketPolicyInputBuilder {
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
        crate::operation::put_bucket_policy::PutBucketPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_bucket_policy::PutBucketPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_bucket_policy::PutBucketPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_bucket_policy::PutBucketPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_bucket_policy::PutBucketPolicyOutput,
        crate::operation::put_bucket_policy::PutBucketPolicyError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the bucket.</p>
    /// <p> <b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region_code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must also follow the format <code> <i>bucket_base_name</i>--<i>az_id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i> </p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket.</p>
    /// <p> <b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region_code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must also follow the format <code> <i>bucket_base_name</i>--<i>az_id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i> </p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket.</p>
    /// <p> <b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region_code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must also follow the format <code> <i>bucket_base_name</i>--<i>az_id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i> </p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_content_md5()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum-<i>algorithm</i> </code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>.</p>
    /// <p>For the <code>x-amz-checksum-<i>algorithm</i> </code> header, replace <code> <i>algorithm</i> </code> with the supported algorithm from the following list: </p>
    /// <ul>
    /// <li> <p>CRC32</p> </li>
    /// <li> <p>CRC32C</p> </li>
    /// <li> <p>SHA1</p> </li>
    /// <li> <p>SHA256</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If the individual checksum value you provide through <code>x-amz-checksum-<i>algorithm</i> </code> doesn't match the checksum algorithm you set through <code>x-amz-sdk-checksum-algorithm</code>, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter and uses the checksum algorithm that matches the provided value in <code>x-amz-checksum-<i>algorithm</i> </code>.</p> <note>
    /// <p>For directory buckets, when you use Amazon Web Services SDKs, <code>CRC32</code> is the default checksum algorithm that's used for performance.</p>
    /// </note>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum-<i>algorithm</i> </code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>.</p>
    /// <p>For the <code>x-amz-checksum-<i>algorithm</i> </code> header, replace <code> <i>algorithm</i> </code> with the supported algorithm from the following list: </p>
    /// <ul>
    /// <li> <p>CRC32</p> </li>
    /// <li> <p>CRC32C</p> </li>
    /// <li> <p>SHA1</p> </li>
    /// <li> <p>SHA256</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If the individual checksum value you provide through <code>x-amz-checksum-<i>algorithm</i> </code> doesn't match the checksum algorithm you set through <code>x-amz-sdk-checksum-algorithm</code>, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter and uses the checksum algorithm that matches the provided value in <code>x-amz-checksum-<i>algorithm</i> </code>.</p> <note>
    /// <p>For directory buckets, when you use Amazon Web Services SDKs, <code>CRC32</code> is the default checksum algorithm that's used for performance.</p>
    /// </note>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum-<i>algorithm</i> </code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>.</p>
    /// <p>For the <code>x-amz-checksum-<i>algorithm</i> </code> header, replace <code> <i>algorithm</i> </code> with the supported algorithm from the following list: </p>
    /// <ul>
    /// <li> <p>CRC32</p> </li>
    /// <li> <p>CRC32C</p> </li>
    /// <li> <p>SHA1</p> </li>
    /// <li> <p>SHA256</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If the individual checksum value you provide through <code>x-amz-checksum-<i>algorithm</i> </code> doesn't match the checksum algorithm you set through <code>x-amz-sdk-checksum-algorithm</code>, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter and uses the checksum algorithm that matches the provided value in <code>x-amz-checksum-<i>algorithm</i> </code>.</p> <note>
    /// <p>For directory buckets, when you use Amazon Web Services SDKs, <code>CRC32</code> is the default checksum algorithm that's used for performance.</p>
    /// </note>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        self.inner.get_checksum_algorithm()
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn confirm_remove_self_bucket_access(mut self, input: bool) -> Self {
        self.inner = self.inner.confirm_remove_self_bucket_access(input);
        self
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_confirm_remove_self_bucket_access(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_confirm_remove_self_bucket_access(input);
        self
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p> <note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_confirm_remove_self_bucket_access(&self) -> &::std::option::Option<bool> {
        self.inner.get_confirm_remove_self_bucket_access()
    }
    /// <p>The bucket policy as a JSON document.</p>
    /// <p>For directory buckets, the only IAM action supported in the bucket policy is <code>s3express:CreateSession</code>.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    /// <p>For directory buckets, the only IAM action supported in the bucket policy is <code>s3express:CreateSession</code>.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    /// <p>For directory buckets, the only IAM action supported in the bucket policy is <code>s3express:CreateSession</code>.</p>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p> <note>
    /// <p>For directory buckets, this header is not supported in this API operation. If you specify this header, the request fails with the HTTP status code <code>501 Not Implemented</code>.</p>
    /// </note>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p> <note>
    /// <p>For directory buckets, this header is not supported in this API operation. If you specify this header, the request fails with the HTTP status code <code>501 Not Implemented</code>.</p>
    /// </note>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p> <note>
    /// <p>For directory buckets, this header is not supported in this API operation. If you specify this header, the request fails with the HTTP status code <code>501 Not Implemented</code>.</p>
    /// </note>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}
