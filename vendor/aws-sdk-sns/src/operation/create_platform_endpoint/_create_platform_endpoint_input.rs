// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for CreatePlatformEndpoint action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePlatformEndpointInput {
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub platform_application_arn: ::std::option::Option<::std::string::String>,
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p>
    pub token: ::std::option::Option<::std::string::String>,
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub custom_user_data: ::std::option::Option<::std::string::String>,
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreatePlatformEndpointInput {
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub fn platform_application_arn(&self) -> ::std::option::Option<&str> {
        self.platform_application_arn.as_deref()
    }
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p>
    pub fn token(&self) -> ::std::option::Option<&str> {
        self.token.as_deref()
    }
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub fn custom_user_data(&self) -> ::std::option::Option<&str> {
        self.custom_user_data.as_deref()
    }
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.attributes.as_ref()
    }
}
impl CreatePlatformEndpointInput {
    /// Creates a new builder-style object to manufacture [`CreatePlatformEndpointInput`](crate::operation::create_platform_endpoint::CreatePlatformEndpointInput).
    pub fn builder() -> crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointInputBuilder {
        crate::operation::create_platform_endpoint::builders::CreatePlatformEndpointInputBuilder::default()
    }
}

/// A builder for [`CreatePlatformEndpointInput`](crate::operation::create_platform_endpoint::CreatePlatformEndpointInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreatePlatformEndpointInputBuilder {
    pub(crate) platform_application_arn: ::std::option::Option<::std::string::String>,
    pub(crate) token: ::std::option::Option<::std::string::String>,
    pub(crate) custom_user_data: ::std::option::Option<::std::string::String>,
    pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreatePlatformEndpointInputBuilder {
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    /// This field is required.
    pub fn platform_application_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.platform_application_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub fn set_platform_application_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.platform_application_arn = input;
        self
    }
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub fn get_platform_application_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.platform_application_arn
    }
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p>
    /// This field is required.
    pub fn token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p>
    pub fn set_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token = input;
        self
    }
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token equivalent is called the registration ID.</p>
    pub fn get_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.token
    }
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub fn custom_user_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.custom_user_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub fn set_custom_user_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.custom_user_data = input;
        self
    }
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub fn get_custom_user_data(&self) -> &::std::option::Option<::std::string::String> {
        &self.custom_user_data
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.attributes = input;
        self
    }
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.attributes
    }
    /// Consumes the builder and constructs a [`CreatePlatformEndpointInput`](crate::operation::create_platform_endpoint::CreatePlatformEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_platform_endpoint::CreatePlatformEndpointInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_platform_endpoint::CreatePlatformEndpointInput {
            platform_application_arn: self.platform_application_arn,
            token: self.token,
            custom_user_data: self.custom_user_data,
            attributes: self.attributes,
        })
    }
}
