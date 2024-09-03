// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::upgrade_elasticsearch_domain::_upgrade_elasticsearch_domain_output::UpgradeElasticsearchDomainOutputBuilder;

pub use crate::operation::upgrade_elasticsearch_domain::_upgrade_elasticsearch_domain_input::UpgradeElasticsearchDomainInputBuilder;

impl crate::operation::upgrade_elasticsearch_domain::builders::UpgradeElasticsearchDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.upgrade_elasticsearch_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpgradeElasticsearchDomain`.
///
/// <p>Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpgradeElasticsearchDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::upgrade_elasticsearch_domain::builders::UpgradeElasticsearchDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainOutput,
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainError,
    > for UpgradeElasticsearchDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainOutput,
            crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpgradeElasticsearchDomainFluentBuilder {
    /// Creates a new `UpgradeElasticsearchDomainFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpgradeElasticsearchDomain as a reference.
    pub fn as_input(&self) -> &crate::operation::upgrade_elasticsearch_domain::builders::UpgradeElasticsearchDomainInputBuilder {
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
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomain::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomain::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainOutput,
        crate::operation::upgrade_elasticsearch_domain::UpgradeElasticsearchDomainError,
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
    /// <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    pub fn target_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_version(input.into());
        self
    }
    /// <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    pub fn set_target_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_version(input);
        self
    }
    /// <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    pub fn get_target_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_version()
    }
    /// <p>This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade.</p>
    pub fn perform_check_only(mut self, input: bool) -> Self {
        self.inner = self.inner.perform_check_only(input);
        self
    }
    /// <p>This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade.</p>
    pub fn set_perform_check_only(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_perform_check_only(input);
        self
    }
    /// <p>This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade.</p>
    pub fn get_perform_check_only(&self) -> &::std::option::Option<bool> {
        self.inner.get_perform_check_only()
    }
}
