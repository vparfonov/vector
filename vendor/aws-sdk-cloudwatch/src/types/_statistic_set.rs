// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a set of statistics that describes a specific metric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StatisticSet {
    /// <p>The number of samples used for the statistic set.</p>
    pub sample_count: ::std::option::Option<f64>,
    /// <p>The sum of values for the sample set.</p>
    pub sum: ::std::option::Option<f64>,
    /// <p>The minimum value of the sample set.</p>
    pub minimum: ::std::option::Option<f64>,
    /// <p>The maximum value of the sample set.</p>
    pub maximum: ::std::option::Option<f64>,
}
impl StatisticSet {
    /// <p>The number of samples used for the statistic set.</p>
    pub fn sample_count(&self) -> ::std::option::Option<f64> {
        self.sample_count
    }
    /// <p>The sum of values for the sample set.</p>
    pub fn sum(&self) -> ::std::option::Option<f64> {
        self.sum
    }
    /// <p>The minimum value of the sample set.</p>
    pub fn minimum(&self) -> ::std::option::Option<f64> {
        self.minimum
    }
    /// <p>The maximum value of the sample set.</p>
    pub fn maximum(&self) -> ::std::option::Option<f64> {
        self.maximum
    }
}
impl StatisticSet {
    /// Creates a new builder-style object to manufacture [`StatisticSet`](crate::types::StatisticSet).
    pub fn builder() -> crate::types::builders::StatisticSetBuilder {
        crate::types::builders::StatisticSetBuilder::default()
    }
}

/// A builder for [`StatisticSet`](crate::types::StatisticSet).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StatisticSetBuilder {
    pub(crate) sample_count: ::std::option::Option<f64>,
    pub(crate) sum: ::std::option::Option<f64>,
    pub(crate) minimum: ::std::option::Option<f64>,
    pub(crate) maximum: ::std::option::Option<f64>,
}
impl StatisticSetBuilder {
    /// <p>The number of samples used for the statistic set.</p>
    /// This field is required.
    pub fn sample_count(mut self, input: f64) -> Self {
        self.sample_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of samples used for the statistic set.</p>
    pub fn set_sample_count(mut self, input: ::std::option::Option<f64>) -> Self {
        self.sample_count = input;
        self
    }
    /// <p>The number of samples used for the statistic set.</p>
    pub fn get_sample_count(&self) -> &::std::option::Option<f64> {
        &self.sample_count
    }
    /// <p>The sum of values for the sample set.</p>
    /// This field is required.
    pub fn sum(mut self, input: f64) -> Self {
        self.sum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sum of values for the sample set.</p>
    pub fn set_sum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.sum = input;
        self
    }
    /// <p>The sum of values for the sample set.</p>
    pub fn get_sum(&self) -> &::std::option::Option<f64> {
        &self.sum
    }
    /// <p>The minimum value of the sample set.</p>
    /// This field is required.
    pub fn minimum(mut self, input: f64) -> Self {
        self.minimum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum value of the sample set.</p>
    pub fn set_minimum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.minimum = input;
        self
    }
    /// <p>The minimum value of the sample set.</p>
    pub fn get_minimum(&self) -> &::std::option::Option<f64> {
        &self.minimum
    }
    /// <p>The maximum value of the sample set.</p>
    /// This field is required.
    pub fn maximum(mut self, input: f64) -> Self {
        self.maximum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum value of the sample set.</p>
    pub fn set_maximum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.maximum = input;
        self
    }
    /// <p>The maximum value of the sample set.</p>
    pub fn get_maximum(&self) -> &::std::option::Option<f64> {
        &self.maximum
    }
    /// Consumes the builder and constructs a [`StatisticSet`](crate::types::StatisticSet).
    pub fn build(self) -> crate::types::StatisticSet {
        crate::types::StatisticSet {
            sample_count: self.sample_count,
            sum: self.sum,
            minimum: self.minimum,
            maximum: self.maximum,
        }
    }
}
