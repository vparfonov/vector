// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_log_anomaly_detectors::_list_log_anomaly_detectors_output::ListLogAnomalyDetectorsOutputBuilder;

pub use crate::operation::list_log_anomaly_detectors::_list_log_anomaly_detectors_input::ListLogAnomalyDetectorsInputBuilder;

impl crate::operation::list_log_anomaly_detectors::builders::ListLogAnomalyDetectorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_log_anomaly_detectors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListLogAnomalyDetectors`.
///
/// <p>Retrieves a list of the log anomaly detectors in the account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListLogAnomalyDetectorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_log_anomaly_detectors::builders::ListLogAnomalyDetectorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsOutput,
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsError,
    > for ListLogAnomalyDetectorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsOutput,
            crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListLogAnomalyDetectorsFluentBuilder {
    /// Creates a new `ListLogAnomalyDetectorsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListLogAnomalyDetectors as a reference.
    pub fn as_input(&self) -> &crate::operation::list_log_anomaly_detectors::builders::ListLogAnomalyDetectorsInputBuilder {
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
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectors::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectors::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsOutput,
        crate::operation::list_log_anomaly_detectors::ListLogAnomalyDetectorsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_log_anomaly_detectors::paginator::ListLogAnomalyDetectorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_log_anomaly_detectors::paginator::ListLogAnomalyDetectorsPaginator {
        crate::operation::list_log_anomaly_detectors::paginator::ListLogAnomalyDetectorsPaginator::new(self.handle, self.inner)
    }
    /// <p>Use this to optionally filter the results to only include anomaly detectors that are associated with the specified log group.</p>
    pub fn filter_log_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_log_group_arn(input.into());
        self
    }
    /// <p>Use this to optionally filter the results to only include anomaly detectors that are associated with the specified log group.</p>
    pub fn set_filter_log_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_filter_log_group_arn(input);
        self
    }
    /// <p>Use this to optionally filter the results to only include anomaly detectors that are associated with the specified log group.</p>
    pub fn get_filter_log_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_filter_log_group_arn()
    }
    /// <p>The maximum number of items to return. If you don't specify a value, the default maximum value of 50 items is used.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items to return. If you don't specify a value, the default maximum value of 50 items is used.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of items to return. If you don't specify a value, the default maximum value of 50 items is used.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
