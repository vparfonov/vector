// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_rotate_secret::_cancel_rotate_secret_output::CancelRotateSecretOutputBuilder;

pub use crate::operation::cancel_rotate_secret::_cancel_rotate_secret_input::CancelRotateSecretInputBuilder;

impl crate::operation::cancel_rotate_secret::builders::CancelRotateSecretInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_rotate_secret::CancelRotateSecretOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_rotate_secret::CancelRotateSecretError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_rotate_secret();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelRotateSecret`.
///
/// <p>Turns off automatic rotation, and if a rotation is currently in progress, cancels the rotation.</p>
/// <p>If you cancel a rotation in progress, it can leave the <code>VersionStage</code> labels in an unexpected state. You might need to remove the staging label <code>AWSPENDING</code> from the partially created version. You also need to determine whether to roll back to the previous version of the secret by moving the staging label <code>AWSCURRENT</code> to the version that has <code>AWSPENDING</code>. To determine which version has a specific staging label, call <code>ListSecretVersionIds</code>. Then use <code>UpdateSecretVersionStage</code> to change staging labels. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
/// <p>To turn on automatic rotation again, call <code>RotateSecret</code>.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p><b>Required permissions: </b> <code>secretsmanager:CancelRotateSecret</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelRotateSecretFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_rotate_secret::builders::CancelRotateSecretInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_rotate_secret::CancelRotateSecretOutput,
        crate::operation::cancel_rotate_secret::CancelRotateSecretError,
    > for CancelRotateSecretFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_rotate_secret::CancelRotateSecretOutput,
            crate::operation::cancel_rotate_secret::CancelRotateSecretError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelRotateSecretFluentBuilder {
    /// Creates a new `CancelRotateSecretFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelRotateSecret as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_rotate_secret::builders::CancelRotateSecretInputBuilder {
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
        crate::operation::cancel_rotate_secret::CancelRotateSecretOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_rotate_secret::CancelRotateSecretError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_rotate_secret::CancelRotateSecret::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_rotate_secret::CancelRotateSecret::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_rotate_secret::CancelRotateSecretOutput,
        crate::operation::cancel_rotate_secret::CancelRotateSecretError,
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
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn get_secret_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_id()
    }
}
