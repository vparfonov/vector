// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_package_version_history::_get_package_version_history_output::GetPackageVersionHistoryOutputBuilder;

pub use crate::operation::get_package_version_history::_get_package_version_history_input::GetPackageVersionHistoryInputBuilder;

impl crate::operation::get_package_version_history::builders::GetPackageVersionHistoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_package_version_history::GetPackageVersionHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_package_version_history::GetPackageVersionHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_package_version_history();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPackageVersionHistory`.
///
/// <p>Returns a list of versions of the package, along with their creation time and commit message.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPackageVersionHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_package_version_history::builders::GetPackageVersionHistoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_package_version_history::GetPackageVersionHistoryOutput,
        crate::operation::get_package_version_history::GetPackageVersionHistoryError,
    > for GetPackageVersionHistoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_package_version_history::GetPackageVersionHistoryOutput,
            crate::operation::get_package_version_history::GetPackageVersionHistoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPackageVersionHistoryFluentBuilder {
    /// Creates a new `GetPackageVersionHistoryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPackageVersionHistory as a reference.
    pub fn as_input(&self) -> &crate::operation::get_package_version_history::builders::GetPackageVersionHistoryInputBuilder {
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
        crate::operation::get_package_version_history::GetPackageVersionHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_package_version_history::GetPackageVersionHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_package_version_history::GetPackageVersionHistory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_package_version_history::GetPackageVersionHistory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_package_version_history::GetPackageVersionHistoryOutput,
        crate::operation::get_package_version_history::GetPackageVersionHistoryError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_package_version_history::paginator::GetPackageVersionHistoryPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_package_version_history::paginator::GetPackageVersionHistoryPaginator {
        crate::operation::get_package_version_history::paginator::GetPackageVersionHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_id(input.into());
        self
    }
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_id(input);
        self
    }
    /// <p>Returns an audit history of versions of the package.</p>
    pub fn get_package_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_id()
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Limits results to a maximum number of versions.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
