// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of a <code><code>DeleteOutboundCrossClusterSearchConnection</code></code> operation. Contains details of deleted outbound connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteOutboundCrossClusterSearchConnectionOutput {
    /// <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    pub cross_cluster_search_connection: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnection>,
    _request_id: Option<String>,
}
impl DeleteOutboundCrossClusterSearchConnectionOutput {
    /// <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    pub fn cross_cluster_search_connection(&self) -> ::std::option::Option<&crate::types::OutboundCrossClusterSearchConnection> {
        self.cross_cluster_search_connection.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteOutboundCrossClusterSearchConnectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteOutboundCrossClusterSearchConnectionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteOutboundCrossClusterSearchConnectionOutput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput).
    pub fn builder(
    ) -> crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionOutputBuilder {
        crate::operation::delete_outbound_cross_cluster_search_connection::builders::DeleteOutboundCrossClusterSearchConnectionOutputBuilder::default(
        )
    }
}

/// A builder for [`DeleteOutboundCrossClusterSearchConnectionOutput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteOutboundCrossClusterSearchConnectionOutputBuilder {
    pub(crate) cross_cluster_search_connection: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnection>,
    _request_id: Option<String>,
}
impl DeleteOutboundCrossClusterSearchConnectionOutputBuilder {
    /// <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    pub fn cross_cluster_search_connection(mut self, input: crate::types::OutboundCrossClusterSearchConnection) -> Self {
        self.cross_cluster_search_connection = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    pub fn set_cross_cluster_search_connection(mut self, input: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnection>) -> Self {
        self.cross_cluster_search_connection = input;
        self
    }
    /// <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
    pub fn get_cross_cluster_search_connection(&self) -> &::std::option::Option<crate::types::OutboundCrossClusterSearchConnection> {
        &self.cross_cluster_search_connection
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteOutboundCrossClusterSearchConnectionOutput`](crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput).
    pub fn build(self) -> crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput {
        crate::operation::delete_outbound_cross_cluster_search_connection::DeleteOutboundCrossClusterSearchConnectionOutput {
            cross_cluster_search_connection: self.cross_cluster_search_connection,
            _request_id: self._request_id,
        }
    }
}
