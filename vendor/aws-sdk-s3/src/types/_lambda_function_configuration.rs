// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for specifying the configuration for Lambda notifications.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LambdaFunctionConfiguration {
    /// <p>An optional unique identifier for configurations in a notification configuration. If you don't provide one, Amazon S3 will assign an ID.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
    pub lambda_function_arn: ::std::string::String,
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub events: ::std::vec::Vec<crate::types::Event>,
    /// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-filtering.html">Configuring event notifications using object key name filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub filter: ::std::option::Option<crate::types::NotificationConfigurationFilter>,
}
impl LambdaFunctionConfiguration {
    /// <p>An optional unique identifier for configurations in a notification configuration. If you don't provide one, Amazon S3 will assign an ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
    pub fn lambda_function_arn(&self) -> &str {
        use std::ops::Deref;
        self.lambda_function_arn.deref()
    }
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn events(&self) -> &[crate::types::Event] {
        use std::ops::Deref;
        self.events.deref()
    }
    /// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-filtering.html">Configuring event notifications using object key name filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::NotificationConfigurationFilter> {
        self.filter.as_ref()
    }
}
impl LambdaFunctionConfiguration {
    /// Creates a new builder-style object to manufacture [`LambdaFunctionConfiguration`](crate::types::LambdaFunctionConfiguration).
    pub fn builder() -> crate::types::builders::LambdaFunctionConfigurationBuilder {
        crate::types::builders::LambdaFunctionConfigurationBuilder::default()
    }
}

/// A builder for [`LambdaFunctionConfiguration`](crate::types::LambdaFunctionConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct LambdaFunctionConfigurationBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) lambda_function_arn: ::std::option::Option<::std::string::String>,
    pub(crate) events: ::std::option::Option<::std::vec::Vec<crate::types::Event>>,
    pub(crate) filter: ::std::option::Option<crate::types::NotificationConfigurationFilter>,
}
impl LambdaFunctionConfigurationBuilder {
    /// <p>An optional unique identifier for configurations in a notification configuration. If you don't provide one, Amazon S3 will assign an ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional unique identifier for configurations in a notification configuration. If you don't provide one, Amazon S3 will assign an ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>An optional unique identifier for configurations in a notification configuration. If you don't provide one, Amazon S3 will assign an ID.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
    /// This field is required.
    pub fn lambda_function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lambda_function_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
    pub fn set_lambda_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lambda_function_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
    pub fn get_lambda_function_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.lambda_function_arn
    }
    /// Appends an item to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    ///
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn events(mut self, input: crate::types::Event) -> Self {
        let mut v = self.events.unwrap_or_default();
        v.push(input);
        self.events = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_events(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Event>>) -> Self {
        self.events = input;
        self
    }
    /// <p>The Amazon S3 bucket event for which to invoke the Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_events(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Event>> {
        &self.events
    }
    /// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-filtering.html">Configuring event notifications using object key name filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn filter(mut self, input: crate::types::NotificationConfigurationFilter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-filtering.html">Configuring event notifications using object key name filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::NotificationConfigurationFilter>) -> Self {
        self.filter = input;
        self
    }
    /// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-filtering.html">Configuring event notifications using object key name filtering</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::NotificationConfigurationFilter> {
        &self.filter
    }
    /// Consumes the builder and constructs a [`LambdaFunctionConfiguration`](crate::types::LambdaFunctionConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`lambda_function_arn`](crate::types::builders::LambdaFunctionConfigurationBuilder::lambda_function_arn)
    /// - [`events`](crate::types::builders::LambdaFunctionConfigurationBuilder::events)
    pub fn build(self) -> ::std::result::Result<crate::types::LambdaFunctionConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::LambdaFunctionConfiguration {
            id: self.id,
            lambda_function_arn: self.lambda_function_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "lambda_function_arn",
                    "lambda_function_arn was not specified but it is required when building LambdaFunctionConfiguration",
                )
            })?,
            events: self.events.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "events",
                    "events was not specified but it is required when building LambdaFunctionConfiguration",
                )
            })?,
            filter: self.filter,
        })
    }
}
