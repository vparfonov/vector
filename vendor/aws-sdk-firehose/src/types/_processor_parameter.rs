// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the processor parameter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProcessorParameter {
    /// <p>The name of the parameter. Currently the following default values are supported: 3 for <code>NumberOfRetries</code> and 60 for the <code>BufferIntervalInSeconds</code>. The <code>BufferSizeInMBs</code> ranges between 0.2 MB and up to 3MB. The default buffering hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is 256 KB.</p>
    pub parameter_name: crate::types::ProcessorParameterName,
    /// <p>The parameter value.</p>
    pub parameter_value: ::std::string::String,
}
impl ProcessorParameter {
    /// <p>The name of the parameter. Currently the following default values are supported: 3 for <code>NumberOfRetries</code> and 60 for the <code>BufferIntervalInSeconds</code>. The <code>BufferSizeInMBs</code> ranges between 0.2 MB and up to 3MB. The default buffering hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is 256 KB.</p>
    pub fn parameter_name(&self) -> &crate::types::ProcessorParameterName {
        &self.parameter_name
    }
    /// <p>The parameter value.</p>
    pub fn parameter_value(&self) -> &str {
        use std::ops::Deref;
        self.parameter_value.deref()
    }
}
impl ProcessorParameter {
    /// Creates a new builder-style object to manufacture [`ProcessorParameter`](crate::types::ProcessorParameter).
    pub fn builder() -> crate::types::builders::ProcessorParameterBuilder {
        crate::types::builders::ProcessorParameterBuilder::default()
    }
}

/// A builder for [`ProcessorParameter`](crate::types::ProcessorParameter).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProcessorParameterBuilder {
    pub(crate) parameter_name: ::std::option::Option<crate::types::ProcessorParameterName>,
    pub(crate) parameter_value: ::std::option::Option<::std::string::String>,
}
impl ProcessorParameterBuilder {
    /// <p>The name of the parameter. Currently the following default values are supported: 3 for <code>NumberOfRetries</code> and 60 for the <code>BufferIntervalInSeconds</code>. The <code>BufferSizeInMBs</code> ranges between 0.2 MB and up to 3MB. The default buffering hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is 256 KB.</p>
    /// This field is required.
    pub fn parameter_name(mut self, input: crate::types::ProcessorParameterName) -> Self {
        self.parameter_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the parameter. Currently the following default values are supported: 3 for <code>NumberOfRetries</code> and 60 for the <code>BufferIntervalInSeconds</code>. The <code>BufferSizeInMBs</code> ranges between 0.2 MB and up to 3MB. The default buffering hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is 256 KB.</p>
    pub fn set_parameter_name(mut self, input: ::std::option::Option<crate::types::ProcessorParameterName>) -> Self {
        self.parameter_name = input;
        self
    }
    /// <p>The name of the parameter. Currently the following default values are supported: 3 for <code>NumberOfRetries</code> and 60 for the <code>BufferIntervalInSeconds</code>. The <code>BufferSizeInMBs</code> ranges between 0.2 MB and up to 3MB. The default buffering hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is 256 KB.</p>
    pub fn get_parameter_name(&self) -> &::std::option::Option<crate::types::ProcessorParameterName> {
        &self.parameter_name
    }
    /// <p>The parameter value.</p>
    /// This field is required.
    pub fn parameter_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.parameter_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The parameter value.</p>
    pub fn set_parameter_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.parameter_value = input;
        self
    }
    /// <p>The parameter value.</p>
    pub fn get_parameter_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.parameter_value
    }
    /// Consumes the builder and constructs a [`ProcessorParameter`](crate::types::ProcessorParameter).
    /// This method will fail if any of the following fields are not set:
    /// - [`parameter_name`](crate::types::builders::ProcessorParameterBuilder::parameter_name)
    /// - [`parameter_value`](crate::types::builders::ProcessorParameterBuilder::parameter_value)
    pub fn build(self) -> ::std::result::Result<crate::types::ProcessorParameter, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ProcessorParameter {
            parameter_name: self.parameter_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "parameter_name",
                    "parameter_name was not specified but it is required when building ProcessorParameter",
                )
            })?,
            parameter_value: self.parameter_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "parameter_value",
                    "parameter_value was not specified but it is required when building ProcessorParameter",
                )
            })?,
        })
    }
}
