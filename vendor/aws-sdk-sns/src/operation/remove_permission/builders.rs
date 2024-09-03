// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_permission::_remove_permission_output::RemovePermissionOutputBuilder;

pub use crate::operation::remove_permission::_remove_permission_input::RemovePermissionInputBuilder;

impl crate::operation::remove_permission::builders::RemovePermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_permission::RemovePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_permission::RemovePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemovePermission`.
///
/// <p>Removes a statement from a topic's access control policy.</p><note>
/// <p>To remove the ability to change topic permissions, you must deny permissions to the <code>AddPermission</code>, <code>RemovePermission</code>, and <code>SetTopicAttributes</code> actions in your IAM policy.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemovePermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_permission::builders::RemovePermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_permission::RemovePermissionOutput,
        crate::operation::remove_permission::RemovePermissionError,
    > for RemovePermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_permission::RemovePermissionOutput,
            crate::operation::remove_permission::RemovePermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemovePermissionFluentBuilder {
    /// Creates a new `RemovePermissionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemovePermission as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_permission::builders::RemovePermissionInputBuilder {
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
        crate::operation::remove_permission::RemovePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_permission::RemovePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_permission::RemovePermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_permission::RemovePermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_permission::RemovePermissionOutput,
        crate::operation::remove_permission::RemovePermissionError,
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
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_arn(input.into());
        self
    }
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_arn(input);
        self
    }
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_arn()
    }
    /// <p>The unique label of the statement you want to remove.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label(input.into());
        self
    }
    /// <p>The unique label of the statement you want to remove.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label(input);
        self
    }
    /// <p>The unique label of the statement you want to remove.</p>
    pub fn get_label(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_label()
    }
}
