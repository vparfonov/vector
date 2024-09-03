// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the <code>ListPhoneNumbersOptedOut</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPhoneNumbersOptedOutInput {
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListPhoneNumbersOptedOutInput {
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListPhoneNumbersOptedOutInput {
    /// Creates a new builder-style object to manufacture [`ListPhoneNumbersOptedOutInput`](crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutInput).
    pub fn builder() -> crate::operation::list_phone_numbers_opted_out::builders::ListPhoneNumbersOptedOutInputBuilder {
        crate::operation::list_phone_numbers_opted_out::builders::ListPhoneNumbersOptedOutInputBuilder::default()
    }
}

/// A builder for [`ListPhoneNumbersOptedOutInput`](crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListPhoneNumbersOptedOutInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListPhoneNumbersOptedOutInputBuilder {
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`ListPhoneNumbersOptedOutInput`](crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutInput { next_token: self.next_token })
    }
}
