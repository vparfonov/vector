// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration of a destination in the Serverless offering for Amazon OpenSearch Service.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AmazonOpenSearchServerlessDestinationConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.</p>
    pub role_arn: ::std::string::String,
    /// <p>The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.</p>
    pub collection_endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The Serverless offering for Amazon OpenSearch Service index name.</p>
    pub index_name: ::std::string::String,
    /// <p>The buffering options. If no value is specified, the default values for AmazonopensearchserviceBufferingHints are used.</p>
    pub buffering_hints: ::std::option::Option<crate::types::AmazonOpenSearchServerlessBufferingHints>,
    /// <p>The retry behavior in case Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service. The default value is 300 (5 minutes).</p>
    pub retry_options: ::std::option::Option<crate::types::AmazonOpenSearchServerlessRetryOptions>,
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with AmazonOpenSearchService-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with AmazonOpenSearchService-failed/ appended to the prefix.</p>
    pub s3_backup_mode: ::std::option::Option<crate::types::AmazonOpenSearchServerlessS3BackupMode>,
    /// <p>Describes the configuration of a destination in Amazon S3.</p>
    pub s3_configuration: ::std::option::Option<crate::types::S3DestinationConfiguration>,
    /// <p>Describes a data processing configuration.</p>
    pub processing_configuration: ::std::option::Option<crate::types::ProcessingConfiguration>,
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
    /// <p>The details of the VPC of the Amazon OpenSearch or Amazon OpenSearch Serverless destination.</p>
    pub vpc_configuration: ::std::option::Option<crate::types::VpcConfiguration>,
}
impl AmazonOpenSearchServerlessDestinationConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.</p>
    pub fn role_arn(&self) -> &str {
        use std::ops::Deref;
        self.role_arn.deref()
    }
    /// <p>The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.</p>
    pub fn collection_endpoint(&self) -> ::std::option::Option<&str> {
        self.collection_endpoint.as_deref()
    }
    /// <p>The Serverless offering for Amazon OpenSearch Service index name.</p>
    pub fn index_name(&self) -> &str {
        use std::ops::Deref;
        self.index_name.deref()
    }
    /// <p>The buffering options. If no value is specified, the default values for AmazonopensearchserviceBufferingHints are used.</p>
    pub fn buffering_hints(&self) -> ::std::option::Option<&crate::types::AmazonOpenSearchServerlessBufferingHints> {
        self.buffering_hints.as_ref()
    }
    /// <p>The retry behavior in case Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service. The default value is 300 (5 minutes).</p>
    pub fn retry_options(&self) -> ::std::option::Option<&crate::types::AmazonOpenSearchServerlessRetryOptions> {
        self.retry_options.as_ref()
    }
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with AmazonOpenSearchService-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with AmazonOpenSearchService-failed/ appended to the prefix.</p>
    pub fn s3_backup_mode(&self) -> ::std::option::Option<&crate::types::AmazonOpenSearchServerlessS3BackupMode> {
        self.s3_backup_mode.as_ref()
    }
    /// <p>Describes the configuration of a destination in Amazon S3.</p>
    pub fn s3_configuration(&self) -> ::std::option::Option<&crate::types::S3DestinationConfiguration> {
        self.s3_configuration.as_ref()
    }
    /// <p>Describes a data processing configuration.</p>
    pub fn processing_configuration(&self) -> ::std::option::Option<&crate::types::ProcessingConfiguration> {
        self.processing_configuration.as_ref()
    }
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub fn cloud_watch_logging_options(&self) -> ::std::option::Option<&crate::types::CloudWatchLoggingOptions> {
        self.cloud_watch_logging_options.as_ref()
    }
    /// <p>The details of the VPC of the Amazon OpenSearch or Amazon OpenSearch Serverless destination.</p>
    pub fn vpc_configuration(&self) -> ::std::option::Option<&crate::types::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
}
impl AmazonOpenSearchServerlessDestinationConfiguration {
    /// Creates a new builder-style object to manufacture [`AmazonOpenSearchServerlessDestinationConfiguration`](crate::types::AmazonOpenSearchServerlessDestinationConfiguration).
    pub fn builder() -> crate::types::builders::AmazonOpenSearchServerlessDestinationConfigurationBuilder {
        crate::types::builders::AmazonOpenSearchServerlessDestinationConfigurationBuilder::default()
    }
}

/// A builder for [`AmazonOpenSearchServerlessDestinationConfiguration`](crate::types::AmazonOpenSearchServerlessDestinationConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AmazonOpenSearchServerlessDestinationConfigurationBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) collection_endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) buffering_hints: ::std::option::Option<crate::types::AmazonOpenSearchServerlessBufferingHints>,
    pub(crate) retry_options: ::std::option::Option<crate::types::AmazonOpenSearchServerlessRetryOptions>,
    pub(crate) s3_backup_mode: ::std::option::Option<crate::types::AmazonOpenSearchServerlessS3BackupMode>,
    pub(crate) s3_configuration: ::std::option::Option<crate::types::S3DestinationConfiguration>,
    pub(crate) processing_configuration: ::std::option::Option<crate::types::ProcessingConfiguration>,
    pub(crate) cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
    pub(crate) vpc_configuration: ::std::option::Option<crate::types::VpcConfiguration>,
}
impl AmazonOpenSearchServerlessDestinationConfigurationBuilder {
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.</p>
    /// This field is required.
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.</p>
    pub fn collection_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.collection_endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.</p>
    pub fn set_collection_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.collection_endpoint = input;
        self
    }
    /// <p>The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.</p>
    pub fn get_collection_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        &self.collection_endpoint
    }
    /// <p>The Serverless offering for Amazon OpenSearch Service index name.</p>
    /// This field is required.
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Serverless offering for Amazon OpenSearch Service index name.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The Serverless offering for Amazon OpenSearch Service index name.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// <p>The buffering options. If no value is specified, the default values for AmazonopensearchserviceBufferingHints are used.</p>
    pub fn buffering_hints(mut self, input: crate::types::AmazonOpenSearchServerlessBufferingHints) -> Self {
        self.buffering_hints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The buffering options. If no value is specified, the default values for AmazonopensearchserviceBufferingHints are used.</p>
    pub fn set_buffering_hints(mut self, input: ::std::option::Option<crate::types::AmazonOpenSearchServerlessBufferingHints>) -> Self {
        self.buffering_hints = input;
        self
    }
    /// <p>The buffering options. If no value is specified, the default values for AmazonopensearchserviceBufferingHints are used.</p>
    pub fn get_buffering_hints(&self) -> &::std::option::Option<crate::types::AmazonOpenSearchServerlessBufferingHints> {
        &self.buffering_hints
    }
    /// <p>The retry behavior in case Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service. The default value is 300 (5 minutes).</p>
    pub fn retry_options(mut self, input: crate::types::AmazonOpenSearchServerlessRetryOptions) -> Self {
        self.retry_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The retry behavior in case Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service. The default value is 300 (5 minutes).</p>
    pub fn set_retry_options(mut self, input: ::std::option::Option<crate::types::AmazonOpenSearchServerlessRetryOptions>) -> Self {
        self.retry_options = input;
        self
    }
    /// <p>The retry behavior in case Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service. The default value is 300 (5 minutes).</p>
    pub fn get_retry_options(&self) -> &::std::option::Option<crate::types::AmazonOpenSearchServerlessRetryOptions> {
        &self.retry_options
    }
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with AmazonOpenSearchService-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with AmazonOpenSearchService-failed/ appended to the prefix.</p>
    pub fn s3_backup_mode(mut self, input: crate::types::AmazonOpenSearchServerlessS3BackupMode) -> Self {
        self.s3_backup_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with AmazonOpenSearchService-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with AmazonOpenSearchService-failed/ appended to the prefix.</p>
    pub fn set_s3_backup_mode(mut self, input: ::std::option::Option<crate::types::AmazonOpenSearchServerlessS3BackupMode>) -> Self {
        self.s3_backup_mode = input;
        self
    }
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with AmazonOpenSearchService-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with AmazonOpenSearchService-failed/ appended to the prefix.</p>
    pub fn get_s3_backup_mode(&self) -> &::std::option::Option<crate::types::AmazonOpenSearchServerlessS3BackupMode> {
        &self.s3_backup_mode
    }
    /// <p>Describes the configuration of a destination in Amazon S3.</p>
    /// This field is required.
    pub fn s3_configuration(mut self, input: crate::types::S3DestinationConfiguration) -> Self {
        self.s3_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the configuration of a destination in Amazon S3.</p>
    pub fn set_s3_configuration(mut self, input: ::std::option::Option<crate::types::S3DestinationConfiguration>) -> Self {
        self.s3_configuration = input;
        self
    }
    /// <p>Describes the configuration of a destination in Amazon S3.</p>
    pub fn get_s3_configuration(&self) -> &::std::option::Option<crate::types::S3DestinationConfiguration> {
        &self.s3_configuration
    }
    /// <p>Describes a data processing configuration.</p>
    pub fn processing_configuration(mut self, input: crate::types::ProcessingConfiguration) -> Self {
        self.processing_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes a data processing configuration.</p>
    pub fn set_processing_configuration(mut self, input: ::std::option::Option<crate::types::ProcessingConfiguration>) -> Self {
        self.processing_configuration = input;
        self
    }
    /// <p>Describes a data processing configuration.</p>
    pub fn get_processing_configuration(&self) -> &::std::option::Option<crate::types::ProcessingConfiguration> {
        &self.processing_configuration
    }
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub fn cloud_watch_logging_options(mut self, input: crate::types::CloudWatchLoggingOptions) -> Self {
        self.cloud_watch_logging_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub fn set_cloud_watch_logging_options(mut self, input: ::std::option::Option<crate::types::CloudWatchLoggingOptions>) -> Self {
        self.cloud_watch_logging_options = input;
        self
    }
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub fn get_cloud_watch_logging_options(&self) -> &::std::option::Option<crate::types::CloudWatchLoggingOptions> {
        &self.cloud_watch_logging_options
    }
    /// <p>The details of the VPC of the Amazon OpenSearch or Amazon OpenSearch Serverless destination.</p>
    pub fn vpc_configuration(mut self, input: crate::types::VpcConfiguration) -> Self {
        self.vpc_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The details of the VPC of the Amazon OpenSearch or Amazon OpenSearch Serverless destination.</p>
    pub fn set_vpc_configuration(mut self, input: ::std::option::Option<crate::types::VpcConfiguration>) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// <p>The details of the VPC of the Amazon OpenSearch or Amazon OpenSearch Serverless destination.</p>
    pub fn get_vpc_configuration(&self) -> &::std::option::Option<crate::types::VpcConfiguration> {
        &self.vpc_configuration
    }
    /// Consumes the builder and constructs a [`AmazonOpenSearchServerlessDestinationConfiguration`](crate::types::AmazonOpenSearchServerlessDestinationConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`role_arn`](crate::types::builders::AmazonOpenSearchServerlessDestinationConfigurationBuilder::role_arn)
    /// - [`index_name`](crate::types::builders::AmazonOpenSearchServerlessDestinationConfigurationBuilder::index_name)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::AmazonOpenSearchServerlessDestinationConfiguration, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::AmazonOpenSearchServerlessDestinationConfiguration {
            role_arn: self.role_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "role_arn",
                    "role_arn was not specified but it is required when building AmazonOpenSearchServerlessDestinationConfiguration",
                )
            })?,
            collection_endpoint: self.collection_endpoint,
            index_name: self.index_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_name",
                    "index_name was not specified but it is required when building AmazonOpenSearchServerlessDestinationConfiguration",
                )
            })?,
            buffering_hints: self.buffering_hints,
            retry_options: self.retry_options,
            s3_backup_mode: self.s3_backup_mode,
            s3_configuration: self.s3_configuration,
            processing_configuration: self.processing_configuration,
            cloud_watch_logging_options: self.cloud_watch_logging_options,
            vpc_configuration: self.vpc_configuration,
        })
    }
}
