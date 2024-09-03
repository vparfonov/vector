// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>One data point from the metric time series returned in a Contributor Insights rule report.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetInsightRuleReport.html">GetInsightRuleReport</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InsightRuleMetricDatapoint {
    /// <p>The timestamp of the data point.</p>
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The number of unique contributors who published data during this timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub unique_contributors: ::std::option::Option<f64>,
    /// <p>The maximum value provided by one contributor during this timestamp. Each timestamp is evaluated separately, so the identity of the max contributor could be different for each timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub max_contributor_value: ::std::option::Option<f64>,
    /// <p>The number of occurrences that matched the rule during this data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub sample_count: ::std::option::Option<f64>,
    /// <p>The average value from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub average: ::std::option::Option<f64>,
    /// <p>The sum of the values from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub sum: ::std::option::Option<f64>,
    /// <p>The minimum value from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub minimum: ::std::option::Option<f64>,
    /// <p>The maximum value from a single occurence from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub maximum: ::std::option::Option<f64>,
}
impl InsightRuleMetricDatapoint {
    /// <p>The timestamp of the data point.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The number of unique contributors who published data during this timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn unique_contributors(&self) -> ::std::option::Option<f64> {
        self.unique_contributors
    }
    /// <p>The maximum value provided by one contributor during this timestamp. Each timestamp is evaluated separately, so the identity of the max contributor could be different for each timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn max_contributor_value(&self) -> ::std::option::Option<f64> {
        self.max_contributor_value
    }
    /// <p>The number of occurrences that matched the rule during this data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn sample_count(&self) -> ::std::option::Option<f64> {
        self.sample_count
    }
    /// <p>The average value from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn average(&self) -> ::std::option::Option<f64> {
        self.average
    }
    /// <p>The sum of the values from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn sum(&self) -> ::std::option::Option<f64> {
        self.sum
    }
    /// <p>The minimum value from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn minimum(&self) -> ::std::option::Option<f64> {
        self.minimum
    }
    /// <p>The maximum value from a single occurence from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn maximum(&self) -> ::std::option::Option<f64> {
        self.maximum
    }
}
impl InsightRuleMetricDatapoint {
    /// Creates a new builder-style object to manufacture [`InsightRuleMetricDatapoint`](crate::types::InsightRuleMetricDatapoint).
    pub fn builder() -> crate::types::builders::InsightRuleMetricDatapointBuilder {
        crate::types::builders::InsightRuleMetricDatapointBuilder::default()
    }
}

/// A builder for [`InsightRuleMetricDatapoint`](crate::types::InsightRuleMetricDatapoint).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InsightRuleMetricDatapointBuilder {
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) unique_contributors: ::std::option::Option<f64>,
    pub(crate) max_contributor_value: ::std::option::Option<f64>,
    pub(crate) sample_count: ::std::option::Option<f64>,
    pub(crate) average: ::std::option::Option<f64>,
    pub(crate) sum: ::std::option::Option<f64>,
    pub(crate) minimum: ::std::option::Option<f64>,
    pub(crate) maximum: ::std::option::Option<f64>,
}
impl InsightRuleMetricDatapointBuilder {
    /// <p>The timestamp of the data point.</p>
    /// This field is required.
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of the data point.</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The timestamp of the data point.</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }
    /// <p>The number of unique contributors who published data during this timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn unique_contributors(mut self, input: f64) -> Self {
        self.unique_contributors = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of unique contributors who published data during this timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_unique_contributors(mut self, input: ::std::option::Option<f64>) -> Self {
        self.unique_contributors = input;
        self
    }
    /// <p>The number of unique contributors who published data during this timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_unique_contributors(&self) -> &::std::option::Option<f64> {
        &self.unique_contributors
    }
    /// <p>The maximum value provided by one contributor during this timestamp. Each timestamp is evaluated separately, so the identity of the max contributor could be different for each timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn max_contributor_value(mut self, input: f64) -> Self {
        self.max_contributor_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum value provided by one contributor during this timestamp. Each timestamp is evaluated separately, so the identity of the max contributor could be different for each timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_max_contributor_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.max_contributor_value = input;
        self
    }
    /// <p>The maximum value provided by one contributor during this timestamp. Each timestamp is evaluated separately, so the identity of the max contributor could be different for each timestamp.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_max_contributor_value(&self) -> &::std::option::Option<f64> {
        &self.max_contributor_value
    }
    /// <p>The number of occurrences that matched the rule during this data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn sample_count(mut self, input: f64) -> Self {
        self.sample_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of occurrences that matched the rule during this data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_sample_count(mut self, input: ::std::option::Option<f64>) -> Self {
        self.sample_count = input;
        self
    }
    /// <p>The number of occurrences that matched the rule during this data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_sample_count(&self) -> &::std::option::Option<f64> {
        &self.sample_count
    }
    /// <p>The average value from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn average(mut self, input: f64) -> Self {
        self.average = ::std::option::Option::Some(input);
        self
    }
    /// <p>The average value from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_average(mut self, input: ::std::option::Option<f64>) -> Self {
        self.average = input;
        self
    }
    /// <p>The average value from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_average(&self) -> &::std::option::Option<f64> {
        &self.average
    }
    /// <p>The sum of the values from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn sum(mut self, input: f64) -> Self {
        self.sum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sum of the values from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_sum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.sum = input;
        self
    }
    /// <p>The sum of the values from all contributors during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_sum(&self) -> &::std::option::Option<f64> {
        &self.sum
    }
    /// <p>The minimum value from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn minimum(mut self, input: f64) -> Self {
        self.minimum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum value from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_minimum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.minimum = input;
        self
    }
    /// <p>The minimum value from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_minimum(&self) -> &::std::option::Option<f64> {
        &self.minimum
    }
    /// <p>The maximum value from a single occurence from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn maximum(mut self, input: f64) -> Self {
        self.maximum = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum value from a single occurence from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn set_maximum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.maximum = input;
        self
    }
    /// <p>The maximum value from a single occurence from a single contributor during the time period represented by that data point.</p>
    /// <p>This statistic is returned only if you included it in the <code>Metrics</code> array in your request.</p>
    pub fn get_maximum(&self) -> &::std::option::Option<f64> {
        &self.maximum
    }
    /// Consumes the builder and constructs a [`InsightRuleMetricDatapoint`](crate::types::InsightRuleMetricDatapoint).
    pub fn build(self) -> crate::types::InsightRuleMetricDatapoint {
        crate::types::InsightRuleMetricDatapoint {
            timestamp: self.timestamp,
            unique_contributors: self.unique_contributors,
            max_contributor_value: self.max_contributor_value,
            sample_count: self.sample_count,
            average: self.average,
            sum: self.sum,
            minimum: self.minimum,
            maximum: self.maximum,
        }
    }
}
