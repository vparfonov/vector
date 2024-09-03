// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_random_password::_get_random_password_output::GetRandomPasswordOutputBuilder;

pub use crate::operation::get_random_password::_get_random_password_input::GetRandomPasswordInputBuilder;

impl crate::operation::get_random_password::builders::GetRandomPasswordInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_random_password::GetRandomPasswordOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_random_password::GetRandomPasswordError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_random_password();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRandomPassword`.
///
/// <p>Generates a random password. We recommend that you specify the maximum length and include every character type that the system you are generating a password for can support. By default, Secrets Manager uses uppercase and lowercase letters, numbers, and the following characters in passwords: <code>!\"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\\]^_`{|}~</code></p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action.</p>
/// <p><b>Required permissions: </b> <code>secretsmanager:GetRandomPassword</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRandomPasswordFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_random_password::builders::GetRandomPasswordInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_random_password::GetRandomPasswordOutput,
        crate::operation::get_random_password::GetRandomPasswordError,
    > for GetRandomPasswordFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_random_password::GetRandomPasswordOutput,
            crate::operation::get_random_password::GetRandomPasswordError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRandomPasswordFluentBuilder {
    /// Creates a new `GetRandomPasswordFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRandomPassword as a reference.
    pub fn as_input(&self) -> &crate::operation::get_random_password::builders::GetRandomPasswordInputBuilder {
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
        crate::operation::get_random_password::GetRandomPasswordOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_random_password::GetRandomPasswordError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_random_password::GetRandomPassword::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_random_password::GetRandomPassword::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_random_password::GetRandomPasswordOutput,
        crate::operation::get_random_password::GetRandomPasswordError,
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
    /// <p>The length of the password. If you don't include this parameter, the default length is 32 characters.</p>
    pub fn password_length(mut self, input: i64) -> Self {
        self.inner = self.inner.password_length(input);
        self
    }
    /// <p>The length of the password. If you don't include this parameter, the default length is 32 characters.</p>
    pub fn set_password_length(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_password_length(input);
        self
    }
    /// <p>The length of the password. If you don't include this parameter, the default length is 32 characters.</p>
    pub fn get_password_length(&self) -> &::std::option::Option<i64> {
        self.inner.get_password_length()
    }
    /// <p>A string of the characters that you don't want in the password.</p>
    pub fn exclude_characters(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.exclude_characters(input.into());
        self
    }
    /// <p>A string of the characters that you don't want in the password.</p>
    pub fn set_exclude_characters(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_exclude_characters(input);
        self
    }
    /// <p>A string of the characters that you don't want in the password.</p>
    pub fn get_exclude_characters(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_exclude_characters()
    }
    /// <p>Specifies whether to exclude numbers from the password. If you don't include this switch, the password can contain numbers.</p>
    pub fn exclude_numbers(mut self, input: bool) -> Self {
        self.inner = self.inner.exclude_numbers(input);
        self
    }
    /// <p>Specifies whether to exclude numbers from the password. If you don't include this switch, the password can contain numbers.</p>
    pub fn set_exclude_numbers(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_exclude_numbers(input);
        self
    }
    /// <p>Specifies whether to exclude numbers from the password. If you don't include this switch, the password can contain numbers.</p>
    pub fn get_exclude_numbers(&self) -> &::std::option::Option<bool> {
        self.inner.get_exclude_numbers()
    }
    /// <p>Specifies whether to exclude the following punctuation characters from the password: <code>! " # $ % &amp; ' ( ) * + , - . / : ; &lt; = &gt; ? @ \[ \ \] ^ _ ` { | } ~</code>. If you don't include this switch, the password can contain punctuation.</p>
    pub fn exclude_punctuation(mut self, input: bool) -> Self {
        self.inner = self.inner.exclude_punctuation(input);
        self
    }
    /// <p>Specifies whether to exclude the following punctuation characters from the password: <code>! " # $ % &amp; ' ( ) * + , - . / : ; &lt; = &gt; ? @ \[ \ \] ^ _ ` { | } ~</code>. If you don't include this switch, the password can contain punctuation.</p>
    pub fn set_exclude_punctuation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_exclude_punctuation(input);
        self
    }
    /// <p>Specifies whether to exclude the following punctuation characters from the password: <code>! " # $ % &amp; ' ( ) * + , - . / : ; &lt; = &gt; ? @ \[ \ \] ^ _ ` { | } ~</code>. If you don't include this switch, the password can contain punctuation.</p>
    pub fn get_exclude_punctuation(&self) -> &::std::option::Option<bool> {
        self.inner.get_exclude_punctuation()
    }
    /// <p>Specifies whether to exclude uppercase letters from the password. If you don't include this switch, the password can contain uppercase letters.</p>
    pub fn exclude_uppercase(mut self, input: bool) -> Self {
        self.inner = self.inner.exclude_uppercase(input);
        self
    }
    /// <p>Specifies whether to exclude uppercase letters from the password. If you don't include this switch, the password can contain uppercase letters.</p>
    pub fn set_exclude_uppercase(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_exclude_uppercase(input);
        self
    }
    /// <p>Specifies whether to exclude uppercase letters from the password. If you don't include this switch, the password can contain uppercase letters.</p>
    pub fn get_exclude_uppercase(&self) -> &::std::option::Option<bool> {
        self.inner.get_exclude_uppercase()
    }
    /// <p>Specifies whether to exclude lowercase letters from the password. If you don't include this switch, the password can contain lowercase letters.</p>
    pub fn exclude_lowercase(mut self, input: bool) -> Self {
        self.inner = self.inner.exclude_lowercase(input);
        self
    }
    /// <p>Specifies whether to exclude lowercase letters from the password. If you don't include this switch, the password can contain lowercase letters.</p>
    pub fn set_exclude_lowercase(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_exclude_lowercase(input);
        self
    }
    /// <p>Specifies whether to exclude lowercase letters from the password. If you don't include this switch, the password can contain lowercase letters.</p>
    pub fn get_exclude_lowercase(&self) -> &::std::option::Option<bool> {
        self.inner.get_exclude_lowercase()
    }
    /// <p>Specifies whether to include the space character. If you include this switch, the password can contain space characters.</p>
    pub fn include_space(mut self, input: bool) -> Self {
        self.inner = self.inner.include_space(input);
        self
    }
    /// <p>Specifies whether to include the space character. If you include this switch, the password can contain space characters.</p>
    pub fn set_include_space(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_space(input);
        self
    }
    /// <p>Specifies whether to include the space character. If you include this switch, the password can contain space characters.</p>
    pub fn get_include_space(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_space()
    }
    /// <p>Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation. If you don't include this switch, the password contains at least one of every character type.</p>
    pub fn require_each_included_type(mut self, input: bool) -> Self {
        self.inner = self.inner.require_each_included_type(input);
        self
    }
    /// <p>Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation. If you don't include this switch, the password contains at least one of every character type.</p>
    pub fn set_require_each_included_type(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_require_each_included_type(input);
        self
    }
    /// <p>Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation. If you don't include this switch, the password contains at least one of every character type.</p>
    pub fn get_require_each_included_type(&self) -> &::std::option::Option<bool> {
        self.inner.get_require_each_included_type()
    }
}
