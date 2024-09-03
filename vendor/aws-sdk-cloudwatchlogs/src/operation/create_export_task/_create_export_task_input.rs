// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateExportTaskInput {
    /// <p>The name of the export task.</p>
    pub task_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the log group.</p>
    pub log_group_name: ::std::option::Option<::std::string::String>,
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub log_stream_name_prefix: ::std::option::Option<::std::string::String>,
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub from: ::std::option::Option<i64>,
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub to: ::std::option::Option<i64>,
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub destination: ::std::option::Option<::std::string::String>,
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub destination_prefix: ::std::option::Option<::std::string::String>,
}
impl CreateExportTaskInput {
    /// <p>The name of the export task.</p>
    pub fn task_name(&self) -> ::std::option::Option<&str> {
        self.task_name.as_deref()
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(&self) -> ::std::option::Option<&str> {
        self.log_group_name.as_deref()
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn log_stream_name_prefix(&self) -> ::std::option::Option<&str> {
        self.log_stream_name_prefix.as_deref()
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn from(&self) -> ::std::option::Option<i64> {
        self.from
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn to(&self) -> ::std::option::Option<i64> {
        self.to
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn destination_prefix(&self) -> ::std::option::Option<&str> {
        self.destination_prefix.as_deref()
    }
}
impl CreateExportTaskInput {
    /// Creates a new builder-style object to manufacture [`CreateExportTaskInput`](crate::operation::create_export_task::CreateExportTaskInput).
    pub fn builder() -> crate::operation::create_export_task::builders::CreateExportTaskInputBuilder {
        crate::operation::create_export_task::builders::CreateExportTaskInputBuilder::default()
    }
}

/// A builder for [`CreateExportTaskInput`](crate::operation::create_export_task::CreateExportTaskInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateExportTaskInputBuilder {
    pub(crate) task_name: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) log_stream_name_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) from: ::std::option::Option<i64>,
    pub(crate) to: ::std::option::Option<i64>,
    pub(crate) destination: ::std::option::Option<::std::string::String>,
    pub(crate) destination_prefix: ::std::option::Option<::std::string::String>,
}
impl CreateExportTaskInputBuilder {
    /// <p>The name of the export task.</p>
    pub fn task_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the export task.</p>
    pub fn set_task_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task_name = input;
        self
    }
    /// <p>The name of the export task.</p>
    pub fn get_task_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.task_name
    }
    /// <p>The name of the log group.</p>
    /// This field is required.
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_name = input;
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_name
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn log_stream_name_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_stream_name_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn set_log_stream_name_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_stream_name_prefix = input;
        self
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn get_log_stream_name_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_stream_name_prefix
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    /// This field is required.
    pub fn from(mut self, input: i64) -> Self {
        self.from = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn set_from(mut self, input: ::std::option::Option<i64>) -> Self {
        self.from = input;
        self
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn get_from(&self) -> &::std::option::Option<i64> {
        &self.from
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    /// This field is required.
    pub fn to(mut self, input: i64) -> Self {
        self.to = ::std::option::Option::Some(input);
        self
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn set_to(mut self, input: ::std::option::Option<i64>) -> Self {
        self.to = input;
        self
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn get_to(&self) -> &::std::option::Option<i64> {
        &self.to
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    /// This field is required.
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn get_destination(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn destination_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn set_destination_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_prefix = input;
        self
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn get_destination_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_prefix
    }
    /// Consumes the builder and constructs a [`CreateExportTaskInput`](crate::operation::create_export_task::CreateExportTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_export_task::CreateExportTaskInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_export_task::CreateExportTaskInput {
            task_name: self.task_name,
            log_group_name: self.log_group_name,
            log_stream_name_prefix: self.log_stream_name_prefix,
            from: self.from,
            to: self.to,
            destination: self.destination,
            destination_prefix: self.destination_prefix,
        })
    }
}
