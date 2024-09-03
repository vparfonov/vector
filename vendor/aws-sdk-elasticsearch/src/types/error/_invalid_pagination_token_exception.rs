// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request processing has failed because of invalid pagination token provided by customer. Returns an HTTP status code of 400.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvalidPaginationTokenException {
    /// <p>A description of the error.</p>
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl InvalidPaginationTokenException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for InvalidPaginationTokenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "InvalidPaginationTokenException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for InvalidPaginationTokenException {}
impl ::aws_types::request_id::RequestId for crate::types::error::InvalidPaginationTokenException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidPaginationTokenException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InvalidPaginationTokenException {
    /// Creates a new builder-style object to manufacture [`InvalidPaginationTokenException`](crate::types::error::InvalidPaginationTokenException).
    pub fn builder() -> crate::types::error::builders::InvalidPaginationTokenExceptionBuilder {
        crate::types::error::builders::InvalidPaginationTokenExceptionBuilder::default()
    }
}

/// A builder for [`InvalidPaginationTokenException`](crate::types::error::InvalidPaginationTokenException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InvalidPaginationTokenExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl InvalidPaginationTokenExceptionBuilder {
    /// <p>A description of the error.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the error.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>A description of the error.</p>
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
    /// Consumes the builder and constructs a [`InvalidPaginationTokenException`](crate::types::error::InvalidPaginationTokenException).
    pub fn build(self) -> crate::types::error::InvalidPaginationTokenException {
        crate::types::error::InvalidPaginationTokenException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
