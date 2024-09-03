// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_bucket_intelligent_tiering_configuration::_delete_bucket_intelligent_tiering_configuration_output::DeleteBucketIntelligentTieringConfigurationOutputBuilder;

pub use crate::operation::delete_bucket_intelligent_tiering_configuration::_delete_bucket_intelligent_tiering_configuration_input::DeleteBucketIntelligentTieringConfigurationInputBuilder;

impl crate::operation::delete_bucket_intelligent_tiering_configuration::builders::DeleteBucketIntelligentTieringConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_bucket_intelligent_tiering_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteBucketIntelligentTieringConfiguration`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>Deletes the S3 Intelligent-Tiering configuration from the specified bucket.</p>
/// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
/// <p>The S3 Intelligent-Tiering storage class is the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
/// <p>Operations related to <code>DeleteBucketIntelligentTieringConfiguration</code> include:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html">GetBucketIntelligentTieringConfiguration</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html">PutBucketIntelligentTieringConfiguration</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBucketIntelligentTieringConfigurations.html">ListBucketIntelligentTieringConfigurations</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteBucketIntelligentTieringConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_bucket_intelligent_tiering_configuration::builders::DeleteBucketIntelligentTieringConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationOutput,
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationError,
    > for DeleteBucketIntelligentTieringConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationOutput,
            crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteBucketIntelligentTieringConfigurationFluentBuilder {
    /// Creates a new `DeleteBucketIntelligentTieringConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteBucketIntelligentTieringConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::delete_bucket_intelligent_tiering_configuration::builders::DeleteBucketIntelligentTieringConfigurationInputBuilder {
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
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfiguration::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationOutput,
        crate::operation::delete_bucket_intelligent_tiering_configuration::DeleteBucketIntelligentTieringConfigurationError,
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
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
