// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Encloses the <code>Id</code> of an entry in <code> <code>DeleteMessageBatch</code>.</code></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMessageBatchResultEntry {
    /// <p>Represents a successfully deleted message.</p>
    pub id: ::std::string::String,
}
impl DeleteMessageBatchResultEntry {
    /// <p>Represents a successfully deleted message.</p>
    pub fn id(&self) -> &str {
        use std::ops::Deref;
        self.id.deref()
    }
}
impl DeleteMessageBatchResultEntry {
    /// Creates a new builder-style object to manufacture [`DeleteMessageBatchResultEntry`](crate::types::DeleteMessageBatchResultEntry).
    pub fn builder() -> crate::types::builders::DeleteMessageBatchResultEntryBuilder {
        crate::types::builders::DeleteMessageBatchResultEntryBuilder::default()
    }
}

/// A builder for [`DeleteMessageBatchResultEntry`](crate::types::DeleteMessageBatchResultEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteMessageBatchResultEntryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl DeleteMessageBatchResultEntryBuilder {
    /// <p>Represents a successfully deleted message.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Represents a successfully deleted message.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Represents a successfully deleted message.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Consumes the builder and constructs a [`DeleteMessageBatchResultEntry`](crate::types::DeleteMessageBatchResultEntry).
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](crate::types::builders::DeleteMessageBatchResultEntryBuilder::id)
    pub fn build(self) -> ::std::result::Result<crate::types::DeleteMessageBatchResultEntry, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::DeleteMessageBatchResultEntry {
            id: self.id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "id",
                    "id was not specified but it is required when building DeleteMessageBatchResultEntry",
                )
            })?,
        })
    }
}
