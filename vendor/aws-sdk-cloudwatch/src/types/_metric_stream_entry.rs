// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure contains the configuration information about one metric stream.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricStreamEntry {
    /// <p>The ARN of the metric stream.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The date that the metric stream was originally created.</p>
    pub creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date that the configuration of this metric stream was most recently updated.</p>
    pub last_update_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The name of the metric stream.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the Kinesis Firehose devlivery stream that is used for this metric stream.</p>
    pub firehose_arn: ::std::option::Option<::std::string::String>,
    /// <p>The current state of this stream. Valid values are <code>running</code> and <code>stopped</code>.</p>
    pub state: ::std::option::Option<::std::string::String>,
    /// <p>The output format of this metric stream. Valid values are <code>json</code>, <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>.</p>
    pub output_format: ::std::option::Option<crate::types::MetricStreamOutputFormat>,
}
impl MetricStreamEntry {
    /// <p>The ARN of the metric stream.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The date that the metric stream was originally created.</p>
    pub fn creation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
    /// <p>The date that the configuration of this metric stream was most recently updated.</p>
    pub fn last_update_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_date.as_ref()
    }
    /// <p>The name of the metric stream.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ARN of the Kinesis Firehose devlivery stream that is used for this metric stream.</p>
    pub fn firehose_arn(&self) -> ::std::option::Option<&str> {
        self.firehose_arn.as_deref()
    }
    /// <p>The current state of this stream. Valid values are <code>running</code> and <code>stopped</code>.</p>
    pub fn state(&self) -> ::std::option::Option<&str> {
        self.state.as_deref()
    }
    /// <p>The output format of this metric stream. Valid values are <code>json</code>, <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>.</p>
    pub fn output_format(&self) -> ::std::option::Option<&crate::types::MetricStreamOutputFormat> {
        self.output_format.as_ref()
    }
}
impl MetricStreamEntry {
    /// Creates a new builder-style object to manufacture [`MetricStreamEntry`](crate::types::MetricStreamEntry).
    pub fn builder() -> crate::types::builders::MetricStreamEntryBuilder {
        crate::types::builders::MetricStreamEntryBuilder::default()
    }
}

/// A builder for [`MetricStreamEntry`](crate::types::MetricStreamEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct MetricStreamEntryBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_update_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) firehose_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<::std::string::String>,
    pub(crate) output_format: ::std::option::Option<crate::types::MetricStreamOutputFormat>,
}
impl MetricStreamEntryBuilder {
    /// <p>The ARN of the metric stream.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the metric stream.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the metric stream.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The date that the metric stream was originally created.</p>
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date that the metric stream was originally created.</p>
    pub fn set_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>The date that the metric stream was originally created.</p>
    pub fn get_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_date
    }
    /// <p>The date that the configuration of this metric stream was most recently updated.</p>
    pub fn last_update_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date that the configuration of this metric stream was most recently updated.</p>
    pub fn set_last_update_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_update_date = input;
        self
    }
    /// <p>The date that the configuration of this metric stream was most recently updated.</p>
    pub fn get_last_update_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_update_date
    }
    /// <p>The name of the metric stream.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the metric stream.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the metric stream.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The ARN of the Kinesis Firehose devlivery stream that is used for this metric stream.</p>
    pub fn firehose_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.firehose_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Kinesis Firehose devlivery stream that is used for this metric stream.</p>
    pub fn set_firehose_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.firehose_arn = input;
        self
    }
    /// <p>The ARN of the Kinesis Firehose devlivery stream that is used for this metric stream.</p>
    pub fn get_firehose_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.firehose_arn
    }
    /// <p>The current state of this stream. Valid values are <code>running</code> and <code>stopped</code>.</p>
    pub fn state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current state of this stream. Valid values are <code>running</code> and <code>stopped</code>.</p>
    pub fn set_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of this stream. Valid values are <code>running</code> and <code>stopped</code>.</p>
    pub fn get_state(&self) -> &::std::option::Option<::std::string::String> {
        &self.state
    }
    /// <p>The output format of this metric stream. Valid values are <code>json</code>, <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>.</p>
    pub fn output_format(mut self, input: crate::types::MetricStreamOutputFormat) -> Self {
        self.output_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The output format of this metric stream. Valid values are <code>json</code>, <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>.</p>
    pub fn set_output_format(mut self, input: ::std::option::Option<crate::types::MetricStreamOutputFormat>) -> Self {
        self.output_format = input;
        self
    }
    /// <p>The output format of this metric stream. Valid values are <code>json</code>, <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>.</p>
    pub fn get_output_format(&self) -> &::std::option::Option<crate::types::MetricStreamOutputFormat> {
        &self.output_format
    }
    /// Consumes the builder and constructs a [`MetricStreamEntry`](crate::types::MetricStreamEntry).
    pub fn build(self) -> crate::types::MetricStreamEntry {
        crate::types::MetricStreamEntry {
            arn: self.arn,
            creation_date: self.creation_date,
            last_update_date: self.last_update_date,
            name: self.name,
            firehose_arn: self.firehose_arn,
            state: self.state,
            output_format: self.output_format,
        }
    }
}
