// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The user-specified message attribute value. For string data types, the value attribute has the same restrictions on the content as the message body. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a>.</p>
/// <p>Name, type, and value must not be empty or null. In addition, the message body should not be empty or null. All parts of the message attribute, including name, type, and value, are included in the message size restriction, which is currently 256 KB (262,144 bytes). For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html">Amazon SNS message attributes</a> and <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Publishing to a mobile phone</a> in the <i>Amazon SNS Developer Guide.</i> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MessageAttributeValue {
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub data_type: ::std::string::String,
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub string_value: ::std::option::Option<::std::string::String>,
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub binary_value: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl MessageAttributeValue {
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub fn data_type(&self) -> &str {
        use std::ops::Deref;
        self.data_type.deref()
    }
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub fn string_value(&self) -> ::std::option::Option<&str> {
        self.string_value.as_deref()
    }
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub fn binary_value(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.binary_value.as_ref()
    }
}
impl MessageAttributeValue {
    /// Creates a new builder-style object to manufacture [`MessageAttributeValue`](crate::types::MessageAttributeValue).
    pub fn builder() -> crate::types::builders::MessageAttributeValueBuilder {
        crate::types::builders::MessageAttributeValueBuilder::default()
    }
}

/// A builder for [`MessageAttributeValue`](crate::types::MessageAttributeValue).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MessageAttributeValueBuilder {
    pub(crate) data_type: ::std::option::Option<::std::string::String>,
    pub(crate) string_value: ::std::option::Option<::std::string::String>,
    pub(crate) binary_value: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl MessageAttributeValueBuilder {
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    /// This field is required.
    pub fn data_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub fn set_data_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_type = input;
        self
    }
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub fn get_data_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_type
    }
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub fn string_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.string_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub fn set_string_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.string_value = input;
        self
    }
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub fn get_string_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.string_value
    }
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub fn binary_value(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.binary_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub fn set_binary_value(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.binary_value = input;
        self
    }
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub fn get_binary_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.binary_value
    }
    /// Consumes the builder and constructs a [`MessageAttributeValue`](crate::types::MessageAttributeValue).
    /// This method will fail if any of the following fields are not set:
    /// - [`data_type`](crate::types::builders::MessageAttributeValueBuilder::data_type)
    pub fn build(self) -> ::std::result::Result<crate::types::MessageAttributeValue, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::MessageAttributeValue {
            data_type: self.data_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data_type",
                    "data_type was not specified but it is required when building MessageAttributeValue",
                )
            })?,
            string_value: self.string_value,
            binary_value: self.binary_value,
        })
    }
}
