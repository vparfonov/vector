// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a log group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LogGroup {
    /// <p>The name of the log group.</p>
    pub log_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    pub creation_time: ::std::option::Option<i64>,
    /// <p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.</p>
    /// <p>To set a log group so that its log events do not expire, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DeleteRetentionPolicy.html">DeleteRetentionPolicy</a>.</p>
    pub retention_in_days: ::std::option::Option<i32>,
    /// <p>The number of metric filters.</p>
    pub metric_filter_count: ::std::option::Option<i32>,
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in IAM policies when specifying permissions for most API actions. The exception is when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. The permissions for those three actions require the ARN version that doesn't include a trailing <code>:*</code>.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The number of bytes stored.</p>
    pub stored_bytes: ::std::option::Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Displays whether this log group has a protection policy, or whether it had one in the past. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
    pub data_protection_status: ::std::option::Option<crate::types::DataProtectionStatus>,
    /// <p>Displays all the properties that this log group has inherited from account-level settings.</p>
    pub inherited_properties: ::std::option::Option<::std::vec::Vec<crate::types::InheritedProperty>>,
    /// <p>This specifies the log group class for this log group. There are two classes:</p>
    /// <ul>
    /// <li>
    /// <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p></li>
    /// <li>
    /// <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs features and incurs lower costs.</p></li>
    /// </ul>
    /// <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a></p>
    pub log_group_class: ::std::option::Option<crate::types::LogGroupClass>,
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in the following situations:</p>
    /// <ul>
    /// <li>
    /// <p>In the <code>logGroupIdentifier</code> input field in many CloudWatch Logs APIs.</p></li>
    /// <li>
    /// <p>In the <code>resourceArn</code> field in tagging APIs</p></li>
    /// <li>
    /// <p>In IAM policies, when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>.</p></li>
    /// </ul>
    pub log_group_arn: ::std::option::Option<::std::string::String>,
}
impl LogGroup {
    /// <p>The name of the log group.</p>
    pub fn log_group_name(&self) -> ::std::option::Option<&str> {
        self.log_group_name.as_deref()
    }
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    pub fn creation_time(&self) -> ::std::option::Option<i64> {
        self.creation_time
    }
    /// <p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.</p>
    /// <p>To set a log group so that its log events do not expire, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DeleteRetentionPolicy.html">DeleteRetentionPolicy</a>.</p>
    pub fn retention_in_days(&self) -> ::std::option::Option<i32> {
        self.retention_in_days
    }
    /// <p>The number of metric filters.</p>
    pub fn metric_filter_count(&self) -> ::std::option::Option<i32> {
        self.metric_filter_count
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in IAM policies when specifying permissions for most API actions. The exception is when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. The permissions for those three actions require the ARN version that doesn't include a trailing <code>:*</code>.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The number of bytes stored.</p>
    pub fn stored_bytes(&self) -> ::std::option::Option<i64> {
        self.stored_bytes
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>Displays whether this log group has a protection policy, or whether it had one in the past. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
    pub fn data_protection_status(&self) -> ::std::option::Option<&crate::types::DataProtectionStatus> {
        self.data_protection_status.as_ref()
    }
    /// <p>Displays all the properties that this log group has inherited from account-level settings.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.inherited_properties.is_none()`.
    pub fn inherited_properties(&self) -> &[crate::types::InheritedProperty] {
        self.inherited_properties.as_deref().unwrap_or_default()
    }
    /// <p>This specifies the log group class for this log group. There are two classes:</p>
    /// <ul>
    /// <li>
    /// <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p></li>
    /// <li>
    /// <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs features and incurs lower costs.</p></li>
    /// </ul>
    /// <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a></p>
    pub fn log_group_class(&self) -> ::std::option::Option<&crate::types::LogGroupClass> {
        self.log_group_class.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in the following situations:</p>
    /// <ul>
    /// <li>
    /// <p>In the <code>logGroupIdentifier</code> input field in many CloudWatch Logs APIs.</p></li>
    /// <li>
    /// <p>In the <code>resourceArn</code> field in tagging APIs</p></li>
    /// <li>
    /// <p>In IAM policies, when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>.</p></li>
    /// </ul>
    pub fn log_group_arn(&self) -> ::std::option::Option<&str> {
        self.log_group_arn.as_deref()
    }
}
impl LogGroup {
    /// Creates a new builder-style object to manufacture [`LogGroup`](crate::types::LogGroup).
    pub fn builder() -> crate::types::builders::LogGroupBuilder {
        crate::types::builders::LogGroupBuilder::default()
    }
}

/// A builder for [`LogGroup`](crate::types::LogGroup).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct LogGroupBuilder {
    pub(crate) log_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<i64>,
    pub(crate) retention_in_days: ::std::option::Option<i32>,
    pub(crate) metric_filter_count: ::std::option::Option<i32>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) stored_bytes: ::std::option::Option<i64>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) data_protection_status: ::std::option::Option<crate::types::DataProtectionStatus>,
    pub(crate) inherited_properties: ::std::option::Option<::std::vec::Vec<crate::types::InheritedProperty>>,
    pub(crate) log_group_class: ::std::option::Option<crate::types::LogGroupClass>,
    pub(crate) log_group_arn: ::std::option::Option<::std::string::String>,
}
impl LogGroupBuilder {
    /// <p>The name of the log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_name = input;
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_name
    }
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    pub fn creation_time(mut self, input: i64) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<i64> {
        &self.creation_time
    }
    /// <p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.</p>
    /// <p>To set a log group so that its log events do not expire, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DeleteRetentionPolicy.html">DeleteRetentionPolicy</a>.</p>
    pub fn retention_in_days(mut self, input: i32) -> Self {
        self.retention_in_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.</p>
    /// <p>To set a log group so that its log events do not expire, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DeleteRetentionPolicy.html">DeleteRetentionPolicy</a>.</p>
    pub fn set_retention_in_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.retention_in_days = input;
        self
    }
    /// <p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.</p>
    /// <p>To set a log group so that its log events do not expire, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DeleteRetentionPolicy.html">DeleteRetentionPolicy</a>.</p>
    pub fn get_retention_in_days(&self) -> &::std::option::Option<i32> {
        &self.retention_in_days
    }
    /// <p>The number of metric filters.</p>
    pub fn metric_filter_count(mut self, input: i32) -> Self {
        self.metric_filter_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of metric filters.</p>
    pub fn set_metric_filter_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.metric_filter_count = input;
        self
    }
    /// <p>The number of metric filters.</p>
    pub fn get_metric_filter_count(&self) -> &::std::option::Option<i32> {
        &self.metric_filter_count
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in IAM policies when specifying permissions for most API actions. The exception is when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. The permissions for those three actions require the ARN version that doesn't include a trailing <code>:*</code>.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in IAM policies when specifying permissions for most API actions. The exception is when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. The permissions for those three actions require the ARN version that doesn't include a trailing <code>:*</code>.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN includes a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in IAM policies when specifying permissions for most API actions. The exception is when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. The permissions for those three actions require the ARN version that doesn't include a trailing <code>:*</code>.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The number of bytes stored.</p>
    pub fn stored_bytes(mut self, input: i64) -> Self {
        self.stored_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of bytes stored.</p>
    pub fn set_stored_bytes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.stored_bytes = input;
        self
    }
    /// <p>The number of bytes stored.</p>
    pub fn get_stored_bytes(&self) -> &::std::option::Option<i64> {
        &self.stored_bytes
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log data.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// <p>Displays whether this log group has a protection policy, or whether it had one in the past. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
    pub fn data_protection_status(mut self, input: crate::types::DataProtectionStatus) -> Self {
        self.data_protection_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Displays whether this log group has a protection policy, or whether it had one in the past. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
    pub fn set_data_protection_status(mut self, input: ::std::option::Option<crate::types::DataProtectionStatus>) -> Self {
        self.data_protection_status = input;
        self
    }
    /// <p>Displays whether this log group has a protection policy, or whether it had one in the past. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a>.</p>
    pub fn get_data_protection_status(&self) -> &::std::option::Option<crate::types::DataProtectionStatus> {
        &self.data_protection_status
    }
    /// Appends an item to `inherited_properties`.
    ///
    /// To override the contents of this collection use [`set_inherited_properties`](Self::set_inherited_properties).
    ///
    /// <p>Displays all the properties that this log group has inherited from account-level settings.</p>
    pub fn inherited_properties(mut self, input: crate::types::InheritedProperty) -> Self {
        let mut v = self.inherited_properties.unwrap_or_default();
        v.push(input);
        self.inherited_properties = ::std::option::Option::Some(v);
        self
    }
    /// <p>Displays all the properties that this log group has inherited from account-level settings.</p>
    pub fn set_inherited_properties(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InheritedProperty>>) -> Self {
        self.inherited_properties = input;
        self
    }
    /// <p>Displays all the properties that this log group has inherited from account-level settings.</p>
    pub fn get_inherited_properties(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InheritedProperty>> {
        &self.inherited_properties
    }
    /// <p>This specifies the log group class for this log group. There are two classes:</p>
    /// <ul>
    /// <li>
    /// <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p></li>
    /// <li>
    /// <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs features and incurs lower costs.</p></li>
    /// </ul>
    /// <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a></p>
    pub fn log_group_class(mut self, input: crate::types::LogGroupClass) -> Self {
        self.log_group_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>This specifies the log group class for this log group. There are two classes:</p>
    /// <ul>
    /// <li>
    /// <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p></li>
    /// <li>
    /// <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs features and incurs lower costs.</p></li>
    /// </ul>
    /// <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a></p>
    pub fn set_log_group_class(mut self, input: ::std::option::Option<crate::types::LogGroupClass>) -> Self {
        self.log_group_class = input;
        self
    }
    /// <p>This specifies the log group class for this log group. There are two classes:</p>
    /// <ul>
    /// <li>
    /// <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p></li>
    /// <li>
    /// <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs features and incurs lower costs.</p></li>
    /// </ul>
    /// <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a></p>
    pub fn get_log_group_class(&self) -> &::std::option::Option<crate::types::LogGroupClass> {
        &self.log_group_class
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in the following situations:</p>
    /// <ul>
    /// <li>
    /// <p>In the <code>logGroupIdentifier</code> input field in many CloudWatch Logs APIs.</p></li>
    /// <li>
    /// <p>In the <code>resourceArn</code> field in tagging APIs</p></li>
    /// <li>
    /// <p>In IAM policies, when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>.</p></li>
    /// </ul>
    pub fn log_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in the following situations:</p>
    /// <ul>
    /// <li>
    /// <p>In the <code>logGroupIdentifier</code> input field in many CloudWatch Logs APIs.</p></li>
    /// <li>
    /// <p>In the <code>resourceArn</code> field in tagging APIs</p></li>
    /// <li>
    /// <p>In IAM policies, when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>.</p></li>
    /// </ul>
    pub fn set_log_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the log group. This version of the ARN doesn't include a trailing <code>:*</code> after the log group name.</p>
    /// <p>Use this version to refer to the ARN in the following situations:</p>
    /// <ul>
    /// <li>
    /// <p>In the <code>logGroupIdentifier</code> input field in many CloudWatch Logs APIs.</p></li>
    /// <li>
    /// <p>In the <code>resourceArn</code> field in tagging APIs</p></li>
    /// <li>
    /// <p>In IAM policies, when specifying permissions for <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a>, and <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>.</p></li>
    /// </ul>
    pub fn get_log_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_arn
    }
    /// Consumes the builder and constructs a [`LogGroup`](crate::types::LogGroup).
    pub fn build(self) -> crate::types::LogGroup {
        crate::types::LogGroup {
            log_group_name: self.log_group_name,
            creation_time: self.creation_time,
            retention_in_days: self.retention_in_days,
            metric_filter_count: self.metric_filter_count,
            arn: self.arn,
            stored_bytes: self.stored_bytes,
            kms_key_id: self.kms_key_id,
            data_protection_status: self.data_protection_status,
            inherited_properties: self.inherited_properties,
            log_group_class: self.log_group_class,
            log_group_arn: self.log_group_arn,
        }
    }
}
