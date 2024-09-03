// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAccountPolicyOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteAccountPolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteAccountPolicyOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAccountPolicyOutput`](crate::operation::delete_account_policy::DeleteAccountPolicyOutput).
    pub fn builder() -> crate::operation::delete_account_policy::builders::DeleteAccountPolicyOutputBuilder {
        crate::operation::delete_account_policy::builders::DeleteAccountPolicyOutputBuilder::default()
    }
}

/// A builder for [`DeleteAccountPolicyOutput`](crate::operation::delete_account_policy::DeleteAccountPolicyOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteAccountPolicyOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteAccountPolicyOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAccountPolicyOutput`](crate::operation::delete_account_policy::DeleteAccountPolicyOutput).
    pub fn build(self) -> crate::operation::delete_account_policy::DeleteAccountPolicyOutput {
        crate::operation::delete_account_policy::DeleteAccountPolicyOutput {
            _request_id: self._request_id,
        }
    }
}
