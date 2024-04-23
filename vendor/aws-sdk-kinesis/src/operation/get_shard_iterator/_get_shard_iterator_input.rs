// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input for <code>GetShardIterator</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetShardIteratorInput {
    /// <p>The name of the Amazon Kinesis data stream.</p>
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    pub shard_id: ::std::option::Option<::std::string::String>,
    /// <p>Determines how the shard iterator is used to start reading data records from the shard.</p>
    /// <p>The following are the valid Amazon Kinesis shard iterator types:</p>
    /// <ul>
    /// <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li>
    /// <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li>
    /// <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li>
    /// </ul>
    pub shard_iterator_type: ::std::option::Option<crate::types::ShardIteratorType>,
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    pub starting_sequence_number: ::std::option::Option<::std::string::String>,
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ARN of the stream.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
}
impl GetShardIteratorInput {
    /// <p>The name of the Amazon Kinesis data stream.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    pub fn shard_id(&self) -> ::std::option::Option<&str> {
        self.shard_id.as_deref()
    }
    /// <p>Determines how the shard iterator is used to start reading data records from the shard.</p>
    /// <p>The following are the valid Amazon Kinesis shard iterator types:</p>
    /// <ul>
    /// <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li>
    /// <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li>
    /// <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li>
    /// </ul>
    pub fn shard_iterator_type(&self) -> ::std::option::Option<&crate::types::ShardIteratorType> {
        self.shard_iterator_type.as_ref()
    }
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    pub fn starting_sequence_number(&self) -> ::std::option::Option<&str> {
        self.starting_sequence_number.as_deref()
    }
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
}
impl GetShardIteratorInput {
    /// Creates a new builder-style object to manufacture [`GetShardIteratorInput`](crate::operation::get_shard_iterator::GetShardIteratorInput).
    pub fn builder() -> crate::operation::get_shard_iterator::builders::GetShardIteratorInputBuilder {
        crate::operation::get_shard_iterator::builders::GetShardIteratorInputBuilder::default()
    }
}

/// A builder for [`GetShardIteratorInput`](crate::operation::get_shard_iterator::GetShardIteratorInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetShardIteratorInputBuilder {
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) shard_id: ::std::option::Option<::std::string::String>,
    pub(crate) shard_iterator_type: ::std::option::Option<crate::types::ShardIteratorType>,
    pub(crate) starting_sequence_number: ::std::option::Option<::std::string::String>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
}
impl GetShardIteratorInputBuilder {
    /// <p>The name of the Amazon Kinesis data stream.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon Kinesis data stream.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// <p>The name of the Amazon Kinesis data stream.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_name
    }
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    /// This field is required.
    pub fn shard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.shard_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    pub fn set_shard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.shard_id = input;
        self
    }
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    pub fn get_shard_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.shard_id
    }
    /// <p>Determines how the shard iterator is used to start reading data records from the shard.</p>
    /// <p>The following are the valid Amazon Kinesis shard iterator types:</p>
    /// <ul>
    /// <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li>
    /// <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li>
    /// <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li>
    /// </ul>
    /// This field is required.
    pub fn shard_iterator_type(mut self, input: crate::types::ShardIteratorType) -> Self {
        self.shard_iterator_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines how the shard iterator is used to start reading data records from the shard.</p>
    /// <p>The following are the valid Amazon Kinesis shard iterator types:</p>
    /// <ul>
    /// <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li>
    /// <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li>
    /// <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li>
    /// </ul>
    pub fn set_shard_iterator_type(mut self, input: ::std::option::Option<crate::types::ShardIteratorType>) -> Self {
        self.shard_iterator_type = input;
        self
    }
    /// <p>Determines how the shard iterator is used to start reading data records from the shard.</p>
    /// <p>The following are the valid Amazon Kinesis shard iterator types:</p>
    /// <ul>
    /// <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li>
    /// <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li>
    /// <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li>
    /// <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li>
    /// </ul>
    pub fn get_shard_iterator_type(&self) -> &::std::option::Option<crate::types::ShardIteratorType> {
        &self.shard_iterator_type
    }
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    pub fn starting_sequence_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.starting_sequence_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    pub fn set_starting_sequence_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.starting_sequence_number = input;
        self
    }
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    pub fn get_starting_sequence_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.starting_sequence_number
    }
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// Consumes the builder and constructs a [`GetShardIteratorInput`](crate::operation::get_shard_iterator::GetShardIteratorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_shard_iterator::GetShardIteratorInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_shard_iterator::GetShardIteratorInput {
            stream_name: self.stream_name,
            shard_id: self.shard_id,
            shard_iterator_type: self.shard_iterator_type,
            starting_sequence_number: self.starting_sequence_number,
            timestamp: self.timestamp,
            stream_arn: self.stream_arn,
        })
    }
}
