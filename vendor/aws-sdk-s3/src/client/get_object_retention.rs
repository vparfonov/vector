// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObjectRetention`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_bucket):<br>required: **true**<br><p>The bucket name containing the object whose retention settings you want to retrieve.</p> <p><b>Access points</b> - When you use this action with an access point, you must provide the alias of the access point in place of the bucket name or specify the access point ARN. When using the access point ARN, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p><br>
    ///   - [`key(impl Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_key):<br>required: **true**<br><p>The key name for the object whose retention settings you want to retrieve.</p><br>
    ///   - [`version_id(impl Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_version_id):<br>required: **false**<br><p>The version ID for the object whose retention settings you want to retrieve.</p><br>
    ///   - [`request_payer(RequestPayer)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_request_payer):<br>required: **false**<br><p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p><note>  <p>This functionality is not supported for directory buckets.</p> </note><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`GetObjectRetentionOutput`](crate::operation::get_object_retention::GetObjectRetentionOutput) with field(s):
    ///   - [`retention(Option<ObjectLockRetention>)`](crate::operation::get_object_retention::GetObjectRetentionOutput::retention): <p>The container element for an object's retention settings.</p>
    /// - On failure, responds with [`SdkError<GetObjectRetentionError>`](crate::operation::get_object_retention::GetObjectRetentionError)
    pub fn get_object_retention(&self) -> crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder {
        crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::new(self.handle.clone())
    }
}
