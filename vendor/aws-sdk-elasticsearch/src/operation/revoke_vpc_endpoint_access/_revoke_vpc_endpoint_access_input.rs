// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Revokes access to an Amazon OpenSearch Service domain that was provided through an interface VPC endpoint.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RevokeVpcEndpointAccessInput {
    /// <p>The name of the OpenSearch Service domain.</p>
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>The account ID to revoke access from.</p>
    pub account: ::std::option::Option<::std::string::String>,
}
impl RevokeVpcEndpointAccessInput {
    /// <p>The name of the OpenSearch Service domain.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>The account ID to revoke access from.</p>
    pub fn account(&self) -> ::std::option::Option<&str> {
        self.account.as_deref()
    }
}
impl RevokeVpcEndpointAccessInput {
    /// Creates a new builder-style object to manufacture [`RevokeVpcEndpointAccessInput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessInput).
    pub fn builder() -> crate::operation::revoke_vpc_endpoint_access::builders::RevokeVpcEndpointAccessInputBuilder {
        crate::operation::revoke_vpc_endpoint_access::builders::RevokeVpcEndpointAccessInputBuilder::default()
    }
}

/// A builder for [`RevokeVpcEndpointAccessInput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RevokeVpcEndpointAccessInputBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) account: ::std::option::Option<::std::string::String>,
}
impl RevokeVpcEndpointAccessInputBuilder {
    /// <p>The name of the OpenSearch Service domain.</p>
    /// This field is required.
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the OpenSearch Service domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>The name of the OpenSearch Service domain.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_name
    }
    /// <p>The account ID to revoke access from.</p>
    /// This field is required.
    pub fn account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID to revoke access from.</p>
    pub fn set_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account = input;
        self
    }
    /// <p>The account ID to revoke access from.</p>
    pub fn get_account(&self) -> &::std::option::Option<::std::string::String> {
        &self.account
    }
    /// Consumes the builder and constructs a [`RevokeVpcEndpointAccessInput`](crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::revoke_vpc_endpoint_access::RevokeVpcEndpointAccessInput {
            domain_name: self.domain_name,
            account: self.account,
        })
    }
}
