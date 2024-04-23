// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Encloses data related to a successful message in a batch request for topic.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PublishBatchResultEntry {
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>An identifier for the message.</p>
    pub message_id: ::std::option::Option<::std::string::String>,
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics.</p>
    /// <p>The large, non-consecutive number that Amazon SNS assigns to each message.</p>
    /// <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub sequence_number: ::std::option::Option<::std::string::String>,
}
impl PublishBatchResultEntry {
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>An identifier for the message.</p>
    pub fn message_id(&self) -> ::std::option::Option<&str> {
        self.message_id.as_deref()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics.</p>
    /// <p>The large, non-consecutive number that Amazon SNS assigns to each message.</p>
    /// <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub fn sequence_number(&self) -> ::std::option::Option<&str> {
        self.sequence_number.as_deref()
    }
}
impl PublishBatchResultEntry {
    /// Creates a new builder-style object to manufacture [`PublishBatchResultEntry`](crate::types::PublishBatchResultEntry).
    pub fn builder() -> crate::types::builders::PublishBatchResultEntryBuilder {
        crate::types::builders::PublishBatchResultEntryBuilder::default()
    }
}

/// A builder for [`PublishBatchResultEntry`](crate::types::PublishBatchResultEntry).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PublishBatchResultEntryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) message_id: ::std::option::Option<::std::string::String>,
    pub(crate) sequence_number: ::std::option::Option<::std::string::String>,
}
impl PublishBatchResultEntryBuilder {
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>An identifier for the message.</p>
    pub fn message_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier for the message.</p>
    pub fn set_message_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_id = input;
        self
    }
    /// <p>An identifier for the message.</p>
    pub fn get_message_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_id
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics.</p>
    /// <p>The large, non-consecutive number that Amazon SNS assigns to each message.</p>
    /// <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub fn sequence_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sequence_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics.</p>
    /// <p>The large, non-consecutive number that Amazon SNS assigns to each message.</p>
    /// <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub fn set_sequence_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sequence_number = input;
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) topics.</p>
    /// <p>The large, non-consecutive number that Amazon SNS assigns to each message.</p>
    /// <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub fn get_sequence_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.sequence_number
    }
    /// Consumes the builder and constructs a [`PublishBatchResultEntry`](crate::types::PublishBatchResultEntry).
    pub fn build(self) -> crate::types::PublishBatchResultEntry {
        crate::types::PublishBatchResultEntry {
            id: self.id,
            message_id: self.message_id,
            sequence_number: self.sequence_number,
        }
    }
}
