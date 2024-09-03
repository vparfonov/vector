// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_domain_auto_tunes::_describe_domain_auto_tunes_output::DescribeDomainAutoTunesOutputBuilder;

pub use crate::operation::describe_domain_auto_tunes::_describe_domain_auto_tunes_input::DescribeDomainAutoTunesInputBuilder;

impl crate::operation::describe_domain_auto_tunes::builders::DescribeDomainAutoTunesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_domain_auto_tunes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeDomainAutoTunes`.
///
/// <p>Provides scheduled Auto-Tune action details for the Elasticsearch domain, such as Auto-Tune action type, description, severity, and scheduled date.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDomainAutoTunesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_domain_auto_tunes::builders::DescribeDomainAutoTunesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
    > for DescribeDomainAutoTunesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeDomainAutoTunesFluentBuilder {
    /// Creates a new `DescribeDomainAutoTunesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeDomainAutoTunes as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_domain_auto_tunes::builders::DescribeDomainAutoTunesInputBuilder {
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
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator {
        crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator::new(self.handle, self.inner)
    }
    /// <p>Specifies the domain name for which you want Auto-Tune action details.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>Specifies the domain name for which you want Auto-Tune action details.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>Specifies the domain name for which you want Auto-Tune action details.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
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
