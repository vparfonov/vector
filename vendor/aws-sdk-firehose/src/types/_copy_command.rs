// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a <code>COPY</code> command for Amazon Redshift.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CopyCommand {
    /// <p>The name of the target table. The table must already exist in the database.</p>
    pub data_table_name: ::std::string::String,
    /// <p>A comma-separated list of column names.</p>
    pub data_table_columns: ::std::option::Option<::std::string::String>,
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p>
    /// <p><code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p>
    /// <p><code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p>
    /// <p><code>delimiter '|' escape</code> - the delimiter should be escaped.</p>
    /// <p><code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p>
    /// <p><code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p>
    /// <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    pub copy_options: ::std::option::Option<::std::string::String>,
}
impl CopyCommand {
    /// <p>The name of the target table. The table must already exist in the database.</p>
    pub fn data_table_name(&self) -> &str {
        use std::ops::Deref;
        self.data_table_name.deref()
    }
    /// <p>A comma-separated list of column names.</p>
    pub fn data_table_columns(&self) -> ::std::option::Option<&str> {
        self.data_table_columns.as_deref()
    }
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p>
    /// <p><code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p>
    /// <p><code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p>
    /// <p><code>delimiter '|' escape</code> - the delimiter should be escaped.</p>
    /// <p><code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p>
    /// <p><code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p>
    /// <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    pub fn copy_options(&self) -> ::std::option::Option<&str> {
        self.copy_options.as_deref()
    }
}
impl CopyCommand {
    /// Creates a new builder-style object to manufacture [`CopyCommand`](crate::types::CopyCommand).
    pub fn builder() -> crate::types::builders::CopyCommandBuilder {
        crate::types::builders::CopyCommandBuilder::default()
    }
}

/// A builder for [`CopyCommand`](crate::types::CopyCommand).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CopyCommandBuilder {
    pub(crate) data_table_name: ::std::option::Option<::std::string::String>,
    pub(crate) data_table_columns: ::std::option::Option<::std::string::String>,
    pub(crate) copy_options: ::std::option::Option<::std::string::String>,
}
impl CopyCommandBuilder {
    /// <p>The name of the target table. The table must already exist in the database.</p>
    /// This field is required.
    pub fn data_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the target table. The table must already exist in the database.</p>
    pub fn set_data_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_table_name = input;
        self
    }
    /// <p>The name of the target table. The table must already exist in the database.</p>
    pub fn get_data_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_table_name
    }
    /// <p>A comma-separated list of column names.</p>
    pub fn data_table_columns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_table_columns = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A comma-separated list of column names.</p>
    pub fn set_data_table_columns(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_table_columns = input;
        self
    }
    /// <p>A comma-separated list of column names.</p>
    pub fn get_data_table_columns(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_table_columns
    }
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p>
    /// <p><code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p>
    /// <p><code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p>
    /// <p><code>delimiter '|' escape</code> - the delimiter should be escaped.</p>
    /// <p><code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p>
    /// <p><code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p>
    /// <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    pub fn copy_options(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.copy_options = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p>
    /// <p><code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p>
    /// <p><code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p>
    /// <p><code>delimiter '|' escape</code> - the delimiter should be escaped.</p>
    /// <p><code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p>
    /// <p><code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p>
    /// <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    pub fn set_copy_options(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.copy_options = input;
        self
    }
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p>
    /// <p><code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p>
    /// <p><code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p>
    /// <p><code>delimiter '|' escape</code> - the delimiter should be escaped.</p>
    /// <p><code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p>
    /// <p><code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p>
    /// <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    pub fn get_copy_options(&self) -> &::std::option::Option<::std::string::String> {
        &self.copy_options
    }
    /// Consumes the builder and constructs a [`CopyCommand`](crate::types::CopyCommand).
    /// This method will fail if any of the following fields are not set:
    /// - [`data_table_name`](crate::types::builders::CopyCommandBuilder::data_table_name)
    pub fn build(self) -> ::std::result::Result<crate::types::CopyCommand, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CopyCommand {
            data_table_name: self.data_table_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data_table_name",
                    "data_table_name was not specified but it is required when building CopyCommand",
                )
            })?,
            data_table_columns: self.data_table_columns,
            copy_options: self.copy_options,
        })
    }
}
