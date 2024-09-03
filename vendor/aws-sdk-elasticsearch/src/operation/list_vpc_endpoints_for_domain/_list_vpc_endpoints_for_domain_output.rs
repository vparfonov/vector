// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for response parameters to the <code><code>ListVpcEndpointsForDomain</code></code> operation. Returns a list containing summarized details of the VPC endpoints.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVpcEndpointsForDomainOutput {
    /// <p>Provides list of <code>VpcEndpointSummary</code> summarizing details of the VPC endpoints.</p>
    pub vpc_endpoint_summary_list: ::std::vec::Vec<crate::types::VpcEndpointSummary>,
    /// <p>Information about each endpoint associated with the domain.</p>
    pub next_token: ::std::string::String,
    _request_id: Option<String>,
}
impl ListVpcEndpointsForDomainOutput {
    /// <p>Provides list of <code>VpcEndpointSummary</code> summarizing details of the VPC endpoints.</p>
    pub fn vpc_endpoint_summary_list(&self) -> &[crate::types::VpcEndpointSummary] {
        use std::ops::Deref;
        self.vpc_endpoint_summary_list.deref()
    }
    /// <p>Information about each endpoint associated with the domain.</p>
    pub fn next_token(&self) -> &str {
        use std::ops::Deref;
        self.next_token.deref()
    }
}
impl ::aws_types::request_id::RequestId for ListVpcEndpointsForDomainOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListVpcEndpointsForDomainOutput {
    /// Creates a new builder-style object to manufacture [`ListVpcEndpointsForDomainOutput`](crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput).
    pub fn builder() -> crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder {
        crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder::default()
    }
}

/// A builder for [`ListVpcEndpointsForDomainOutput`](crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListVpcEndpointsForDomainOutputBuilder {
    pub(crate) vpc_endpoint_summary_list: ::std::option::Option<::std::vec::Vec<crate::types::VpcEndpointSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListVpcEndpointsForDomainOutputBuilder {
    /// Appends an item to `vpc_endpoint_summary_list`.
    ///
    /// To override the contents of this collection use [`set_vpc_endpoint_summary_list`](Self::set_vpc_endpoint_summary_list).
    ///
    /// <p>Provides list of <code>VpcEndpointSummary</code> summarizing details of the VPC endpoints.</p>
    pub fn vpc_endpoint_summary_list(mut self, input: crate::types::VpcEndpointSummary) -> Self {
        let mut v = self.vpc_endpoint_summary_list.unwrap_or_default();
        v.push(input);
        self.vpc_endpoint_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Provides list of <code>VpcEndpointSummary</code> summarizing details of the VPC endpoints.</p>
    pub fn set_vpc_endpoint_summary_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpcEndpointSummary>>) -> Self {
        self.vpc_endpoint_summary_list = input;
        self
    }
    /// <p>Provides list of <code>VpcEndpointSummary</code> summarizing details of the VPC endpoints.</p>
    pub fn get_vpc_endpoint_summary_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpcEndpointSummary>> {
        &self.vpc_endpoint_summary_list
    }
    /// <p>Information about each endpoint associated with the domain.</p>
    /// This field is required.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information about each endpoint associated with the domain.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Information about each endpoint associated with the domain.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListVpcEndpointsForDomainOutput`](crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`vpc_endpoint_summary_list`](crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder::vpc_endpoint_summary_list)
    /// - [`next_token`](crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder::next_token)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput {
            vpc_endpoint_summary_list: self.vpc_endpoint_summary_list.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "vpc_endpoint_summary_list",
                    "vpc_endpoint_summary_list was not specified but it is required when building ListVpcEndpointsForDomainOutput",
                )
            })?,
            next_token: self.next_token.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "next_token",
                    "next_token was not specified but it is required when building ListVpcEndpointsForDomainOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
