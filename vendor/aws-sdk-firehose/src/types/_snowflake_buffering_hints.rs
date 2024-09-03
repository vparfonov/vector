// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the buffering to perform before delivering data to the Snowflake destination. If you do not specify any value, Firehose uses the default values.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SnowflakeBufferingHints {
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 1.</p>
    pub size_in_mbs: ::std::option::Option<i32>,
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 0.</p>
    pub interval_in_seconds: ::std::option::Option<i32>,
}
impl SnowflakeBufferingHints {
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 1.</p>
    pub fn size_in_mbs(&self) -> ::std::option::Option<i32> {
        self.size_in_mbs
    }
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 0.</p>
    pub fn interval_in_seconds(&self) -> ::std::option::Option<i32> {
        self.interval_in_seconds
    }
}
impl SnowflakeBufferingHints {
    /// Creates a new builder-style object to manufacture [`SnowflakeBufferingHints`](crate::types::SnowflakeBufferingHints).
    pub fn builder() -> crate::types::builders::SnowflakeBufferingHintsBuilder {
        crate::types::builders::SnowflakeBufferingHintsBuilder::default()
    }
}

/// A builder for [`SnowflakeBufferingHints`](crate::types::SnowflakeBufferingHints).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SnowflakeBufferingHintsBuilder {
    pub(crate) size_in_mbs: ::std::option::Option<i32>,
    pub(crate) interval_in_seconds: ::std::option::Option<i32>,
}
impl SnowflakeBufferingHintsBuilder {
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 1.</p>
    pub fn size_in_mbs(mut self, input: i32) -> Self {
        self.size_in_mbs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 1.</p>
    pub fn set_size_in_mbs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.size_in_mbs = input;
        self
    }
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 1.</p>
    pub fn get_size_in_mbs(&self) -> &::std::option::Option<i32> {
        &self.size_in_mbs
    }
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 0.</p>
    pub fn interval_in_seconds(mut self, input: i32) -> Self {
        self.interval_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 0.</p>
    pub fn set_interval_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.interval_in_seconds = input;
        self
    }
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 0.</p>
    pub fn get_interval_in_seconds(&self) -> &::std::option::Option<i32> {
        &self.interval_in_seconds
    }
    /// Consumes the builder and constructs a [`SnowflakeBufferingHints`](crate::types::SnowflakeBufferingHints).
    pub fn build(self) -> crate::types::SnowflakeBufferingHints {
        crate::types::SnowflakeBufferingHints {
            size_in_mbs: self.size_in_mbs,
            interval_in_seconds: self.interval_in_seconds,
        }
    }
}
