// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure represents one anomaly that has been found by a logs anomaly detector.</p>
/// <p>For more information about patterns and anomalies, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_CreateLogAnomalyDetector.html">CreateLogAnomalyDetector</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Anomaly {
    /// <p>The unique ID that CloudWatch Logs assigned to this anomaly.</p>
    pub anomaly_id: ::std::string::String,
    /// <p>The ID of the pattern used to help identify this anomaly.</p>
    pub pattern_id: ::std::string::String,
    /// <p>The ARN of the anomaly detector that identified this anomaly.</p>
    pub anomaly_detector_arn: ::std::string::String,
    /// <p>The pattern used to help identify this anomaly, in string format.</p>
    pub pattern_string: ::std::string::String,
    /// <p>The pattern used to help identify this anomaly, in regular expression format.</p>
    pub pattern_regex: ::std::option::Option<::std::string::String>,
    /// <p>The priority level of this anomaly, as determined by CloudWatch Logs. Priority is computed based on log severity labels such as <code>FATAL</code> and <code>ERROR</code> and the amount of deviation from the baseline. Possible values are <code>HIGH</code>, <code>MEDIUM</code>, and <code>LOW</code>.</p>
    pub priority: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the anomaly detector first saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub first_seen: i64,
    /// <p>The date and time when the anomaly detector most recently saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub last_seen: i64,
    /// <p>A human-readable description of the anomaly. This description is generated by CloudWatch Logs.</p>
    pub description: ::std::string::String,
    /// <p>Specifies whether this anomaly is still ongoing.</p>
    pub active: bool,
    /// <p>Indicates the current state of this anomaly. If it is still being treated as an anomaly, the value is <code>Active</code>. If you have suppressed this anomaly by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a> operation, the value is <code>Suppressed</code>. If this behavior is now considered to be normal, the value is <code>Baseline</code>.</p>
    pub state: crate::types::State,
    /// <p>A map showing times when the anomaly detector ran, and the number of occurrences of this anomaly that were detected at each of those runs. The times are specified in epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub histogram: ::std::collections::HashMap<::std::string::String, i64>,
    /// <p>An array of sample log event messages that are considered to be part of this anomaly.</p>
    pub log_samples: ::std::vec::Vec<crate::types::LogEvent>,
    /// <p>An array of structures where each structure contains information about one token that makes up the pattern.</p>
    pub pattern_tokens: ::std::vec::Vec<crate::types::PatternToken>,
    /// <p>An array of ARNS of the log groups that contained log events considered to be part of this anomaly.</p>
    pub log_group_arn_list: ::std::vec::Vec<::std::string::String>,
    /// <p>Indicates whether this anomaly is currently suppressed. To suppress an anomaly, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a>.</p>
    pub suppressed: ::std::option::Option<bool>,
    /// <p>If the anomaly is suppressed, this indicates when it was suppressed.</p>
    pub suppressed_date: i64,
    /// <p>If the anomaly is suppressed, this indicates when the suppression will end. If this value is <code>0</code>, the anomaly was suppressed with no expiration, with the <code>INFINITE</code> value.</p>
    pub suppressed_until: i64,
    /// <p>If this anomaly is suppressed, this field is <code>true</code> if the suppression is because the pattern is suppressed. If <code>false</code>, then only this particular anomaly is suppressed.</p>
    pub is_pattern_level_suppression: ::std::option::Option<bool>,
}
impl Anomaly {
    /// <p>The unique ID that CloudWatch Logs assigned to this anomaly.</p>
    pub fn anomaly_id(&self) -> &str {
        use std::ops::Deref;
        self.anomaly_id.deref()
    }
    /// <p>The ID of the pattern used to help identify this anomaly.</p>
    pub fn pattern_id(&self) -> &str {
        use std::ops::Deref;
        self.pattern_id.deref()
    }
    /// <p>The ARN of the anomaly detector that identified this anomaly.</p>
    pub fn anomaly_detector_arn(&self) -> &str {
        use std::ops::Deref;
        self.anomaly_detector_arn.deref()
    }
    /// <p>The pattern used to help identify this anomaly, in string format.</p>
    pub fn pattern_string(&self) -> &str {
        use std::ops::Deref;
        self.pattern_string.deref()
    }
    /// <p>The pattern used to help identify this anomaly, in regular expression format.</p>
    pub fn pattern_regex(&self) -> ::std::option::Option<&str> {
        self.pattern_regex.as_deref()
    }
    /// <p>The priority level of this anomaly, as determined by CloudWatch Logs. Priority is computed based on log severity labels such as <code>FATAL</code> and <code>ERROR</code> and the amount of deviation from the baseline. Possible values are <code>HIGH</code>, <code>MEDIUM</code>, and <code>LOW</code>.</p>
    pub fn priority(&self) -> ::std::option::Option<&str> {
        self.priority.as_deref()
    }
    /// <p>The date and time when the anomaly detector first saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn first_seen(&self) -> i64 {
        self.first_seen
    }
    /// <p>The date and time when the anomaly detector most recently saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn last_seen(&self) -> i64 {
        self.last_seen
    }
    /// <p>A human-readable description of the anomaly. This description is generated by CloudWatch Logs.</p>
    pub fn description(&self) -> &str {
        use std::ops::Deref;
        self.description.deref()
    }
    /// <p>Specifies whether this anomaly is still ongoing.</p>
    pub fn active(&self) -> bool {
        self.active
    }
    /// <p>Indicates the current state of this anomaly. If it is still being treated as an anomaly, the value is <code>Active</code>. If you have suppressed this anomaly by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a> operation, the value is <code>Suppressed</code>. If this behavior is now considered to be normal, the value is <code>Baseline</code>.</p>
    pub fn state(&self) -> &crate::types::State {
        &self.state
    }
    /// <p>A map showing times when the anomaly detector ran, and the number of occurrences of this anomaly that were detected at each of those runs. The times are specified in epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn histogram(&self) -> &::std::collections::HashMap<::std::string::String, i64> {
        &self.histogram
    }
    /// <p>An array of sample log event messages that are considered to be part of this anomaly.</p>
    pub fn log_samples(&self) -> &[crate::types::LogEvent] {
        use std::ops::Deref;
        self.log_samples.deref()
    }
    /// <p>An array of structures where each structure contains information about one token that makes up the pattern.</p>
    pub fn pattern_tokens(&self) -> &[crate::types::PatternToken] {
        use std::ops::Deref;
        self.pattern_tokens.deref()
    }
    /// <p>An array of ARNS of the log groups that contained log events considered to be part of this anomaly.</p>
    pub fn log_group_arn_list(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.log_group_arn_list.deref()
    }
    /// <p>Indicates whether this anomaly is currently suppressed. To suppress an anomaly, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a>.</p>
    pub fn suppressed(&self) -> ::std::option::Option<bool> {
        self.suppressed
    }
    /// <p>If the anomaly is suppressed, this indicates when it was suppressed.</p>
    pub fn suppressed_date(&self) -> i64 {
        self.suppressed_date
    }
    /// <p>If the anomaly is suppressed, this indicates when the suppression will end. If this value is <code>0</code>, the anomaly was suppressed with no expiration, with the <code>INFINITE</code> value.</p>
    pub fn suppressed_until(&self) -> i64 {
        self.suppressed_until
    }
    /// <p>If this anomaly is suppressed, this field is <code>true</code> if the suppression is because the pattern is suppressed. If <code>false</code>, then only this particular anomaly is suppressed.</p>
    pub fn is_pattern_level_suppression(&self) -> ::std::option::Option<bool> {
        self.is_pattern_level_suppression
    }
}
impl Anomaly {
    /// Creates a new builder-style object to manufacture [`Anomaly`](crate::types::Anomaly).
    pub fn builder() -> crate::types::builders::AnomalyBuilder {
        crate::types::builders::AnomalyBuilder::default()
    }
}

/// A builder for [`Anomaly`](crate::types::Anomaly).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AnomalyBuilder {
    pub(crate) anomaly_id: ::std::option::Option<::std::string::String>,
    pub(crate) pattern_id: ::std::option::Option<::std::string::String>,
    pub(crate) anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    pub(crate) pattern_string: ::std::option::Option<::std::string::String>,
    pub(crate) pattern_regex: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<::std::string::String>,
    pub(crate) first_seen: ::std::option::Option<i64>,
    pub(crate) last_seen: ::std::option::Option<i64>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) active: ::std::option::Option<bool>,
    pub(crate) state: ::std::option::Option<crate::types::State>,
    pub(crate) histogram: ::std::option::Option<::std::collections::HashMap<::std::string::String, i64>>,
    pub(crate) log_samples: ::std::option::Option<::std::vec::Vec<crate::types::LogEvent>>,
    pub(crate) pattern_tokens: ::std::option::Option<::std::vec::Vec<crate::types::PatternToken>>,
    pub(crate) log_group_arn_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) suppressed: ::std::option::Option<bool>,
    pub(crate) suppressed_date: ::std::option::Option<i64>,
    pub(crate) suppressed_until: ::std::option::Option<i64>,
    pub(crate) is_pattern_level_suppression: ::std::option::Option<bool>,
}
impl AnomalyBuilder {
    /// <p>The unique ID that CloudWatch Logs assigned to this anomaly.</p>
    /// This field is required.
    pub fn anomaly_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.anomaly_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID that CloudWatch Logs assigned to this anomaly.</p>
    pub fn set_anomaly_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.anomaly_id = input;
        self
    }
    /// <p>The unique ID that CloudWatch Logs assigned to this anomaly.</p>
    pub fn get_anomaly_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.anomaly_id
    }
    /// <p>The ID of the pattern used to help identify this anomaly.</p>
    /// This field is required.
    pub fn pattern_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pattern_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the pattern used to help identify this anomaly.</p>
    pub fn set_pattern_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pattern_id = input;
        self
    }
    /// <p>The ID of the pattern used to help identify this anomaly.</p>
    pub fn get_pattern_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.pattern_id
    }
    /// <p>The ARN of the anomaly detector that identified this anomaly.</p>
    /// This field is required.
    pub fn anomaly_detector_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.anomaly_detector_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the anomaly detector that identified this anomaly.</p>
    pub fn set_anomaly_detector_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.anomaly_detector_arn = input;
        self
    }
    /// <p>The ARN of the anomaly detector that identified this anomaly.</p>
    pub fn get_anomaly_detector_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.anomaly_detector_arn
    }
    /// <p>The pattern used to help identify this anomaly, in string format.</p>
    /// This field is required.
    pub fn pattern_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pattern_string = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pattern used to help identify this anomaly, in string format.</p>
    pub fn set_pattern_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pattern_string = input;
        self
    }
    /// <p>The pattern used to help identify this anomaly, in string format.</p>
    pub fn get_pattern_string(&self) -> &::std::option::Option<::std::string::String> {
        &self.pattern_string
    }
    /// <p>The pattern used to help identify this anomaly, in regular expression format.</p>
    pub fn pattern_regex(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pattern_regex = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pattern used to help identify this anomaly, in regular expression format.</p>
    pub fn set_pattern_regex(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pattern_regex = input;
        self
    }
    /// <p>The pattern used to help identify this anomaly, in regular expression format.</p>
    pub fn get_pattern_regex(&self) -> &::std::option::Option<::std::string::String> {
        &self.pattern_regex
    }
    /// <p>The priority level of this anomaly, as determined by CloudWatch Logs. Priority is computed based on log severity labels such as <code>FATAL</code> and <code>ERROR</code> and the amount of deviation from the baseline. Possible values are <code>HIGH</code>, <code>MEDIUM</code>, and <code>LOW</code>.</p>
    pub fn priority(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.priority = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The priority level of this anomaly, as determined by CloudWatch Logs. Priority is computed based on log severity labels such as <code>FATAL</code> and <code>ERROR</code> and the amount of deviation from the baseline. Possible values are <code>HIGH</code>, <code>MEDIUM</code>, and <code>LOW</code>.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The priority level of this anomaly, as determined by CloudWatch Logs. Priority is computed based on log severity labels such as <code>FATAL</code> and <code>ERROR</code> and the amount of deviation from the baseline. Possible values are <code>HIGH</code>, <code>MEDIUM</code>, and <code>LOW</code>.</p>
    pub fn get_priority(&self) -> &::std::option::Option<::std::string::String> {
        &self.priority
    }
    /// <p>The date and time when the anomaly detector first saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    /// This field is required.
    pub fn first_seen(mut self, input: i64) -> Self {
        self.first_seen = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the anomaly detector first saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn set_first_seen(mut self, input: ::std::option::Option<i64>) -> Self {
        self.first_seen = input;
        self
    }
    /// <p>The date and time when the anomaly detector first saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn get_first_seen(&self) -> &::std::option::Option<i64> {
        &self.first_seen
    }
    /// <p>The date and time when the anomaly detector most recently saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    /// This field is required.
    pub fn last_seen(mut self, input: i64) -> Self {
        self.last_seen = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the anomaly detector most recently saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn set_last_seen(mut self, input: ::std::option::Option<i64>) -> Self {
        self.last_seen = input;
        self
    }
    /// <p>The date and time when the anomaly detector most recently saw this anomaly. It is specified as epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn get_last_seen(&self) -> &::std::option::Option<i64> {
        &self.last_seen
    }
    /// <p>A human-readable description of the anomaly. This description is generated by CloudWatch Logs.</p>
    /// This field is required.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A human-readable description of the anomaly. This description is generated by CloudWatch Logs.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A human-readable description of the anomaly. This description is generated by CloudWatch Logs.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>Specifies whether this anomaly is still ongoing.</p>
    /// This field is required.
    pub fn active(mut self, input: bool) -> Self {
        self.active = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether this anomaly is still ongoing.</p>
    pub fn set_active(mut self, input: ::std::option::Option<bool>) -> Self {
        self.active = input;
        self
    }
    /// <p>Specifies whether this anomaly is still ongoing.</p>
    pub fn get_active(&self) -> &::std::option::Option<bool> {
        &self.active
    }
    /// <p>Indicates the current state of this anomaly. If it is still being treated as an anomaly, the value is <code>Active</code>. If you have suppressed this anomaly by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a> operation, the value is <code>Suppressed</code>. If this behavior is now considered to be normal, the value is <code>Baseline</code>.</p>
    /// This field is required.
    pub fn state(mut self, input: crate::types::State) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the current state of this anomaly. If it is still being treated as an anomaly, the value is <code>Active</code>. If you have suppressed this anomaly by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a> operation, the value is <code>Suppressed</code>. If this behavior is now considered to be normal, the value is <code>Baseline</code>.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::State>) -> Self {
        self.state = input;
        self
    }
    /// <p>Indicates the current state of this anomaly. If it is still being treated as an anomaly, the value is <code>Active</code>. If you have suppressed this anomaly by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a> operation, the value is <code>Suppressed</code>. If this behavior is now considered to be normal, the value is <code>Baseline</code>.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::State> {
        &self.state
    }
    /// Adds a key-value pair to `histogram`.
    ///
    /// To override the contents of this collection use [`set_histogram`](Self::set_histogram).
    ///
    /// <p>A map showing times when the anomaly detector ran, and the number of occurrences of this anomaly that were detected at each of those runs. The times are specified in epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn histogram(mut self, k: impl ::std::convert::Into<::std::string::String>, v: i64) -> Self {
        let mut hash_map = self.histogram.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.histogram = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map showing times when the anomaly detector ran, and the number of occurrences of this anomaly that were detected at each of those runs. The times are specified in epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn set_histogram(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, i64>>) -> Self {
        self.histogram = input;
        self
    }
    /// <p>A map showing times when the anomaly detector ran, and the number of occurrences of this anomaly that were detected at each of those runs. The times are specified in epoch time, which is the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn get_histogram(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, i64>> {
        &self.histogram
    }
    /// Appends an item to `log_samples`.
    ///
    /// To override the contents of this collection use [`set_log_samples`](Self::set_log_samples).
    ///
    /// <p>An array of sample log event messages that are considered to be part of this anomaly.</p>
    pub fn log_samples(mut self, input: crate::types::LogEvent) -> Self {
        let mut v = self.log_samples.unwrap_or_default();
        v.push(input);
        self.log_samples = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of sample log event messages that are considered to be part of this anomaly.</p>
    pub fn set_log_samples(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LogEvent>>) -> Self {
        self.log_samples = input;
        self
    }
    /// <p>An array of sample log event messages that are considered to be part of this anomaly.</p>
    pub fn get_log_samples(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LogEvent>> {
        &self.log_samples
    }
    /// Appends an item to `pattern_tokens`.
    ///
    /// To override the contents of this collection use [`set_pattern_tokens`](Self::set_pattern_tokens).
    ///
    /// <p>An array of structures where each structure contains information about one token that makes up the pattern.</p>
    pub fn pattern_tokens(mut self, input: crate::types::PatternToken) -> Self {
        let mut v = self.pattern_tokens.unwrap_or_default();
        v.push(input);
        self.pattern_tokens = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of structures where each structure contains information about one token that makes up the pattern.</p>
    pub fn set_pattern_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PatternToken>>) -> Self {
        self.pattern_tokens = input;
        self
    }
    /// <p>An array of structures where each structure contains information about one token that makes up the pattern.</p>
    pub fn get_pattern_tokens(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PatternToken>> {
        &self.pattern_tokens
    }
    /// Appends an item to `log_group_arn_list`.
    ///
    /// To override the contents of this collection use [`set_log_group_arn_list`](Self::set_log_group_arn_list).
    ///
    /// <p>An array of ARNS of the log groups that contained log events considered to be part of this anomaly.</p>
    pub fn log_group_arn_list(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.log_group_arn_list.unwrap_or_default();
        v.push(input.into());
        self.log_group_arn_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of ARNS of the log groups that contained log events considered to be part of this anomaly.</p>
    pub fn set_log_group_arn_list(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.log_group_arn_list = input;
        self
    }
    /// <p>An array of ARNS of the log groups that contained log events considered to be part of this anomaly.</p>
    pub fn get_log_group_arn_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.log_group_arn_list
    }
    /// <p>Indicates whether this anomaly is currently suppressed. To suppress an anomaly, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a>.</p>
    pub fn suppressed(mut self, input: bool) -> Self {
        self.suppressed = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether this anomaly is currently suppressed. To suppress an anomaly, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a>.</p>
    pub fn set_suppressed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.suppressed = input;
        self
    }
    /// <p>Indicates whether this anomaly is currently suppressed. To suppress an anomaly, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateAnomaly.html">UpdateAnomaly</a>.</p>
    pub fn get_suppressed(&self) -> &::std::option::Option<bool> {
        &self.suppressed
    }
    /// <p>If the anomaly is suppressed, this indicates when it was suppressed.</p>
    pub fn suppressed_date(mut self, input: i64) -> Self {
        self.suppressed_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the anomaly is suppressed, this indicates when it was suppressed.</p>
    pub fn set_suppressed_date(mut self, input: ::std::option::Option<i64>) -> Self {
        self.suppressed_date = input;
        self
    }
    /// <p>If the anomaly is suppressed, this indicates when it was suppressed.</p>
    pub fn get_suppressed_date(&self) -> &::std::option::Option<i64> {
        &self.suppressed_date
    }
    /// <p>If the anomaly is suppressed, this indicates when the suppression will end. If this value is <code>0</code>, the anomaly was suppressed with no expiration, with the <code>INFINITE</code> value.</p>
    pub fn suppressed_until(mut self, input: i64) -> Self {
        self.suppressed_until = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the anomaly is suppressed, this indicates when the suppression will end. If this value is <code>0</code>, the anomaly was suppressed with no expiration, with the <code>INFINITE</code> value.</p>
    pub fn set_suppressed_until(mut self, input: ::std::option::Option<i64>) -> Self {
        self.suppressed_until = input;
        self
    }
    /// <p>If the anomaly is suppressed, this indicates when the suppression will end. If this value is <code>0</code>, the anomaly was suppressed with no expiration, with the <code>INFINITE</code> value.</p>
    pub fn get_suppressed_until(&self) -> &::std::option::Option<i64> {
        &self.suppressed_until
    }
    /// <p>If this anomaly is suppressed, this field is <code>true</code> if the suppression is because the pattern is suppressed. If <code>false</code>, then only this particular anomaly is suppressed.</p>
    pub fn is_pattern_level_suppression(mut self, input: bool) -> Self {
        self.is_pattern_level_suppression = ::std::option::Option::Some(input);
        self
    }
    /// <p>If this anomaly is suppressed, this field is <code>true</code> if the suppression is because the pattern is suppressed. If <code>false</code>, then only this particular anomaly is suppressed.</p>
    pub fn set_is_pattern_level_suppression(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_pattern_level_suppression = input;
        self
    }
    /// <p>If this anomaly is suppressed, this field is <code>true</code> if the suppression is because the pattern is suppressed. If <code>false</code>, then only this particular anomaly is suppressed.</p>
    pub fn get_is_pattern_level_suppression(&self) -> &::std::option::Option<bool> {
        &self.is_pattern_level_suppression
    }
    /// Consumes the builder and constructs a [`Anomaly`](crate::types::Anomaly).
    /// This method will fail if any of the following fields are not set:
    /// - [`anomaly_id`](crate::types::builders::AnomalyBuilder::anomaly_id)
    /// - [`pattern_id`](crate::types::builders::AnomalyBuilder::pattern_id)
    /// - [`anomaly_detector_arn`](crate::types::builders::AnomalyBuilder::anomaly_detector_arn)
    /// - [`pattern_string`](crate::types::builders::AnomalyBuilder::pattern_string)
    /// - [`description`](crate::types::builders::AnomalyBuilder::description)
    /// - [`active`](crate::types::builders::AnomalyBuilder::active)
    /// - [`state`](crate::types::builders::AnomalyBuilder::state)
    /// - [`histogram`](crate::types::builders::AnomalyBuilder::histogram)
    /// - [`log_samples`](crate::types::builders::AnomalyBuilder::log_samples)
    /// - [`pattern_tokens`](crate::types::builders::AnomalyBuilder::pattern_tokens)
    /// - [`log_group_arn_list`](crate::types::builders::AnomalyBuilder::log_group_arn_list)
    pub fn build(self) -> ::std::result::Result<crate::types::Anomaly, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Anomaly {
            anomaly_id: self.anomaly_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "anomaly_id",
                    "anomaly_id was not specified but it is required when building Anomaly",
                )
            })?,
            pattern_id: self.pattern_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "pattern_id",
                    "pattern_id was not specified but it is required when building Anomaly",
                )
            })?,
            anomaly_detector_arn: self.anomaly_detector_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "anomaly_detector_arn",
                    "anomaly_detector_arn was not specified but it is required when building Anomaly",
                )
            })?,
            pattern_string: self.pattern_string.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "pattern_string",
                    "pattern_string was not specified but it is required when building Anomaly",
                )
            })?,
            pattern_regex: self.pattern_regex,
            priority: self.priority,
            first_seen: self.first_seen.unwrap_or_default(),
            last_seen: self.last_seen.unwrap_or_default(),
            description: self.description.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "description",
                    "description was not specified but it is required when building Anomaly",
                )
            })?,
            active: self.active.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "active",
                    "active was not specified but it is required when building Anomaly",
                )
            })?,
            state: self.state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state",
                    "state was not specified but it is required when building Anomaly",
                )
            })?,
            histogram: self.histogram.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "histogram",
                    "histogram was not specified but it is required when building Anomaly",
                )
            })?,
            log_samples: self.log_samples.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "log_samples",
                    "log_samples was not specified but it is required when building Anomaly",
                )
            })?,
            pattern_tokens: self.pattern_tokens.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "pattern_tokens",
                    "pattern_tokens was not specified but it is required when building Anomaly",
                )
            })?,
            log_group_arn_list: self.log_group_arn_list.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "log_group_arn_list",
                    "log_group_arn_list was not specified but it is required when building Anomaly",
                )
            })?,
            suppressed: self.suppressed,
            suppressed_date: self.suppressed_date.unwrap_or_default(),
            suppressed_until: self.suppressed_until.unwrap_or_default(),
            is_pattern_level_suppression: self.is_pattern_level_suppression,
        })
    }
}
