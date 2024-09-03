// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_elasticsearch_service_role::_delete_elasticsearch_service_role_output::DeleteElasticsearchServiceRoleOutputBuilder;

pub use crate::operation::delete_elasticsearch_service_role::_delete_elasticsearch_service_role_input::DeleteElasticsearchServiceRoleInputBuilder;

impl crate::operation::delete_elasticsearch_service_role::builders::DeleteElasticsearchServiceRoleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_elasticsearch_service_role();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteElasticsearchServiceRole`.
///
/// <p>Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains before deleting the role. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-enabling-slr" target="_blank">Deleting Elasticsearch Service Role</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteElasticsearchServiceRoleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_elasticsearch_service_role::builders::DeleteElasticsearchServiceRoleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleOutput,
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleError,
    > for DeleteElasticsearchServiceRoleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleOutput,
            crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteElasticsearchServiceRoleFluentBuilder {
    /// Creates a new `DeleteElasticsearchServiceRoleFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteElasticsearchServiceRole as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_elasticsearch_service_role::builders::DeleteElasticsearchServiceRoleInputBuilder {
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
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRole::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRole::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleOutput,
        crate::operation::delete_elasticsearch_service_role::DeleteElasticsearchServiceRoleError,
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
}
