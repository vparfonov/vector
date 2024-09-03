// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateSmsSandboxPhoneNumberInput {
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub phone_number: ::std::option::Option<::std::string::String>,
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub language_code: ::std::option::Option<crate::types::LanguageCodeString>,
}
impl CreateSmsSandboxPhoneNumberInput {
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub fn phone_number(&self) -> ::std::option::Option<&str> {
        self.phone_number.as_deref()
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn language_code(&self) -> ::std::option::Option<&crate::types::LanguageCodeString> {
        self.language_code.as_ref()
    }
}
impl ::std::fmt::Debug for CreateSmsSandboxPhoneNumberInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateSmsSandboxPhoneNumberInput");
        formatter.field("phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("language_code", &self.language_code);
        formatter.finish()
    }
}
impl CreateSmsSandboxPhoneNumberInput {
    /// Creates a new builder-style object to manufacture [`CreateSmsSandboxPhoneNumberInput`](crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput).
    pub fn builder() -> crate::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder {
        crate::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder::default()
    }
}

/// A builder for [`CreateSmsSandboxPhoneNumberInput`](crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateSmsSandboxPhoneNumberInputBuilder {
    pub(crate) phone_number: ::std::option::Option<::std::string::String>,
    pub(crate) language_code: ::std::option::Option<crate::types::LanguageCodeString>,
}
impl CreateSmsSandboxPhoneNumberInputBuilder {
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    /// This field is required.
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phone_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phone_number = input;
        self
    }
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.phone_number
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn language_code(mut self, input: crate::types::LanguageCodeString) -> Self {
        self.language_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn set_language_code(mut self, input: ::std::option::Option<crate::types::LanguageCodeString>) -> Self {
        self.language_code = input;
        self
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn get_language_code(&self) -> &::std::option::Option<crate::types::LanguageCodeString> {
        &self.language_code
    }
    /// Consumes the builder and constructs a [`CreateSmsSandboxPhoneNumberInput`](crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput {
            phone_number: self.phone_number,
            language_code: self.language_code,
        })
    }
}
impl ::std::fmt::Debug for CreateSmsSandboxPhoneNumberInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateSmsSandboxPhoneNumberInputBuilder");
        formatter.field("phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("language_code", &self.language_code);
        formatter.finish()
    }
}
