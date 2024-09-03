// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the deserializer you want to use to convert the format of the input data. This parameter is required if <code>Enabled</code> is set to true.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputFormatConfiguration {
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    pub deserializer: ::std::option::Option<crate::types::Deserializer>,
}
impl InputFormatConfiguration {
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    pub fn deserializer(&self) -> ::std::option::Option<&crate::types::Deserializer> {
        self.deserializer.as_ref()
    }
}
impl InputFormatConfiguration {
    /// Creates a new builder-style object to manufacture [`InputFormatConfiguration`](crate::types::InputFormatConfiguration).
    pub fn builder() -> crate::types::builders::InputFormatConfigurationBuilder {
        crate::types::builders::InputFormatConfigurationBuilder::default()
    }
}

/// A builder for [`InputFormatConfiguration`](crate::types::InputFormatConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InputFormatConfigurationBuilder {
    pub(crate) deserializer: ::std::option::Option<crate::types::Deserializer>,
}
impl InputFormatConfigurationBuilder {
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    pub fn deserializer(mut self, input: crate::types::Deserializer) -> Self {
        self.deserializer = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    pub fn set_deserializer(mut self, input: ::std::option::Option<crate::types::Deserializer>) -> Self {
        self.deserializer = input;
        self
    }
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    pub fn get_deserializer(&self) -> &::std::option::Option<crate::types::Deserializer> {
        &self.deserializer
    }
    /// Consumes the builder and constructs a [`InputFormatConfiguration`](crate::types::InputFormatConfiguration).
    pub fn build(self) -> crate::types::InputFormatConfiguration {
        crate::types::InputFormatConfiguration {
            deserializer: self.deserializer,
        }
    }
}
