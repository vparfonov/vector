// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_object_versions::_list_object_versions_output::ListObjectVersionsOutputBuilder;

pub use crate::operation::list_object_versions::_list_object_versions_input::ListObjectVersionsInputBuilder;

impl crate::operation::list_object_versions::builders::ListObjectVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_object_versions::ListObjectVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_object_versions::ListObjectVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_object_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListObjectVersions`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>Returns metadata about all versions of the objects in a bucket. You can also use request parameters as selection criteria to return metadata about a subset of all the object versions.</p><important>
/// <p>To use this operation, you must have permission to perform the <code>s3:ListBucketVersions</code> action. Be aware of the name difference.</p>
/// </important> <note>
/// <p>A <code>200 OK</code> response can contain valid or invalid XML. Make sure to design your application to parse the contents of the response and handle it appropriately.</p>
/// </note>
/// <p>To use this operation, you must have READ access to the bucket.</p>
/// <p>The following operations are related to <code>ListObjectVersions</code>:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html">ListObjectsV2</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html">DeleteObject</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListObjectVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_object_versions::builders::ListObjectVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_object_versions::ListObjectVersionsOutput,
        crate::operation::list_object_versions::ListObjectVersionsError,
    > for ListObjectVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_object_versions::ListObjectVersionsOutput,
            crate::operation::list_object_versions::ListObjectVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListObjectVersionsFluentBuilder {
    /// Creates a new `ListObjectVersionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListObjectVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_object_versions::builders::ListObjectVersionsInputBuilder {
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
        crate::operation::list_object_versions::ListObjectVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_object_versions::ListObjectVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_object_versions::ListObjectVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_object_versions::ListObjectVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_object_versions::ListObjectVersionsOutput,
        crate::operation::list_object_versions::ListObjectVersionsError,
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
    /// <p>The bucket name that contains the objects.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket name that contains the objects.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The bucket name that contains the objects.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>A delimiter is a character that you specify to group keys. All keys that contain the same string between the <code>prefix</code> and the first occurrence of the delimiter are grouped under a single result element in <code>CommonPrefixes</code>. These groups are counted as one result against the <code>max-keys</code> limitation. These keys are not returned elsewhere in the response.</p>
    pub fn delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.delimiter(input.into());
        self
    }
    /// <p>A delimiter is a character that you specify to group keys. All keys that contain the same string between the <code>prefix</code> and the first occurrence of the delimiter are grouped under a single result element in <code>CommonPrefixes</code>. These groups are counted as one result against the <code>max-keys</code> limitation. These keys are not returned elsewhere in the response.</p>
    pub fn set_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_delimiter(input);
        self
    }
    /// <p>A delimiter is a character that you specify to group keys. All keys that contain the same string between the <code>prefix</code> and the first occurrence of the delimiter are grouped under a single result element in <code>CommonPrefixes</code>. These groups are counted as one result against the <code>max-keys</code> limitation. These keys are not returned elsewhere in the response.</p>
    pub fn get_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_delimiter()
    }
    /// <p>Encoding type used by Amazon S3 to encode the <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html">object keys</a> in the response. Responses are encoded only in UTF-8. An object key can contain any Unicode character. However, the XML 1.0 parser can't parse certain characters, such as characters with an ASCII value from 0 to 10. For characters that aren't supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response. For more information about characters to avoid in object key names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-guidelines">Object key naming guidelines</a>.</p><note>
    /// <p>When using the URL encoding type, non-ASCII characters that are used in an object's key name will be percent-encoded according to UTF-8 code values. For example, the object <code>test_file(3).png</code> will appear as <code>test_file%283%29.png</code>.</p>
    /// </note>
    pub fn encoding_type(mut self, input: crate::types::EncodingType) -> Self {
        self.inner = self.inner.encoding_type(input);
        self
    }
    /// <p>Encoding type used by Amazon S3 to encode the <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html">object keys</a> in the response. Responses are encoded only in UTF-8. An object key can contain any Unicode character. However, the XML 1.0 parser can't parse certain characters, such as characters with an ASCII value from 0 to 10. For characters that aren't supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response. For more information about characters to avoid in object key names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-guidelines">Object key naming guidelines</a>.</p><note>
    /// <p>When using the URL encoding type, non-ASCII characters that are used in an object's key name will be percent-encoded according to UTF-8 code values. For example, the object <code>test_file(3).png</code> will appear as <code>test_file%283%29.png</code>.</p>
    /// </note>
    pub fn set_encoding_type(mut self, input: ::std::option::Option<crate::types::EncodingType>) -> Self {
        self.inner = self.inner.set_encoding_type(input);
        self
    }
    /// <p>Encoding type used by Amazon S3 to encode the <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html">object keys</a> in the response. Responses are encoded only in UTF-8. An object key can contain any Unicode character. However, the XML 1.0 parser can't parse certain characters, such as characters with an ASCII value from 0 to 10. For characters that aren't supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response. For more information about characters to avoid in object key names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-guidelines">Object key naming guidelines</a>.</p><note>
    /// <p>When using the URL encoding type, non-ASCII characters that are used in an object's key name will be percent-encoded according to UTF-8 code values. For example, the object <code>test_file(3).png</code> will appear as <code>test_file%283%29.png</code>.</p>
    /// </note>
    pub fn get_encoding_type(&self) -> &::std::option::Option<crate::types::EncodingType> {
        self.inner.get_encoding_type()
    }
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub fn key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_marker(input.into());
        self
    }
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub fn set_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_marker(input);
        self
    }
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub fn get_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_marker()
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. If additional keys satisfy the search criteria, but were not returned because <code>max-keys</code> was exceeded, the response contains <code><istruncated>
    /// true
    /// </istruncated></code>. To return the additional keys, see <code>key-marker</code> and <code>version-id-marker</code>.</p>
    pub fn max_keys(mut self, input: i32) -> Self {
        self.inner = self.inner.max_keys(input);
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. If additional keys satisfy the search criteria, but were not returned because <code>max-keys</code> was exceeded, the response contains <code><istruncated>
    /// true
    /// </istruncated></code>. To return the additional keys, see <code>key-marker</code> and <code>version-id-marker</code>.</p>
    pub fn set_max_keys(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_keys(input);
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. If additional keys satisfy the search criteria, but were not returned because <code>max-keys</code> was exceeded, the response contains <code><istruncated>
    /// true
    /// </istruncated></code>. To return the additional keys, see <code>key-marker</code> and <code>version-id-marker</code>.</p>
    pub fn get_max_keys(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_keys()
    }
    /// <p>Use this parameter to select only those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different groupings of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.) You can use <code>prefix</code> with <code>delimiter</code> to roll up numerous objects into a single result under <code>CommonPrefixes</code>.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.prefix(input.into());
        self
    }
    /// <p>Use this parameter to select only those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different groupings of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.) You can use <code>prefix</code> with <code>delimiter</code> to roll up numerous objects into a single result under <code>CommonPrefixes</code>.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_prefix(input);
        self
    }
    /// <p>Use this parameter to select only those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different groupings of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.) You can use <code>prefix</code> with <code>delimiter</code> to roll up numerous objects into a single result under <code>CommonPrefixes</code>.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_prefix()
    }
    /// <p>Specifies the object version you want to start listing from.</p>
    pub fn version_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id_marker(input.into());
        self
    }
    /// <p>Specifies the object version you want to start listing from.</p>
    pub fn set_version_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id_marker(input);
        self
    }
    /// <p>Specifies the object version you want to start listing from.</p>
    pub fn get_version_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id_marker()
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
    ///
    /// Appends an item to `OptionalObjectAttributes`.
    ///
    /// To override the contents of this collection use [`set_optional_object_attributes`](Self::set_optional_object_attributes).
    ///
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn optional_object_attributes(mut self, input: crate::types::OptionalObjectAttributes) -> Self {
        self.inner = self.inner.optional_object_attributes(input);
        self
    }
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn set_optional_object_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>>) -> Self {
        self.inner = self.inner.set_optional_object_attributes(input);
        self
    }
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn get_optional_object_attributes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>> {
        self.inner.get_optional_object_attributes()
    }
}
