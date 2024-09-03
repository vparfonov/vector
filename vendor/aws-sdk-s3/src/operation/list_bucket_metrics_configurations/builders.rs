// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_bucket_metrics_configurations::_list_bucket_metrics_configurations_output::ListBucketMetricsConfigurationsOutputBuilder;

pub use crate::operation::list_bucket_metrics_configurations::_list_bucket_metrics_configurations_input::ListBucketMetricsConfigurationsInputBuilder;

impl crate::operation::list_bucket_metrics_configurations::builders::ListBucketMetricsConfigurationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_bucket_metrics_configurations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBucketMetricsConfigurations`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>Lists the metrics configurations for the bucket. The metrics configurations are only for the request metrics of the bucket and do not provide information on daily storage metrics. You can have up to 1,000 configurations per bucket.</p>
/// <p>This action supports list pagination and does not return more than 100 configurations at a time. Always check the <code>IsTruncated</code> element in the response. If there are no more configurations to list, <code>IsTruncated</code> is set to false. If there are more configurations to list, <code>IsTruncated</code> is set to true, and there is a value in <code>NextContinuationToken</code>. You use the <code>NextContinuationToken</code> value to continue the pagination of the list by passing the value in <code>continuation-token</code> in the request to <code>GET</code> the next page.</p>
/// <p>To use this operation, you must have permissions to perform the <code>s3:GetMetricsConfiguration</code> action. The bucket owner has this permission by default. The bucket owner can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3 Resources</a>.</p>
/// <p>For more information about metrics configurations and CloudWatch request metrics, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cloudwatch-monitoring.html">Monitoring Metrics with Amazon CloudWatch</a>.</p>
/// <p>The following operations are related to <code>ListBucketMetricsConfigurations</code>:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketMetricsConfiguration.html">PutBucketMetricsConfiguration</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketMetricsConfiguration.html">GetBucketMetricsConfiguration</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketMetricsConfiguration.html">DeleteBucketMetricsConfiguration</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBucketMetricsConfigurationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_bucket_metrics_configurations::builders::ListBucketMetricsConfigurationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsOutput,
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsError,
    > for ListBucketMetricsConfigurationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsOutput,
            crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListBucketMetricsConfigurationsFluentBuilder {
    /// Creates a new `ListBucketMetricsConfigurationsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListBucketMetricsConfigurations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_bucket_metrics_configurations::builders::ListBucketMetricsConfigurationsInputBuilder {
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
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsOutput,
        crate::operation::list_bucket_metrics_configurations::ListBucketMetricsConfigurationsError,
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
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn continuation_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.continuation_token(input.into());
        self
    }
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn set_continuation_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_continuation_token(input);
        self
    }
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn get_continuation_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_continuation_token()
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
