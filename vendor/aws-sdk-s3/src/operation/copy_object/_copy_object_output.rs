// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CopyObjectOutput {
    /// <p>Container for all response elements.</p>
    pub copy_object_result: ::std::option::Option<crate::types::CopyObjectResult>,
    /// <p>If the object expiration is configured, the response includes this header.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub expiration: ::std::option::Option<::std::string::String>,
    /// <p>Version ID of the source object that was copied.</p><note>
    /// <p>This functionality is not supported when the source object is in a directory bucket.</p>
    /// </note>
    pub copy_source_version_id: ::std::option::Option<::std::string::String>,
    /// <p>Version ID of the newly created copy.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub version_id: ::std::option::Option<::std::string::String>,
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>, <code>aws:kms:dsse</code>).</p><note>
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
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub bucket_key_enabled: ::std::option::Option<bool>,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CopyObjectOutput {
    /// <p>Container for all response elements.</p>
    pub fn copy_object_result(&self) -> ::std::option::Option<&crate::types::CopyObjectResult> {
        self.copy_object_result.as_ref()
    }
    /// <p>If the object expiration is configured, the response includes this header.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn expiration(&self) -> ::std::option::Option<&str> {
        self.expiration.as_deref()
    }
    /// <p>Version ID of the source object that was copied.</p><note>
    /// <p>This functionality is not supported when the source object is in a directory bucket.</p>
    /// </note>
    pub fn copy_source_version_id(&self) -> ::std::option::Option<&str> {
        self.copy_source_version_id.as_deref()
    }
    /// <p>Version ID of the newly created copy.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn version_id(&self) -> ::std::option::Option<&str> {
        self.version_id.as_deref()
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>, <code>aws:kms:dsse</code>).</p><note>
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
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
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
}
impl ::std::fmt::Debug for CopyObjectOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CopyObjectOutput");
        formatter.field("copy_object_result", &self.copy_object_result);
        formatter.field("expiration", &self.expiration);
        formatter.field("copy_source_version_id", &self.copy_source_version_id);
        formatter.field("version_id", &self.version_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("ssekms_encryption_context", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl crate::s3_request_id::RequestIdExt for CopyObjectOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CopyObjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CopyObjectOutput {
    /// Creates a new builder-style object to manufacture [`CopyObjectOutput`](crate::operation::copy_object::CopyObjectOutput).
    pub fn builder() -> crate::operation::copy_object::builders::CopyObjectOutputBuilder {
        crate::operation::copy_object::builders::CopyObjectOutputBuilder::default()
    }
}

/// A builder for [`CopyObjectOutput`](crate::operation::copy_object::CopyObjectOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CopyObjectOutputBuilder {
    pub(crate) copy_object_result: ::std::option::Option<crate::types::CopyObjectResult>,
    pub(crate) expiration: ::std::option::Option<::std::string::String>,
    pub(crate) copy_source_version_id: ::std::option::Option<::std::string::String>,
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    pub(crate) server_side_encryption: ::std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) sse_customer_algorithm: ::std::option::Option<::std::string::String>,
    pub(crate) sse_customer_key_md5: ::std::option::Option<::std::string::String>,
    pub(crate) ssekms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) ssekms_encryption_context: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_key_enabled: ::std::option::Option<bool>,
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CopyObjectOutputBuilder {
    /// <p>Container for all response elements.</p>
    pub fn copy_object_result(mut self, input: crate::types::CopyObjectResult) -> Self {
        self.copy_object_result = ::std::option::Option::Some(input);
        self
    }
    /// <p>Container for all response elements.</p>
    pub fn set_copy_object_result(mut self, input: ::std::option::Option<crate::types::CopyObjectResult>) -> Self {
        self.copy_object_result = input;
        self
    }
    /// <p>Container for all response elements.</p>
    pub fn get_copy_object_result(&self) -> &::std::option::Option<crate::types::CopyObjectResult> {
        &self.copy_object_result
    }
    /// <p>If the object expiration is configured, the response includes this header.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn expiration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expiration = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the object expiration is configured, the response includes this header.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_expiration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expiration = input;
        self
    }
    /// <p>If the object expiration is configured, the response includes this header.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_expiration(&self) -> &::std::option::Option<::std::string::String> {
        &self.expiration
    }
    /// <p>Version ID of the source object that was copied.</p><note>
    /// <p>This functionality is not supported when the source object is in a directory bucket.</p>
    /// </note>
    pub fn copy_source_version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.copy_source_version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Version ID of the source object that was copied.</p><note>
    /// <p>This functionality is not supported when the source object is in a directory bucket.</p>
    /// </note>
    pub fn set_copy_source_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.copy_source_version_id = input;
        self
    }
    /// <p>Version ID of the source object that was copied.</p><note>
    /// <p>This functionality is not supported when the source object is in a directory bucket.</p>
    /// </note>
    pub fn get_copy_source_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.copy_source_version_id
    }
    /// <p>Version ID of the newly created copy.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Version ID of the newly created copy.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>Version ID of the newly created copy.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.version_id
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>, <code>aws:kms:dsse</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn server_side_encryption(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.server_side_encryption = ::std::option::Option::Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>, <code>aws:kms:dsse</code>).</p><note>
    /// <p>For directory buckets, only server-side encryption with Amazon S3 managed keys (SSE-S3) (<code>AES256</code>) is supported.</p>
    /// </note>
    pub fn set_server_side_encryption(mut self, input: ::std::option::Option<crate::types::ServerSideEncryption>) -> Self {
        self.server_side_encryption = input;
        self
    }
    /// <p>The server-side encryption algorithm used when you store this object in Amazon S3 (for example, <code>AES256</code>, <code>aws:kms</code>, <code>aws:kms:dsse</code>).</p><note>
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
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn bucket_key_enabled(mut self, input: bool) -> Self {
        self.bucket_key_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_bucket_key_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.bucket_key_enabled = input;
        self
    }
    /// <p>Indicates whether the copied object uses an S3 Bucket Key for server-side encryption with Key Management Service (KMS) keys (SSE-KMS).</p><note>
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
    /// Consumes the builder and constructs a [`CopyObjectOutput`](crate::operation::copy_object::CopyObjectOutput).
    pub fn build(self) -> crate::operation::copy_object::CopyObjectOutput {
        crate::operation::copy_object::CopyObjectOutput {
            copy_object_result: self.copy_object_result,
            expiration: self.expiration,
            copy_source_version_id: self.copy_source_version_id,
            version_id: self.version_id,
            server_side_encryption: self.server_side_encryption,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key_md5: self.sse_customer_key_md5,
            ssekms_key_id: self.ssekms_key_id,
            ssekms_encryption_context: self.ssekms_encryption_context,
            bucket_key_enabled: self.bucket_key_enabled,
            request_charged: self.request_charged,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for CopyObjectOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CopyObjectOutputBuilder");
        formatter.field("copy_object_result", &self.copy_object_result);
        formatter.field("expiration", &self.expiration);
        formatter.field("copy_source_version_id", &self.copy_source_version_id);
        formatter.field("version_id", &self.version_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("ssekms_encryption_context", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
