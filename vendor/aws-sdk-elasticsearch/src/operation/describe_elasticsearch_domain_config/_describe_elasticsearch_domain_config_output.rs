// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of a <code>DescribeElasticsearchDomainConfig</code> request. Contains the configuration information of the requested domain.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeElasticsearchDomainConfigOutput {
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    pub domain_config: ::std::option::Option<crate::types::ElasticsearchDomainConfig>,
    _request_id: Option<String>,
}
impl DescribeElasticsearchDomainConfigOutput {
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    pub fn domain_config(&self) -> ::std::option::Option<&crate::types::ElasticsearchDomainConfig> {
        self.domain_config.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeElasticsearchDomainConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeElasticsearchDomainConfigOutput {
    /// Creates a new builder-style object to manufacture [`DescribeElasticsearchDomainConfigOutput`](crate::operation::describe_elasticsearch_domain_config::DescribeElasticsearchDomainConfigOutput).
    pub fn builder() -> crate::operation::describe_elasticsearch_domain_config::builders::DescribeElasticsearchDomainConfigOutputBuilder {
        crate::operation::describe_elasticsearch_domain_config::builders::DescribeElasticsearchDomainConfigOutputBuilder::default()
    }
}

/// A builder for [`DescribeElasticsearchDomainConfigOutput`](crate::operation::describe_elasticsearch_domain_config::DescribeElasticsearchDomainConfigOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeElasticsearchDomainConfigOutputBuilder {
    pub(crate) domain_config: ::std::option::Option<crate::types::ElasticsearchDomainConfig>,
    _request_id: Option<String>,
}
impl DescribeElasticsearchDomainConfigOutputBuilder {
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    /// This field is required.
    pub fn domain_config(mut self, input: crate::types::ElasticsearchDomainConfig) -> Self {
        self.domain_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    pub fn set_domain_config(mut self, input: ::std::option::Option<crate::types::ElasticsearchDomainConfig>) -> Self {
        self.domain_config = input;
        self
    }
    /// <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p>
    pub fn get_domain_config(&self) -> &::std::option::Option<crate::types::ElasticsearchDomainConfig> {
        &self.domain_config
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeElasticsearchDomainConfigOutput`](crate::operation::describe_elasticsearch_domain_config::DescribeElasticsearchDomainConfigOutput).
    pub fn build(self) -> crate::operation::describe_elasticsearch_domain_config::DescribeElasticsearchDomainConfigOutput {
        crate::operation::describe_elasticsearch_domain_config::DescribeElasticsearchDomainConfigOutput {
            domain_config: self.domain_config,
            _request_id: self._request_id,
        }
    }
}
