// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected for one of the following reasons:</p>
/// <ul>
/// <li>
/// <p>The KeyUsage value of the KMS key is incompatible with the API operation.</p></li>
/// <li>
/// <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the KMS key (KeySpec).</p></li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KmsInvalidKeyUsage {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl KmsInvalidKeyUsage {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for KmsInvalidKeyUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "KmsInvalidKeyUsage")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for KmsInvalidKeyUsage {}
impl ::aws_types::request_id::RequestId for crate::types::error::KmsInvalidKeyUsage {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for KmsInvalidKeyUsage {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl KmsInvalidKeyUsage {
    /// Creates a new builder-style object to manufacture [`KmsInvalidKeyUsage`](crate::types::error::KmsInvalidKeyUsage).
    pub fn builder() -> crate::types::error::builders::KmsInvalidKeyUsageBuilder {
        crate::types::error::builders::KmsInvalidKeyUsageBuilder::default()
    }
}

/// A builder for [`KmsInvalidKeyUsage`](crate::types::error::KmsInvalidKeyUsage).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct KmsInvalidKeyUsageBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl KmsInvalidKeyUsageBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`KmsInvalidKeyUsage`](crate::types::error::KmsInvalidKeyUsage).
    pub fn build(self) -> crate::types::error::KmsInvalidKeyUsage {
        crate::types::error::KmsInvalidKeyUsage {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
