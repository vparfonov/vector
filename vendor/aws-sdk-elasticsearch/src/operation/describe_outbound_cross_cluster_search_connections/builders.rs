// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_outbound_cross_cluster_search_connections::_describe_outbound_cross_cluster_search_connections_output::DescribeOutboundCrossClusterSearchConnectionsOutputBuilder;

pub use crate::operation::describe_outbound_cross_cluster_search_connections::_describe_outbound_cross_cluster_search_connections_input::DescribeOutboundCrossClusterSearchConnectionsInputBuilder;

impl crate::operation::describe_outbound_cross_cluster_search_connections::builders::DescribeOutboundCrossClusterSearchConnectionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_outbound_cross_cluster_search_connections();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeOutboundCrossClusterSearchConnections`.
///
/// <p>Lists all the outbound cross-cluster search connections for a source domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeOutboundCrossClusterSearchConnectionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_outbound_cross_cluster_search_connections::builders::DescribeOutboundCrossClusterSearchConnectionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
    > for DescribeOutboundCrossClusterSearchConnectionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
            crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeOutboundCrossClusterSearchConnectionsFluentBuilder {
    /// Creates a new `DescribeOutboundCrossClusterSearchConnectionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeOutboundCrossClusterSearchConnections as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_outbound_cross_cluster_search_connections::builders::DescribeOutboundCrossClusterSearchConnectionsInputBuilder
    {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnections::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnections::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
        crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_outbound_cross_cluster_search_connections::paginator::DescribeOutboundCrossClusterSearchConnectionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_outbound_cross_cluster_search_connections::paginator::DescribeOutboundCrossClusterSearchConnectionsPaginator {
        crate::operation::describe_outbound_cross_cluster_search_connections::paginator::DescribeOutboundCrossClusterSearchConnectionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A list of filters used to match properties for outbound cross-cluster search connection. Available <code><code>Filter</code></code> names for this operation are:</p>
    /// <ul>
    /// <li>cross-cluster-search-connection-id</li>
    /// <li>destination-domain-info.domain-name</li>
    /// <li>destination-domain-info.owner-id</li>
    /// <li>destination-domain-info.region</li>
    /// <li>source-domain-info.domain-name</li>
    /// </ul>
    /// <p></p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A list of filters used to match properties for outbound cross-cluster search connection. Available <code><code>Filter</code></code> names for this operation are:</p>
    /// <ul>
    /// <li>cross-cluster-search-connection-id</li>
    /// <li>destination-domain-info.domain-name</li>
    /// <li>destination-domain-info.owner-id</li>
    /// <li>destination-domain-info.region</li>
    /// <li>source-domain-info.domain-name</li>
    /// </ul>
    /// <p></p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A list of filters used to match properties for outbound cross-cluster search connection. Available <code><code>Filter</code></code> names for this operation are:</p>
    /// <ul>
    /// <li>cross-cluster-search-connection-id</li>
    /// <li>destination-domain-info.domain-name</li>
    /// <li>destination-domain-info.owner-id</li>
    /// <li>destination-domain-info.region</li>
    /// <li>source-domain-info.domain-name</li>
    /// </ul>
    /// <p></p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>NextToken is sent in case the earlier API call results contain the NextToken. It is used for pagination.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>NextToken is sent in case the earlier API call results contain the NextToken. It is used for pagination.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>NextToken is sent in case the earlier API call results contain the NextToken. It is used for pagination.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
