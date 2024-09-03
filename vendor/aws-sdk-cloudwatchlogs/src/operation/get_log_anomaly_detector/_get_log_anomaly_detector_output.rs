// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLogAnomalyDetectorOutput {
    /// <p>The name of the log anomaly detector</p>
    pub detector_name: ::std::option::Option<::std::string::String>,
    /// <p>An array of structures, where each structure contains the ARN of a log group associated with this anomaly detector.</p>
    pub log_group_arn_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub evaluation_frequency: ::std::option::Option<crate::types::EvaluationFrequency>,
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub filter_pattern: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the anomaly detector is currently active. To change its status, use the <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p>
    pub anomaly_detector_status: ::std::option::Option<crate::types::AnomalyDetectorStatus>,
    /// <p>The ID of the KMS key assigned to this anomaly detector, if any.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when this anomaly detector was created.</p>
    pub creation_time_stamp: i64,
    /// <p>The date and time when this anomaly detector was most recently modified.</p>
    pub last_modified_time_stamp: i64,
    /// <p>The number of days used as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal.</p>
    pub anomaly_visibility_time: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetLogAnomalyDetectorOutput {
    /// <p>The name of the log anomaly detector</p>
    pub fn detector_name(&self) -> ::std::option::Option<&str> {
        self.detector_name.as_deref()
    }
    /// <p>An array of structures, where each structure contains the ARN of a log group associated with this anomaly detector.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.log_group_arn_list.is_none()`.
    pub fn log_group_arn_list(&self) -> &[::std::string::String] {
        self.log_group_arn_list.as_deref().unwrap_or_default()
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn evaluation_frequency(&self) -> ::std::option::Option<&crate::types::EvaluationFrequency> {
        self.evaluation_frequency.as_ref()
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn filter_pattern(&self) -> ::std::option::Option<&str> {
        self.filter_pattern.as_deref()
    }
    /// <p>Specifies whether the anomaly detector is currently active. To change its status, use the <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p>
    pub fn anomaly_detector_status(&self) -> ::std::option::Option<&crate::types::AnomalyDetectorStatus> {
        self.anomaly_detector_status.as_ref()
    }
    /// <p>The ID of the KMS key assigned to this anomaly detector, if any.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The date and time when this anomaly detector was created.</p>
    pub fn creation_time_stamp(&self) -> i64 {
        self.creation_time_stamp
    }
    /// <p>The date and time when this anomaly detector was most recently modified.</p>
    pub fn last_modified_time_stamp(&self) -> i64 {
        self.last_modified_time_stamp
    }
    /// <p>The number of days used as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal.</p>
    pub fn anomaly_visibility_time(&self) -> ::std::option::Option<i64> {
        self.anomaly_visibility_time
    }
}
impl ::aws_types::request_id::RequestId for GetLogAnomalyDetectorOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetLogAnomalyDetectorOutput {
    /// Creates a new builder-style object to manufacture [`GetLogAnomalyDetectorOutput`](crate::operation::get_log_anomaly_detector::GetLogAnomalyDetectorOutput).
    pub fn builder() -> crate::operation::get_log_anomaly_detector::builders::GetLogAnomalyDetectorOutputBuilder {
        crate::operation::get_log_anomaly_detector::builders::GetLogAnomalyDetectorOutputBuilder::default()
    }
}

/// A builder for [`GetLogAnomalyDetectorOutput`](crate::operation::get_log_anomaly_detector::GetLogAnomalyDetectorOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetLogAnomalyDetectorOutputBuilder {
    pub(crate) detector_name: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_arn_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) evaluation_frequency: ::std::option::Option<crate::types::EvaluationFrequency>,
    pub(crate) filter_pattern: ::std::option::Option<::std::string::String>,
    pub(crate) anomaly_detector_status: ::std::option::Option<crate::types::AnomalyDetectorStatus>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time_stamp: ::std::option::Option<i64>,
    pub(crate) last_modified_time_stamp: ::std::option::Option<i64>,
    pub(crate) anomaly_visibility_time: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetLogAnomalyDetectorOutputBuilder {
    /// <p>The name of the log anomaly detector</p>
    pub fn detector_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detector_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the log anomaly detector</p>
    pub fn set_detector_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detector_name = input;
        self
    }
    /// <p>The name of the log anomaly detector</p>
    pub fn get_detector_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.detector_name
    }
    /// Appends an item to `log_group_arn_list`.
    ///
    /// To override the contents of this collection use [`set_log_group_arn_list`](Self::set_log_group_arn_list).
    ///
    /// <p>An array of structures, where each structure contains the ARN of a log group associated with this anomaly detector.</p>
    pub fn log_group_arn_list(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.log_group_arn_list.unwrap_or_default();
        v.push(input.into());
        self.log_group_arn_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of structures, where each structure contains the ARN of a log group associated with this anomaly detector.</p>
    pub fn set_log_group_arn_list(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.log_group_arn_list = input;
        self
    }
    /// <p>An array of structures, where each structure contains the ARN of a log group associated with this anomaly detector.</p>
    pub fn get_log_group_arn_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.log_group_arn_list
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn evaluation_frequency(mut self, input: crate::types::EvaluationFrequency) -> Self {
        self.evaluation_frequency = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn set_evaluation_frequency(mut self, input: ::std::option::Option<crate::types::EvaluationFrequency>) -> Self {
        self.evaluation_frequency = input;
        self
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn get_evaluation_frequency(&self) -> &::std::option::Option<crate::types::EvaluationFrequency> {
        &self.evaluation_frequency
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn filter_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filter_pattern = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn set_filter_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filter_pattern = input;
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn get_filter_pattern(&self) -> &::std::option::Option<::std::string::String> {
        &self.filter_pattern
    }
    /// <p>Specifies whether the anomaly detector is currently active. To change its status, use the <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p>
    pub fn anomaly_detector_status(mut self, input: crate::types::AnomalyDetectorStatus) -> Self {
        self.anomaly_detector_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the anomaly detector is currently active. To change its status, use the <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p>
    pub fn set_anomaly_detector_status(mut self, input: ::std::option::Option<crate::types::AnomalyDetectorStatus>) -> Self {
        self.anomaly_detector_status = input;
        self
    }
    /// <p>Specifies whether the anomaly detector is currently active. To change its status, use the <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p>
    pub fn get_anomaly_detector_status(&self) -> &::std::option::Option<crate::types::AnomalyDetectorStatus> {
        &self.anomaly_detector_status
    }
    /// <p>The ID of the KMS key assigned to this anomaly detector, if any.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the KMS key assigned to this anomaly detector, if any.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The ID of the KMS key assigned to this anomaly detector, if any.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// <p>The date and time when this anomaly detector was created.</p>
    pub fn creation_time_stamp(mut self, input: i64) -> Self {
        self.creation_time_stamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when this anomaly detector was created.</p>
    pub fn set_creation_time_stamp(mut self, input: ::std::option::Option<i64>) -> Self {
        self.creation_time_stamp = input;
        self
    }
    /// <p>The date and time when this anomaly detector was created.</p>
    pub fn get_creation_time_stamp(&self) -> &::std::option::Option<i64> {
        &self.creation_time_stamp
    }
    /// <p>The date and time when this anomaly detector was most recently modified.</p>
    pub fn last_modified_time_stamp(mut self, input: i64) -> Self {
        self.last_modified_time_stamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when this anomaly detector was most recently modified.</p>
    pub fn set_last_modified_time_stamp(mut self, input: ::std::option::Option<i64>) -> Self {
        self.last_modified_time_stamp = input;
        self
    }
    /// <p>The date and time when this anomaly detector was most recently modified.</p>
    pub fn get_last_modified_time_stamp(&self) -> &::std::option::Option<i64> {
        &self.last_modified_time_stamp
    }
    /// <p>The number of days used as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal.</p>
    pub fn anomaly_visibility_time(mut self, input: i64) -> Self {
        self.anomaly_visibility_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of days used as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal.</p>
    pub fn set_anomaly_visibility_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.anomaly_visibility_time = input;
        self
    }
    /// <p>The number of days used as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal.</p>
    pub fn get_anomaly_visibility_time(&self) -> &::std::option::Option<i64> {
        &self.anomaly_visibility_time
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetLogAnomalyDetectorOutput`](crate::operation::get_log_anomaly_detector::GetLogAnomalyDetectorOutput).
    pub fn build(self) -> crate::operation::get_log_anomaly_detector::GetLogAnomalyDetectorOutput {
        crate::operation::get_log_anomaly_detector::GetLogAnomalyDetectorOutput {
            detector_name: self.detector_name,
            log_group_arn_list: self.log_group_arn_list,
            evaluation_frequency: self.evaluation_frequency,
            filter_pattern: self.filter_pattern,
            anomaly_detector_status: self.anomaly_detector_status,
            kms_key_id: self.kms_key_id,
            creation_time_stamp: self.creation_time_stamp.unwrap_or_default(),
            last_modified_time_stamp: self.last_modified_time_stamp.unwrap_or_default(),
            anomaly_visibility_time: self.anomaly_visibility_time,
            _request_id: self._request_id,
        }
    }
}
