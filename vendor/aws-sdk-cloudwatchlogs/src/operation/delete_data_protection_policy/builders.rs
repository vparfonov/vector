// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_data_protection_policy::_delete_data_protection_policy_output::DeleteDataProtectionPolicyOutputBuilder;

pub use crate::operation::delete_data_protection_policy::_delete_data_protection_policy_input::DeleteDataProtectionPolicyInputBuilder;

impl crate::operation::delete_data_protection_policy::builders::DeleteDataProtectionPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_data_protection_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDataProtectionPolicy`.
///
/// <p>Deletes the data protection policy from the specified log group.</p>
/// <p>For more information about data protection policies, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDataProtectionPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_data_protection_policy::builders::DeleteDataProtectionPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyOutput,
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyError,
    > for DeleteDataProtectionPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyOutput,
            crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDataProtectionPolicyFluentBuilder {
    /// Creates a new `DeleteDataProtectionPolicyFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDataProtectionPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_data_protection_policy::builders::DeleteDataProtectionPolicyInputBuilder {
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
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyOutput,
        crate::operation::delete_data_protection_policy::DeleteDataProtectionPolicyError,
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
    /// <p>The name or ARN of the log group that you want to delete the data protection policy for.</p>
    pub fn log_group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_identifier(input.into());
        self
    }
    /// <p>The name or ARN of the log group that you want to delete the data protection policy for.</p>
    pub fn set_log_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_identifier(input);
        self
    }
    /// <p>The name or ARN of the log group that you want to delete the data protection policy for.</p>
    pub fn get_log_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_identifier()
    }
}
