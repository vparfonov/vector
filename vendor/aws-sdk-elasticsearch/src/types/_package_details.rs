// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Basic information about a package.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PackageDetails {
    /// <p>Internal ID of the package.</p>
    pub package_id: ::std::option::Option<::std::string::String>,
    /// <p>User specified name of the package.</p>
    pub package_name: ::std::option::Option<::std::string::String>,
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    pub package_type: ::std::option::Option<crate::types::PackageType>,
    /// <p>User-specified description of the package.</p>
    pub package_description: ::std::option::Option<::std::string::String>,
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    pub package_status: ::std::option::Option<crate::types::PackageStatus>,
    /// <p>Timestamp which tells creation date of the package.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    #[allow(missing_docs)] // documentation missing in model
    pub last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    #[allow(missing_docs)] // documentation missing in model
    pub available_package_version: ::std::option::Option<::std::string::String>,
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    pub error_details: ::std::option::Option<crate::types::ErrorDetails>,
}
impl PackageDetails {
    /// <p>Internal ID of the package.</p>
    pub fn package_id(&self) -> ::std::option::Option<&str> {
        self.package_id.as_deref()
    }
    /// <p>User specified name of the package.</p>
    pub fn package_name(&self) -> ::std::option::Option<&str> {
        self.package_name.as_deref()
    }
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    pub fn package_type(&self) -> ::std::option::Option<&crate::types::PackageType> {
        self.package_type.as_ref()
    }
    /// <p>User-specified description of the package.</p>
    pub fn package_description(&self) -> ::std::option::Option<&str> {
        self.package_description.as_deref()
    }
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    pub fn package_status(&self) -> ::std::option::Option<&crate::types::PackageStatus> {
        self.package_status.as_ref()
    }
    /// <p>Timestamp which tells creation date of the package.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn last_updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn available_package_version(&self) -> ::std::option::Option<&str> {
        self.available_package_version.as_deref()
    }
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    pub fn error_details(&self) -> ::std::option::Option<&crate::types::ErrorDetails> {
        self.error_details.as_ref()
    }
}
impl PackageDetails {
    /// Creates a new builder-style object to manufacture [`PackageDetails`](crate::types::PackageDetails).
    pub fn builder() -> crate::types::builders::PackageDetailsBuilder {
        crate::types::builders::PackageDetailsBuilder::default()
    }
}

/// A builder for [`PackageDetails`](crate::types::PackageDetails).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PackageDetailsBuilder {
    pub(crate) package_id: ::std::option::Option<::std::string::String>,
    pub(crate) package_name: ::std::option::Option<::std::string::String>,
    pub(crate) package_type: ::std::option::Option<crate::types::PackageType>,
    pub(crate) package_description: ::std::option::Option<::std::string::String>,
    pub(crate) package_status: ::std::option::Option<crate::types::PackageStatus>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) available_package_version: ::std::option::Option<::std::string::String>,
    pub(crate) error_details: ::std::option::Option<crate::types::ErrorDetails>,
}
impl PackageDetailsBuilder {
    /// <p>Internal ID of the package.</p>
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Internal ID of the package.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_id = input;
        self
    }
    /// <p>Internal ID of the package.</p>
    pub fn get_package_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.package_id
    }
    /// <p>User specified name of the package.</p>
    pub fn package_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>User specified name of the package.</p>
    pub fn set_package_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_name = input;
        self
    }
    /// <p>User specified name of the package.</p>
    pub fn get_package_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.package_name
    }
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    pub fn package_type(mut self, input: crate::types::PackageType) -> Self {
        self.package_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    pub fn set_package_type(mut self, input: ::std::option::Option<crate::types::PackageType>) -> Self {
        self.package_type = input;
        self
    }
    /// <p>Currently supports only TXT-DICTIONARY.</p>
    pub fn get_package_type(&self) -> &::std::option::Option<crate::types::PackageType> {
        &self.package_type
    }
    /// <p>User-specified description of the package.</p>
    pub fn package_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>User-specified description of the package.</p>
    pub fn set_package_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_description = input;
        self
    }
    /// <p>User-specified description of the package.</p>
    pub fn get_package_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.package_description
    }
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    pub fn package_status(mut self, input: crate::types::PackageStatus) -> Self {
        self.package_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    pub fn set_package_status(mut self, input: ::std::option::Option<crate::types::PackageStatus>) -> Self {
        self.package_status = input;
        self
    }
    /// <p>Current state of the package. Values are COPYING/COPY_FAILED/AVAILABLE/DELETING/DELETE_FAILED</p>
    pub fn get_package_status(&self) -> &::std::option::Option<crate::types::PackageStatus> {
        &self.package_status
    }
    /// <p>Timestamp which tells creation date of the package.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Timestamp which tells creation date of the package.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>Timestamp which tells creation date of the package.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn last_updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_last_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_at = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_last_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_at
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn available_package_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.available_package_version = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_available_package_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.available_package_version = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_available_package_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.available_package_version
    }
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    pub fn error_details(mut self, input: crate::types::ErrorDetails) -> Self {
        self.error_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    pub fn set_error_details(mut self, input: ::std::option::Option<crate::types::ErrorDetails>) -> Self {
        self.error_details = input;
        self
    }
    /// <p>Additional information if the package is in an error state. Null otherwise.</p>
    pub fn get_error_details(&self) -> &::std::option::Option<crate::types::ErrorDetails> {
        &self.error_details
    }
    /// Consumes the builder and constructs a [`PackageDetails`](crate::types::PackageDetails).
    pub fn build(self) -> crate::types::PackageDetails {
        crate::types::PackageDetails {
            package_id: self.package_id,
            package_name: self.package_name,
            package_type: self.package_type,
            package_description: self.package_description,
            package_status: self.package_status,
            created_at: self.created_at,
            last_updated_at: self.last_updated_at,
            available_package_version: self.available_package_version,
            error_details: self.error_details,
        }
    }
}
