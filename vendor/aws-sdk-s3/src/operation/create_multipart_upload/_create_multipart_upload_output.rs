// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateMultipartUploadOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines the abort action.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub abort_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub abort_rule_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p><note>
    /// <p>Access points are not supported by directory buckets.</p>
    /// </note>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: ::std::option::Option<::std::string::String>,
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub server_side_encryption: ::std::option::Option<crate::types::ServerSideEncryption>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to confirm the encryption algorithm that's used.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub sse_customer_algorithm: ::std::option::Option<::std::string::String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide the round-trip message integrity verification of the customer-provided encryption key.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub sse_customer_key_md5: ::std::option::Option<::std::string::String>,
    /// <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub ssekms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>If present, indicates the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub ssekms_encryption_context: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub bucket_key_enabled: ::std::option::Option<bool>,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CreateMultipartUploadOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines the abort action.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn abort_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.abort_date.as_ref()
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn abort_rule_id(&self) -> ::std::option::Option<&str> {
        self.abort_rule_id.as_deref()
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p><note>
    /// <p>Access points are not supported by directory buckets.</p>
    /// </note>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn upload_id(&self) -> ::std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn server_side_encryption(&self) -> ::std::option::Option<&crate::types::ServerSideEncryption> {
        self.server_side_encryption.as_ref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to confirm the encryption algorithm that's used.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_algorithm(&self) -> ::std::option::Option<&str> {
        self.sse_customer_algorithm.as_deref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide the round-trip message integrity verification of the customer-provided encryption key.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key_md5(&self) -> ::std::option::Option<&str> {
        self.sse_customer_key_md5.as_deref()
    }
    /// <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn ssekms_key_id(&self) -> ::std::option::Option<&str> {
        self.ssekms_key_id.as_deref()
    }
    /// <p>If present, indicates the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn ssekms_encryption_context(&self) -> ::std::option::Option<&str> {
        self.ssekms_encryption_context.as_deref()
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn bucket_key_enabled(&self) -> ::std::option::Option<bool> {
        self.bucket_key_enabled
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn request_charged(&self) -> ::std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(&self) -> ::std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl ::std::fmt::Debug for CreateMultipartUploadOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMultipartUploadOutput");
        formatter.field("abort_date", &self.abort_date);
        formatter.field("abort_rule_id", &self.abort_rule_id);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("ssekms_encryption_context", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl crate::s3_request_id::RequestIdExt for CreateMultipartUploadOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateMultipartUploadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateMultipartUploadOutput {
    /// Creates a new builder-style object to manufacture [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
    pub fn builder() -> crate::operation::create_multipart_upload::builders::CreateMultipartUploadOutputBuilder {
        crate::operation::create_multipart_upload::builders::CreateMultipartUploadOutputBuilder::default()
    }
}

/// A builder for [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateMultipartUploadOutputBuilder {
    pub(crate) abort_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) abort_rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
    pub(crate) server_side_encryption: ::std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) sse_customer_algorithm: ::std::option::Option<::std::string::String>,
    pub(crate) sse_customer_key_md5: ::std::option::Option<::std::string::String>,
    pub(crate) ssekms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) ssekms_encryption_context: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_key_enabled: ::std::option::Option<bool>,
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    pub(crate) checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CreateMultipartUploadOutputBuilder {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines the abort action.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn abort_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.abort_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines the abort action.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_abort_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.abort_date = input;
        self
    }
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Configuration</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines the abort action.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_abort_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.abort_date
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn abort_rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.abort_rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_abort_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.abort_rule_id = input;
        self
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_abort_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.abort_rule_id
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p><note>
    /// <p>Access points are not supported by directory buckets.</p>
    /// </note>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p><note>
    /// <p>Access points are not supported by directory buckets.</p>
    /// </note>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p><note>
    /// <p>Access points are not supported by directory buckets.</p>
    /// </note>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn server_side_encryption(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.server_side_encryption = ::std::option::Option::Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn set_server_side_encryption(mut self, input: ::std::option::Option<crate::types::ServerSideEncryption>) -> Self {
        self.server_side_encryption = input;
        self
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn get_server_side_encryption(&self) -> &::std::option::Option<crate::types::ServerSideEncryption> {
        &self.server_side_encryption
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to confirm the encryption algorithm that's used.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sse_customer_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to confirm the encryption algorithm that's used.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_sse_customer_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sse_customer_algorithm = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to confirm the encryption algorithm that's used.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_sse_customer_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        &self.sse_customer_algorithm
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide the round-trip message integrity verification of the customer-provided encryption key.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn sse_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sse_customer_key_md5 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide the round-trip message integrity verification of the customer-provided encryption key.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_sse_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sse_customer_key_md5 = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide the round-trip message integrity verification of the customer-provided encryption key.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_sse_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.sse_customer_key_md5
    }
    /// <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn ssekms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ssekms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_ssekms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ssekms_key_id = input;
        self
    }
    /// <p>If present, indicates the ID of the Key Management Service (KMS) symmetric encryption customer managed key that was used for the object.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_ssekms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ssekms_key_id
    }
    /// <p>If present, indicates the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn ssekms_encryption_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ssekms_encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If present, indicates the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_ssekms_encryption_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ssekms_encryption_context = input;
        self
    }
    /// <p>If present, indicates the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_ssekms_encryption_context(&self) -> &::std::option::Option<::std::string::String> {
        &self.ssekms_encryption_context
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn bucket_key_enabled(mut self, input: bool) -> Self {
        self.bucket_key_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_bucket_key_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.bucket_key_enabled = input;
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_bucket_key_enabled(&self) -> &::std::option::Option<bool> {
        &self.bucket_key_enabled
    }
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
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.checksum_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.checksum_algorithm = input;
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        &self.checksum_algorithm
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
    /// Consumes the builder and constructs a [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
    pub fn build(self) -> crate::operation::create_multipart_upload::CreateMultipartUploadOutput {
        crate::operation::create_multipart_upload::CreateMultipartUploadOutput {
            abort_date: self.abort_date,
            abort_rule_id: self.abort_rule_id,
            bucket: self.bucket,
            key: self.key,
            upload_id: self.upload_id,
            server_side_encryption: self.server_side_encryption,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key_md5: self.sse_customer_key_md5,
            ssekms_key_id: self.ssekms_key_id,
            ssekms_encryption_context: self.ssekms_encryption_context,
            bucket_key_enabled: self.bucket_key_enabled,
            request_charged: self.request_charged,
            checksum_algorithm: self.checksum_algorithm,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for CreateMultipartUploadOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMultipartUploadOutputBuilder");
        formatter.field("abort_date", &self.abort_date);
        formatter.field("abort_rule_id", &self.abort_rule_id);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("ssekms_encryption_context", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
