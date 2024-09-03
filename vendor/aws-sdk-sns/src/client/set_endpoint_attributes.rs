// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetEndpointAttributes`](crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`endpoint_arn(impl Into<String>)`](crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder::set_endpoint_arn):<br>required: **true**<br><p>EndpointArn used for <code>SetEndpointAttributes</code> action.</p><br>
    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder::set_attributes):<br>required: **true**<br><p>A map of the endpoint attributes. Attributes in this map include the following:</p> <ul>  <li>   <p><code>CustomUserData</code> – arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p></li>  <li>   <p><code>Enabled</code> – flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p></li>  <li>   <p><code>Token</code> – device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p></li> </ul><br>
    /// - On success, responds with [`SetEndpointAttributesOutput`](crate::operation::set_endpoint_attributes::SetEndpointAttributesOutput)
    /// - On failure, responds with [`SdkError<SetEndpointAttributesError>`](crate::operation::set_endpoint_attributes::SetEndpointAttributesError)
    pub fn set_endpoint_attributes(&self) -> crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder {
        crate::operation::set_endpoint_attributes::builders::SetEndpointAttributesFluentBuilder::new(self.handle.clone())
    }
}
