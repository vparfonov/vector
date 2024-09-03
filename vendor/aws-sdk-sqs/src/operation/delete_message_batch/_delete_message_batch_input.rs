// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMessageBatchInput {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub queue_url: ::std::option::Option<::std::string::String>,
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub entries: ::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>>,
}
impl DeleteMessageBatchInput {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(&self) -> ::std::option::Option<&str> {
        self.queue_url.as_deref()
    }
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.entries.is_none()`.
    pub fn entries(&self) -> &[crate::types::DeleteMessageBatchRequestEntry] {
        self.entries.as_deref().unwrap_or_default()
    }
}
impl DeleteMessageBatchInput {
    /// Creates a new builder-style object to manufacture [`DeleteMessageBatchInput`](crate::operation::delete_message_batch::DeleteMessageBatchInput).
    pub fn builder() -> crate::operation::delete_message_batch::builders::DeleteMessageBatchInputBuilder {
        crate::operation::delete_message_batch::builders::DeleteMessageBatchInputBuilder::default()
    }
}

/// A builder for [`DeleteMessageBatchInput`](crate::operation::delete_message_batch::DeleteMessageBatchInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteMessageBatchInputBuilder {
    pub(crate) queue_url: ::std::option::Option<::std::string::String>,
    pub(crate) entries: ::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>>,
}
impl DeleteMessageBatchInputBuilder {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    /// This field is required.
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_url = input;
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.queue_url
    }
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn entries(mut self, input: crate::types::DeleteMessageBatchRequestEntry) -> Self {
        let mut v = self.entries.unwrap_or_default();
        v.push(input);
        self.entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>>) -> Self {
        self.entries = input;
        self
    }
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn get_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>> {
        &self.entries
    }
    /// Consumes the builder and constructs a [`DeleteMessageBatchInput`](crate::operation::delete_message_batch::DeleteMessageBatchInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_message_batch::DeleteMessageBatchInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_message_batch::DeleteMessageBatchInput {
            queue_url: self.queue_url,
            entries: self.entries,
        })
    }
}
