// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents filters for a dimension.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DimensionFilter {
    /// <p>The dimension name to be matched.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The value of the dimension to be matched.</p>
    pub value: ::std::option::Option<::std::string::String>,
}
impl DimensionFilter {
    /// <p>The dimension name to be matched.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The value of the dimension to be matched.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl DimensionFilter {
    /// Creates a new builder-style object to manufacture [`DimensionFilter`](crate::types::DimensionFilter).
    pub fn builder() -> crate::types::builders::DimensionFilterBuilder {
        crate::types::builders::DimensionFilterBuilder::default()
    }
}

/// A builder for [`DimensionFilter`](crate::types::DimensionFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DimensionFilterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl DimensionFilterBuilder {
    /// <p>The dimension name to be matched.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The dimension name to be matched.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The dimension name to be matched.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The value of the dimension to be matched.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the dimension to be matched.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The value of the dimension to be matched.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`DimensionFilter`](crate::types::DimensionFilter).
    pub fn build(self) -> crate::types::DimensionFilter {
        crate::types::DimensionFilter {
            name: self.name,
            value: self.value,
        }
    }
}
