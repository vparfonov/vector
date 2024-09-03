// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::check_if_phone_number_is_opted_out::_check_if_phone_number_is_opted_out_output::CheckIfPhoneNumberIsOptedOutOutputBuilder;

pub use crate::operation::check_if_phone_number_is_opted_out::_check_if_phone_number_is_opted_out_input::CheckIfPhoneNumberIsOptedOutInputBuilder;

impl crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.check_if_phone_number_is_opted_out();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CheckIfPhoneNumberIsOptedOut`.
///
/// <p>Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your Amazon Web Services account. You cannot send SMS messages to a number that is opted out.</p>
/// <p>To resume sending messages, you can opt in the number by using the <code>OptInPhoneNumber</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CheckIfPhoneNumberIsOptedOutFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput,
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
    > for CheckIfPhoneNumberIsOptedOutFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput,
            crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CheckIfPhoneNumberIsOptedOutFluentBuilder {
    /// Creates a new `CheckIfPhoneNumberIsOptedOutFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CheckIfPhoneNumberIsOptedOut as a reference.
    pub fn as_input(&self) -> &crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutInputBuilder {
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
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOut::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOut::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput,
        crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError,
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
    /// <p>The phone number for which you want to check the opt out status.</p>
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number(input.into());
        self
    }
    /// <p>The phone number for which you want to check the opt out status.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number(input);
        self
    }
    /// <p>The phone number for which you want to check the opt out status.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_phone_number()
    }
}
