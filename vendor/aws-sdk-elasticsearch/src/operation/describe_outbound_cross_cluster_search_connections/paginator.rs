// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeOutboundCrossClusterSearchConnections`](crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnections)
pub struct DescribeOutboundCrossClusterSearchConnectionsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder:
        crate::operation::describe_outbound_cross_cluster_search_connections::builders::DescribeOutboundCrossClusterSearchConnectionsInputBuilder,
    stop_on_duplicate_token: bool,
}

impl DescribeOutboundCrossClusterSearchConnectionsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::operation::describe_outbound_cross_cluster_search_connections::builders::DescribeOutboundCrossClusterSearchConnectionsInputBuilder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = ::std::option::Option::Some(limit);
        self
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used
    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).
    pub fn send(
        self,
    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<
        ::std::result::Result<
            crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsOutput,
            ::aws_smithy_runtime_api::client::result::SdkError<
                crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnectionsError,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >,
        >,
    > {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        let runtime_plugins = crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnections::operation_runtime_plugins(
                                handle.runtime_plugins.clone(),
                                &handle.conf,
                                ::std::option::Option::None,
                            ).with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
            move |tx| {
                ::std::boxed::Box::pin(async move {
                    // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                    let mut input = match builder
                        .build()
                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
                    {
                        ::std::result::Result::Ok(input) => input,
                        ::std::result::Result::Err(e) => {
                            let _ = tx.send(::std::result::Result::Err(e)).await;
                            return;
                        }
                    };
                    loop {
                        let resp = crate::operation::describe_outbound_cross_cluster_search_connections::DescribeOutboundCrossClusterSearchConnections::orchestrate(&runtime_plugins, input.clone()).await;
                        // If the input member is None or it was an error
                        let done = match resp {
                            ::std::result::Result::Ok(ref resp) => {
                                let new_token =
                                    crate::lens::reflens_describe_outbound_cross_cluster_search_connections_output_output_next_token(resp);
                                // Pagination is exhausted when the next token is an empty string
                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                    true
                                } else {
                                    input.next_token = new_token.cloned();
                                    is_empty
                                }
                            }
                            ::std::result::Result::Err(_) => true,
                        };
                        if tx.send(resp).await.is_err() {
                            // receiving end was dropped
                            return;
                        }
                        if done {
                            return;
                        }
                    }
                })
            },
        ))
    }
}
