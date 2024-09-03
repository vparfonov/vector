// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the HTTP endpoint destination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HttpEndpointDestinationDescription {
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    pub endpoint_configuration: ::std::option::Option<crate::types::HttpEndpointDescription>,
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
    pub buffering_hints: ::std::option::Option<crate::types::HttpEndpointBufferingHints>,
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    pub request_configuration: ::std::option::Option<crate::types::HttpEndpointRequestConfiguration>,
    /// <p>Describes a data processing configuration.</p>
    pub processing_configuration: ::std::option::Option<crate::types::ProcessingConfiguration>,
    /// <p>Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Describes the retry behavior in case Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    pub retry_options: ::std::option::Option<crate::types::HttpEndpointRetryOptions>,
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    pub s3_backup_mode: ::std::option::Option<crate::types::HttpEndpointS3BackupMode>,
    /// <p>Describes a destination in Amazon S3.</p>
    pub s3_destination_description: ::std::option::Option<crate::types::S3DestinationDescription>,
    /// <p>The configuration that defines how you access secrets for HTTP Endpoint destination.</p>
    pub secrets_manager_configuration: ::std::option::Option<crate::types::SecretsManagerConfiguration>,
}
impl HttpEndpointDestinationDescription {
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    pub fn endpoint_configuration(&self) -> ::std::option::Option<&crate::types::HttpEndpointDescription> {
        self.endpoint_configuration.as_ref()
    }
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
    pub fn buffering_hints(&self) -> ::std::option::Option<&crate::types::HttpEndpointBufferingHints> {
        self.buffering_hints.as_ref()
    }
    /// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
    pub fn cloud_watch_logging_options(&self) -> ::std::option::Option<&crate::types::CloudWatchLoggingOptions> {
        self.cloud_watch_logging_options.as_ref()
    }
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    pub fn request_configuration(&self) -> ::std::option::Option<&crate::types::HttpEndpointRequestConfiguration> {
        self.request_configuration.as_ref()
    }
    /// <p>Describes a data processing configuration.</p>
    pub fn processing_configuration(&self) -> ::std::option::Option<&crate::types::ProcessingConfiguration> {
        self.processing_configuration.as_ref()
    }
    /// <p>Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Describes the retry behavior in case Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    pub fn retry_options(&self) -> ::std::option::Option<&crate::types::HttpEndpointRetryOptions> {
        self.retry_options.as_ref()
    }
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    pub fn s3_backup_mode(&self) -> ::std::option::Option<&crate::types::HttpEndpointS3BackupMode> {
        self.s3_backup_mode.as_ref()
    }
    /// <p>Describes a destination in Amazon S3.</p>
    pub fn s3_destination_description(&self) -> ::std::option::Option<&crate::types::S3DestinationDescription> {
        self.s3_destination_description.as_ref()
    }
    /// <p>The configuration that defines how you access secrets for HTTP Endpoint destination.</p>
    pub fn secrets_manager_configuration(&self) -> ::std::option::Option<&crate::types::SecretsManagerConfiguration> {
        self.secrets_manager_configuration.as_ref()
    }
}
impl HttpEndpointDestinationDescription {
    /// Creates a new builder-style object to manufacture [`HttpEndpointDestinationDescription`](crate::types::HttpEndpointDestinationDescription).
    pub fn builder() -> crate::types::builders::HttpEndpointDestinationDescriptionBuilder {
        crate::types::builders::HttpEndpointDestinationDescriptionBuilder::default()
    }
}

/// A builder for [`HttpEndpointDestinationDescription`](crate::types::HttpEndpointDestinationDescription).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct HttpEndpointDestinationDescriptionBuilder {
    pub(crate) endpoint_configuration: ::std::option::Option<crate::types::HttpEndpointDescription>,
    pub(crate) buffering_hints: ::std::option::Option<crate::types::HttpEndpointBufferingHints>,
    pub(crate) cloud_watch_logging_options: ::std::option::Option<crate::types::CloudWatchLoggingOptions>,
    pub(crate) request_configuration: ::std::option::Option<crate::types::HttpEndpointRequestConfiguration>,
    pub(crate) processing_configuration: ::std::option::Option<crate::types::ProcessingConfiguration>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) retry_options: ::std::option::Option<crate::types::HttpEndpointRetryOptions>,
    pub(crate) s3_backup_mode: ::std::option::Option<crate::types::HttpEndpointS3BackupMode>,
    pub(crate) s3_destination_description: ::std::option::Option<crate::types::S3DestinationDescription>,
    pub(crate) secrets_manager_configuration: ::std::option::Option<crate::types::SecretsManagerConfiguration>,
}
impl HttpEndpointDestinationDescriptionBuilder {
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    pub fn endpoint_configuration(mut self, input: crate::types::HttpEndpointDescription) -> Self {
        self.endpoint_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    pub fn set_endpoint_configuration(mut self, input: ::std::option::Option<crate::types::HttpEndpointDescription>) -> Self {
        self.endpoint_configuration = input;
        self
    }
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    pub fn get_endpoint_configuration(&self) -> &::std::option::Option<crate::types::HttpEndpointDescription> {
        &self.endpoint_configuration
    }
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
    pub fn buffering_hints(mut self, input: crate::types::HttpEndpointBufferingHints) -> Self {
        self.buffering_hints = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
    pub fn set_buffering_hints(mut self, input: ::std::option::Option<crate::types::HttpEndpointBufferingHints>) -> Self {
        self.buffering_hints = input;
        self
    }
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
    pub fn get_buffering_hints(&self) -> &::std::option::Option<crate::types::HttpEndpointBufferingHints> {
        &self.buffering_hints
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
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    pub fn request_configuration(mut self, input: crate::types::HttpEndpointRequestConfiguration) -> Self {
        self.request_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    pub fn set_request_configuration(mut self, input: ::std::option::Option<crate::types::HttpEndpointRequestConfiguration>) -> Self {
        self.request_configuration = input;
        self
    }
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    pub fn get_request_configuration(&self) -> &::std::option::Option<crate::types::HttpEndpointRequestConfiguration> {
        &self.request_configuration
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
    /// <p>Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>Describes the retry behavior in case Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    pub fn retry_options(mut self, input: crate::types::HttpEndpointRetryOptions) -> Self {
        self.retry_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the retry behavior in case Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    pub fn set_retry_options(mut self, input: ::std::option::Option<crate::types::HttpEndpointRetryOptions>) -> Self {
        self.retry_options = input;
        self
    }
    /// <p>Describes the retry behavior in case Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    pub fn get_retry_options(&self) -> &::std::option::Option<crate::types::HttpEndpointRetryOptions> {
        &self.retry_options
    }
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    pub fn s3_backup_mode(mut self, input: crate::types::HttpEndpointS3BackupMode) -> Self {
        self.s3_backup_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    pub fn set_s3_backup_mode(mut self, input: ::std::option::Option<crate::types::HttpEndpointS3BackupMode>) -> Self {
        self.s3_backup_mode = input;
        self
    }
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    pub fn get_s3_backup_mode(&self) -> &::std::option::Option<crate::types::HttpEndpointS3BackupMode> {
        &self.s3_backup_mode
    }
    /// <p>Describes a destination in Amazon S3.</p>
    pub fn s3_destination_description(mut self, input: crate::types::S3DestinationDescription) -> Self {
        self.s3_destination_description = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes a destination in Amazon S3.</p>
    pub fn set_s3_destination_description(mut self, input: ::std::option::Option<crate::types::S3DestinationDescription>) -> Self {
        self.s3_destination_description = input;
        self
    }
    /// <p>Describes a destination in Amazon S3.</p>
    pub fn get_s3_destination_description(&self) -> &::std::option::Option<crate::types::S3DestinationDescription> {
        &self.s3_destination_description
    }
    /// <p>The configuration that defines how you access secrets for HTTP Endpoint destination.</p>
    pub fn secrets_manager_configuration(mut self, input: crate::types::SecretsManagerConfiguration) -> Self {
        self.secrets_manager_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration that defines how you access secrets for HTTP Endpoint destination.</p>
    pub fn set_secrets_manager_configuration(mut self, input: ::std::option::Option<crate::types::SecretsManagerConfiguration>) -> Self {
        self.secrets_manager_configuration = input;
        self
    }
    /// <p>The configuration that defines how you access secrets for HTTP Endpoint destination.</p>
    pub fn get_secrets_manager_configuration(&self) -> &::std::option::Option<crate::types::SecretsManagerConfiguration> {
        &self.secrets_manager_configuration
    }
    /// Consumes the builder and constructs a [`HttpEndpointDestinationDescription`](crate::types::HttpEndpointDestinationDescription).
    pub fn build(self) -> crate::types::HttpEndpointDestinationDescription {
        crate::types::HttpEndpointDestinationDescription {
            endpoint_configuration: self.endpoint_configuration,
            buffering_hints: self.buffering_hints,
            cloud_watch_logging_options: self.cloud_watch_logging_options,
            request_configuration: self.request_configuration,
            processing_configuration: self.processing_configuration,
            role_arn: self.role_arn,
            retry_options: self.retry_options,
            s3_backup_mode: self.s3_backup_mode,
            s3_destination_description: self.s3_destination_description,
            secrets_manager_configuration: self.secrets_manager_configuration,
        }
    }
}
