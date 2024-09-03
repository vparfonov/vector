// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketLogging`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket for which to set the logging parameters.</p><br>
    ///   - [`bucket_logging_status(BucketLoggingStatus)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::bucket_logging_status) / [`set_bucket_logging_status(Option<BucketLoggingStatus>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::set_bucket_logging_status):<br>required: **true**<br><p>Container for logging status information.</p><br>
    ///   - [`content_md5(impl Into<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::set_content_md5):<br>required: **false**<br><p>The MD5 hash of the <code>PutBucketLogging</code> request body.</p> <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p><br>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::set_checksum_algorithm):<br>required: **false**<br><p>Indicates the algorithm used to create the checksum for the object when you use the SDK. This header will not provide any additional functionality if you don't use the SDK. When you send this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p> <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`PutBucketLoggingOutput`](crate::operation::put_bucket_logging::PutBucketLoggingOutput)
    /// - On failure, responds with [`SdkError<PutBucketLoggingError>`](crate::operation::put_bucket_logging::PutBucketLoggingError)
    pub fn put_bucket_logging(&self) -> crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder {
        crate::operation::put_bucket_logging::builders::PutBucketLoggingFluentBuilder::new(self.handle.clone())
    }
}
