// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes how uncompressed comma-separated values (CSV)-formatted results are formatted.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CsvOutput {
    /// <p>Indicates whether to use quotation marks around output fields.</p>
    /// <ul>
    /// <li>
    /// <p><code>ALWAYS</code>: Always use quotation marks for output fields.</p></li>
    /// <li>
    /// <p><code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p></li>
    /// </ul>
    pub quote_fields: ::std::option::Option<crate::types::QuoteFields>,
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: ::std::option::Option<::std::string::String>,
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: ::std::option::Option<::std::string::String>,
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub field_delimiter: ::std::option::Option<::std::string::String>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub quote_character: ::std::option::Option<::std::string::String>,
}
impl CsvOutput {
    /// <p>Indicates whether to use quotation marks around output fields.</p>
    /// <ul>
    /// <li>
    /// <p><code>ALWAYS</code>: Always use quotation marks for output fields.</p></li>
    /// <li>
    /// <p><code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p></li>
    /// </ul>
    pub fn quote_fields(&self) -> ::std::option::Option<&crate::types::QuoteFields> {
        self.quote_fields.as_ref()
    }
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub fn quote_escape_character(&self) -> ::std::option::Option<&str> {
        self.quote_escape_character.as_deref()
    }
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn record_delimiter(&self) -> ::std::option::Option<&str> {
        self.record_delimiter.as_deref()
    }
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn field_delimiter(&self) -> ::std::option::Option<&str> {
        self.field_delimiter.as_deref()
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub fn quote_character(&self) -> ::std::option::Option<&str> {
        self.quote_character.as_deref()
    }
}
impl CsvOutput {
    /// Creates a new builder-style object to manufacture [`CsvOutput`](crate::types::CsvOutput).
    pub fn builder() -> crate::types::builders::CsvOutputBuilder {
        crate::types::builders::CsvOutputBuilder::default()
    }
}

/// A builder for [`CsvOutput`](crate::types::CsvOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CsvOutputBuilder {
    pub(crate) quote_fields: ::std::option::Option<crate::types::QuoteFields>,
    pub(crate) quote_escape_character: ::std::option::Option<::std::string::String>,
    pub(crate) record_delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) field_delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) quote_character: ::std::option::Option<::std::string::String>,
}
impl CsvOutputBuilder {
    /// <p>Indicates whether to use quotation marks around output fields.</p>
    /// <ul>
    /// <li>
    /// <p><code>ALWAYS</code>: Always use quotation marks for output fields.</p></li>
    /// <li>
    /// <p><code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p></li>
    /// </ul>
    pub fn quote_fields(mut self, input: crate::types::QuoteFields) -> Self {
        self.quote_fields = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to use quotation marks around output fields.</p>
    /// <ul>
    /// <li>
    /// <p><code>ALWAYS</code>: Always use quotation marks for output fields.</p></li>
    /// <li>
    /// <p><code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p></li>
    /// </ul>
    pub fn set_quote_fields(mut self, input: ::std::option::Option<crate::types::QuoteFields>) -> Self {
        self.quote_fields = input;
        self
    }
    /// <p>Indicates whether to use quotation marks around output fields.</p>
    /// <ul>
    /// <li>
    /// <p><code>ALWAYS</code>: Always use quotation marks for output fields.</p></li>
    /// <li>
    /// <p><code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p></li>
    /// </ul>
    pub fn get_quote_fields(&self) -> &::std::option::Option<crate::types::QuoteFields> {
        &self.quote_fields
    }
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub fn quote_escape_character(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quote_escape_character = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub fn set_quote_escape_character(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quote_escape_character = input;
        self
    }
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub fn get_quote_escape_character(&self) -> &::std::option::Option<::std::string::String> {
        &self.quote_escape_character
    }
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn record_delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.record_delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn set_record_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.record_delimiter = input;
        self
    }
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub fn get_record_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.record_delimiter
    }
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn field_delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.field_delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn set_field_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.field_delimiter = input;
        self
    }
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub fn get_field_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.field_delimiter
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub fn quote_character(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quote_character = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub fn set_quote_character(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quote_character = input;
        self
    }
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub fn get_quote_character(&self) -> &::std::option::Option<::std::string::String> {
        &self.quote_character
    }
    /// Consumes the builder and constructs a [`CsvOutput`](crate::types::CsvOutput).
    pub fn build(self) -> crate::types::CsvOutput {
        crate::types::CsvOutput {
            quote_fields: self.quote_fields,
            quote_escape_character: self.quote_escape_character,
            record_delimiter: self.record_delimiter,
            field_delimiter: self.field_delimiter,
            quote_character: self.quote_character,
        }
    }
}
