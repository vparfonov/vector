// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAccountPoliciesOutput {
    /// <p>An array of structures that contain information about the CloudWatch Logs account policies that match the specified filters.</p>
    pub account_policies: ::std::option::Option<::std::vec::Vec<crate::types::AccountPolicy>>,
    _request_id: Option<String>,
}
impl DescribeAccountPoliciesOutput {
    /// <p>An array of structures that contain information about the CloudWatch Logs account policies that match the specified filters.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.account_policies.is_none()`.
    pub fn account_policies(&self) -> &[crate::types::AccountPolicy] {
        self.account_policies.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeAccountPoliciesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAccountPoliciesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAccountPoliciesOutput`](crate::operation::describe_account_policies::DescribeAccountPoliciesOutput).
    pub fn builder() -> crate::operation::describe_account_policies::builders::DescribeAccountPoliciesOutputBuilder {
        crate::operation::describe_account_policies::builders::DescribeAccountPoliciesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAccountPoliciesOutput`](crate::operation::describe_account_policies::DescribeAccountPoliciesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeAccountPoliciesOutputBuilder {
    pub(crate) account_policies: ::std::option::Option<::std::vec::Vec<crate::types::AccountPolicy>>,
    _request_id: Option<String>,
}
impl DescribeAccountPoliciesOutputBuilder {
    /// Appends an item to `account_policies`.
    ///
    /// To override the contents of this collection use [`set_account_policies`](Self::set_account_policies).
    ///
    /// <p>An array of structures that contain information about the CloudWatch Logs account policies that match the specified filters.</p>
    pub fn account_policies(mut self, input: crate::types::AccountPolicy) -> Self {
        let mut v = self.account_policies.unwrap_or_default();
        v.push(input);
        self.account_policies = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of structures that contain information about the CloudWatch Logs account policies that match the specified filters.</p>
    pub fn set_account_policies(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AccountPolicy>>) -> Self {
        self.account_policies = input;
        self
    }
    /// <p>An array of structures that contain information about the CloudWatch Logs account policies that match the specified filters.</p>
    pub fn get_account_policies(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountPolicy>> {
        &self.account_policies
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAccountPoliciesOutput`](crate::operation::describe_account_policies::DescribeAccountPoliciesOutput).
    pub fn build(self) -> crate::operation::describe_account_policies::DescribeAccountPoliciesOutput {
        crate::operation::describe_account_policies::DescribeAccountPoliciesOutput {
            account_policies: self.account_policies,
            _request_id: self._request_id,
        }
    }
}
