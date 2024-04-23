// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDataProtectionPolicyOutput {
    /// <p>The log group name or ARN that you specified in your request.</p>
    pub log_group_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The data protection policy document for this log group.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The date and time that this policy was most recently updated.</p>
    pub last_updated_time: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetDataProtectionPolicyOutput {
    /// <p>The log group name or ARN that you specified in your request.</p>
    pub fn log_group_identifier(&self) -> ::std::option::Option<&str> {
        self.log_group_identifier.as_deref()
    }
    /// <p>The data protection policy document for this log group.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The date and time that this policy was most recently updated.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<i64> {
        self.last_updated_time
    }
}
impl ::aws_types::request_id::RequestId for GetDataProtectionPolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDataProtectionPolicyOutput {
    /// Creates a new builder-style object to manufacture [`GetDataProtectionPolicyOutput`](crate::operation::get_data_protection_policy::GetDataProtectionPolicyOutput).
    pub fn builder() -> crate::operation::get_data_protection_policy::builders::GetDataProtectionPolicyOutputBuilder {
        crate::operation::get_data_protection_policy::builders::GetDataProtectionPolicyOutputBuilder::default()
    }
}

/// A builder for [`GetDataProtectionPolicyOutput`](crate::operation::get_data_protection_policy::GetDataProtectionPolicyOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetDataProtectionPolicyOutputBuilder {
    pub(crate) log_group_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) last_updated_time: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetDataProtectionPolicyOutputBuilder {
    /// <p>The log group name or ARN that you specified in your request.</p>
    pub fn log_group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The log group name or ARN that you specified in your request.</p>
    pub fn set_log_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_identifier = input;
        self
    }
    /// <p>The log group name or ARN that you specified in your request.</p>
    pub fn get_log_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_identifier
    }
    /// <p>The data protection policy document for this log group.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The data protection policy document for this log group.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>The data protection policy document for this log group.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// <p>The date and time that this policy was most recently updated.</p>
    pub fn last_updated_time(mut self, input: i64) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that this policy was most recently updated.</p>
    pub fn set_last_updated_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>The date and time that this policy was most recently updated.</p>
    pub fn get_last_updated_time(&self) -> &::std::option::Option<i64> {
        &self.last_updated_time
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetDataProtectionPolicyOutput`](crate::operation::get_data_protection_policy::GetDataProtectionPolicyOutput).
    pub fn build(self) -> crate::operation::get_data_protection_policy::GetDataProtectionPolicyOutput {
        crate::operation::get_data_protection_policy::GetDataProtectionPolicyOutput {
            log_group_identifier: self.log_group_identifier,
            policy_document: self.policy_document,
            last_updated_time: self.last_updated_time,
            _request_id: self._request_id,
        }
    }
}
