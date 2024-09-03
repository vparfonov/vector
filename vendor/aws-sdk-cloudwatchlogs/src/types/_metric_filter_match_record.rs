// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a matched event.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricFilterMatchRecord {
    /// <p>The event number.</p>
    pub event_number: i64,
    /// <p>The raw event data.</p>
    pub event_message: ::std::option::Option<::std::string::String>,
    /// <p>The values extracted from the event data by the filter.</p>
    pub extracted_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl MetricFilterMatchRecord {
    /// <p>The event number.</p>
    pub fn event_number(&self) -> i64 {
        self.event_number
    }
    /// <p>The raw event data.</p>
    pub fn event_message(&self) -> ::std::option::Option<&str> {
        self.event_message.as_deref()
    }
    /// <p>The values extracted from the event data by the filter.</p>
    pub fn extracted_values(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.extracted_values.as_ref()
    }
}
impl MetricFilterMatchRecord {
    /// Creates a new builder-style object to manufacture [`MetricFilterMatchRecord`](crate::types::MetricFilterMatchRecord).
    pub fn builder() -> crate::types::builders::MetricFilterMatchRecordBuilder {
        crate::types::builders::MetricFilterMatchRecordBuilder::default()
    }
}

/// A builder for [`MetricFilterMatchRecord`](crate::types::MetricFilterMatchRecord).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct MetricFilterMatchRecordBuilder {
    pub(crate) event_number: ::std::option::Option<i64>,
    pub(crate) event_message: ::std::option::Option<::std::string::String>,
    pub(crate) extracted_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl MetricFilterMatchRecordBuilder {
    /// <p>The event number.</p>
    pub fn event_number(mut self, input: i64) -> Self {
        self.event_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The event number.</p>
    pub fn set_event_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.event_number = input;
        self
    }
    /// <p>The event number.</p>
    pub fn get_event_number(&self) -> &::std::option::Option<i64> {
        &self.event_number
    }
    /// <p>The raw event data.</p>
    pub fn event_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The raw event data.</p>
    pub fn set_event_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_message = input;
        self
    }
    /// <p>The raw event data.</p>
    pub fn get_event_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_message
    }
    /// Adds a key-value pair to `extracted_values`.
    ///
    /// To override the contents of this collection use [`set_extracted_values`](Self::set_extracted_values).
    ///
    /// <p>The values extracted from the event data by the filter.</p>
    pub fn extracted_values(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.extracted_values.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.extracted_values = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The values extracted from the event data by the filter.</p>
    pub fn set_extracted_values(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.extracted_values = input;
        self
    }
    /// <p>The values extracted from the event data by the filter.</p>
    pub fn get_extracted_values(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.extracted_values
    }
    /// Consumes the builder and constructs a [`MetricFilterMatchRecord`](crate::types::MetricFilterMatchRecord).
    pub fn build(self) -> crate::types::MetricFilterMatchRecord {
        crate::types::MetricFilterMatchRecord {
            event_number: self.event_number.unwrap_or_default(),
            event_message: self.event_message,
            extracted_values: self.extracted_values,
        }
    }
}
