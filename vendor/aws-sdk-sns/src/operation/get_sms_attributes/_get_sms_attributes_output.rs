// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The response from the <code>GetSMSAttributes</code> request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSmsAttributesOutput {
    /// <p>The SMS attribute names and their values.</p>
    pub attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSmsAttributesOutput {
    /// <p>The SMS attribute names and their values.</p>
    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.attributes.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetSmsAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSmsAttributesOutput {
    /// Creates a new builder-style object to manufacture [`GetSmsAttributesOutput`](crate::operation::get_sms_attributes::GetSmsAttributesOutput).
    pub fn builder() -> crate::operation::get_sms_attributes::builders::GetSmsAttributesOutputBuilder {
        crate::operation::get_sms_attributes::builders::GetSmsAttributesOutputBuilder::default()
    }
}

/// A builder for [`GetSmsAttributesOutput`](crate::operation::get_sms_attributes::GetSmsAttributesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetSmsAttributesOutputBuilder {
    pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSmsAttributesOutputBuilder {
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>The SMS attribute names and their values.</p>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The SMS attribute names and their values.</p>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.attributes = input;
        self
    }
    /// <p>The SMS attribute names and their values.</p>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.attributes
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSmsAttributesOutput`](crate::operation::get_sms_attributes::GetSmsAttributesOutput).
    pub fn build(self) -> crate::operation::get_sms_attributes::GetSmsAttributesOutput {
        crate::operation::get_sms_attributes::GetSmsAttributesOutput {
            attributes: self.attributes,
            _request_id: self._request_id,
        }
    }
}
