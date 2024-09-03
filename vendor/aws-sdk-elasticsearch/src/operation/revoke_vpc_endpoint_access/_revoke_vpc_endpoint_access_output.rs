// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for response parameters to the <code><code>RevokeVpcEndpointAccess</code></code> operation. The response body for this operation is empty.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RevokeVpcEndpointAccessOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for RevokeVpcEndpointAccessOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RevokeVpcEndpointAccessOutput {
    /// Creates a new builder-style object to manufacture [`RevokeVpcEndpointAccessOutput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessOutput).
    pub fn builder() -> crate::operation::revoke_vpc_endpoint_access::builders::RevokeVpcEndpointAccessOutputBuilder {
        crate::operation::revoke_vpc_endpoint_access::builders::RevokeVpcEndpointAccessOutputBuilder::default()
    }
}

/// A builder for [`RevokeVpcEndpointAccessOutput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RevokeVpcEndpointAccessOutputBuilder {
    _request_id: Option<String>,
}
impl RevokeVpcEndpointAccessOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RevokeVpcEndpointAccessOutput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessOutput).
    pub fn build(self) -> crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessOutput {
        crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessOutput {
            _request_id: self._request_id,
        }
    }
}
