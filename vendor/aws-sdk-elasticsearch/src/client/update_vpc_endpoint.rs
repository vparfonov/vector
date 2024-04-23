// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVpcEndpoint`](crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpc_endpoint_id(impl Into<String>)`](crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder::vpc_endpoint_id) / [`set_vpc_endpoint_id(Option<String>)`](crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder::set_vpc_endpoint_id):<br>required: **true**<br><p>Unique identifier of the VPC endpoint to be updated.</p><br>
    ///   - [`vpc_options(VpcOptions)`](crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder::vpc_options) / [`set_vpc_options(Option<VpcOptions>)`](crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder::set_vpc_options):<br>required: **true**<br><p>The security groups and/or subnets to add, remove, or modify.</p><br>
    /// - On success, responds with [`UpdateVpcEndpointOutput`](crate::operation::update_vpc_endpoint::UpdateVpcEndpointOutput) with field(s):
    ///   - [`vpc_endpoint(Option<VpcEndpoint>)`](crate::operation::update_vpc_endpoint::UpdateVpcEndpointOutput::vpc_endpoint): <p>The endpoint to be updated.</p>
    /// - On failure, responds with [`SdkError<UpdateVpcEndpointError>`](crate::operation::update_vpc_endpoint::UpdateVpcEndpointError)
    pub fn update_vpc_endpoint(&self) -> crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder {
        crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointFluentBuilder::new(self.handle.clone())
    }
}
