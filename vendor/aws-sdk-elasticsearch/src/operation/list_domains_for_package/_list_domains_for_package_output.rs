// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for response parameters to <code> <code>ListDomainsForPackage</code> </code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDomainsForPackageOutput {
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    pub domain_package_details_list: ::std::option::Option<::std::vec::Vec<crate::types::DomainPackageDetails>>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDomainsForPackageOutput {
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.domain_package_details_list.is_none()`.
    pub fn domain_package_details_list(&self) -> &[crate::types::DomainPackageDetails] {
        self.domain_package_details_list.as_deref().unwrap_or_default()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListDomainsForPackageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListDomainsForPackageOutput {
    /// Creates a new builder-style object to manufacture [`ListDomainsForPackageOutput`](crate::operation::list_domains_for_package::ListDomainsForPackageOutput).
    pub fn builder() -> crate::operation::list_domains_for_package::builders::ListDomainsForPackageOutputBuilder {
        crate::operation::list_domains_for_package::builders::ListDomainsForPackageOutputBuilder::default()
    }
}

/// A builder for [`ListDomainsForPackageOutput`](crate::operation::list_domains_for_package::ListDomainsForPackageOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListDomainsForPackageOutputBuilder {
    pub(crate) domain_package_details_list: ::std::option::Option<::std::vec::Vec<crate::types::DomainPackageDetails>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDomainsForPackageOutputBuilder {
    /// Appends an item to `domain_package_details_list`.
    ///
    /// To override the contents of this collection use [`set_domain_package_details_list`](Self::set_domain_package_details_list).
    ///
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    pub fn domain_package_details_list(mut self, input: crate::types::DomainPackageDetails) -> Self {
        let mut v = self.domain_package_details_list.unwrap_or_default();
        v.push(input);
        self.domain_package_details_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    pub fn set_domain_package_details_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DomainPackageDetails>>) -> Self {
        self.domain_package_details_list = input;
        self
    }
    /// <p>List of <code>DomainPackageDetails</code> objects.</p>
    pub fn get_domain_package_details_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DomainPackageDetails>> {
        &self.domain_package_details_list
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListDomainsForPackageOutput`](crate::operation::list_domains_for_package::ListDomainsForPackageOutput).
    pub fn build(self) -> crate::operation::list_domains_for_package::ListDomainsForPackageOutput {
        crate::operation::list_domains_for_package::ListDomainsForPackageOutput {
            domain_package_details_list: self.domain_package_details_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
