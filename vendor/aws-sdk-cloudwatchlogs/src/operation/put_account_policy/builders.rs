// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_account_policy::_put_account_policy_output::PutAccountPolicyOutputBuilder;

pub use crate::operation::put_account_policy::_put_account_policy_input::PutAccountPolicyInputBuilder;

impl PutAccountPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_account_policy::PutAccountPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_account_policy::PutAccountPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_account_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutAccountPolicy`.
///
/// <p>Creates an account-level data protection policy that applies to all log groups in the account. A data protection policy can help safeguard sensitive data that's ingested by your log groups by auditing and masking the sensitive log data. Each account can have only one account-level policy.</p> <important>
/// <p>Sensitive data is detected and masked when it is ingested into a log group. When you set a data protection policy, log events ingested into the log groups before that time are not masked.</p>
/// </important>
/// <p>If you use <code>PutAccountPolicy</code> to create a data protection policy for your whole account, it applies to both existing log groups and all log groups that are created later in this account. The account policy is applied to existing log groups with eventual consistency. It might take up to 5 minutes before sensitive data in existing log groups begins to be masked.</p>
/// <p>By default, when a user views a log event that includes masked data, the sensitive data is replaced by asterisks. A user who has the <code>logs:Unmask</code> permission can use a <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_GetLogEvents.html">GetLogEvents</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_FilterLogEvents.html">FilterLogEvents</a> operation with the <code>unmask</code> parameter set to <code>true</code> to view the unmasked log events. Users with the <code>logs:Unmask</code> can also view unmasked data in the CloudWatch Logs console by running a CloudWatch Logs Insights query with the <code>unmask</code> query command.</p>
/// <p>For more information, including a list of types of data that can be audited and masked, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data.html">Protect sensitive log data with masking</a>.</p>
/// <p>To use the <code>PutAccountPolicy</code> operation, you must be signed on with the <code>logs:PutDataProtectionPolicy</code> and <code>logs:PutAccountPolicy</code> permissions.</p>
/// <p>The <code>PutAccountPolicy</code> operation applies to all log groups in the account. You can also use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html">PutDataProtectionPolicy</a> to create a data protection policy that applies to just one log group. If a log group has its own data protection policy and the account also has an account-level data protection policy, then the two policies are cumulative. Any sensitive term specified in either policy is masked.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutAccountPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_account_policy::builders::PutAccountPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_account_policy::PutAccountPolicyOutput,
        crate::operation::put_account_policy::PutAccountPolicyError,
    > for PutAccountPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_account_policy::PutAccountPolicyOutput,
            crate::operation::put_account_policy::PutAccountPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutAccountPolicyFluentBuilder {
    /// Creates a new `PutAccountPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutAccountPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::put_account_policy::builders::PutAccountPolicyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_account_policy::PutAccountPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_account_policy::PutAccountPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_account_policy::PutAccountPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_account_policy::PutAccountPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_account_policy::PutAccountPolicyOutput,
        crate::operation::put_account_policy::PutAccountPolicyError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>A name for the policy. This must be unique within the account.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>A name for the policy. This must be unique within the account.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>A name for the policy. This must be unique within the account.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
    /// <p>Specify the data protection policy, in JSON.</p>
    /// <p>This policy must include two JSON blocks:</p>
    /// <ul>
    /// <li> <p>The first block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Audit</code> action. The <code>DataIdentifer</code> array lists the types of sensitive data that you want to mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that you can mask</a>.</p> <p>The <code>Operation</code> property with an <code>Audit</code> action is required to find the sensitive data terms. This <code>Audit</code> action must contain a <code>FindingsDestination</code> object. You can optionally use that <code>FindingsDestination</code> object to list one or more destinations to send audit findings to. If you specify destinations such as log groups, Kinesis Data Firehose streams, and S3 buckets, they must already exist.</p> </li>
    /// <li> <p>The second block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Deidentify</code> action. The <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array in the first block of the policy.</p> <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object. The <code> "MaskConfig": {}</code> object must be empty.</p> </li>
    /// </ul>
    /// <p>For an example data protection policy, see the <b>Examples</b> section on this page.</p> <important>
    /// <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
    /// </important>
    /// <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The <code>Name</code> is different than the operation's <code>policyName</code> parameter, and is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>
    /// <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_document(input.into());
        self
    }
    /// <p>Specify the data protection policy, in JSON.</p>
    /// <p>This policy must include two JSON blocks:</p>
    /// <ul>
    /// <li> <p>The first block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Audit</code> action. The <code>DataIdentifer</code> array lists the types of sensitive data that you want to mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that you can mask</a>.</p> <p>The <code>Operation</code> property with an <code>Audit</code> action is required to find the sensitive data terms. This <code>Audit</code> action must contain a <code>FindingsDestination</code> object. You can optionally use that <code>FindingsDestination</code> object to list one or more destinations to send audit findings to. If you specify destinations such as log groups, Kinesis Data Firehose streams, and S3 buckets, they must already exist.</p> </li>
    /// <li> <p>The second block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Deidentify</code> action. The <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array in the first block of the policy.</p> <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object. The <code> "MaskConfig": {}</code> object must be empty.</p> </li>
    /// </ul>
    /// <p>For an example data protection policy, see the <b>Examples</b> section on this page.</p> <important>
    /// <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
    /// </important>
    /// <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The <code>Name</code> is different than the operation's <code>policyName</code> parameter, and is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>
    /// <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_document(input);
        self
    }
    /// <p>Specify the data protection policy, in JSON.</p>
    /// <p>This policy must include two JSON blocks:</p>
    /// <ul>
    /// <li> <p>The first block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Audit</code> action. The <code>DataIdentifer</code> array lists the types of sensitive data that you want to mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that you can mask</a>.</p> <p>The <code>Operation</code> property with an <code>Audit</code> action is required to find the sensitive data terms. This <code>Audit</code> action must contain a <code>FindingsDestination</code> object. You can optionally use that <code>FindingsDestination</code> object to list one or more destinations to send audit findings to. If you specify destinations such as log groups, Kinesis Data Firehose streams, and S3 buckets, they must already exist.</p> </li>
    /// <li> <p>The second block must include both a <code>DataIdentifer</code> array and an <code>Operation</code> property with an <code>Deidentify</code> action. The <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array in the first block of the policy.</p> <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object. The <code> "MaskConfig": {}</code> object must be empty.</p> </li>
    /// </ul>
    /// <p>For an example data protection policy, see the <b>Examples</b> section on this page.</p> <important>
    /// <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
    /// </important>
    /// <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The <code>Name</code> is different than the operation's <code>policyName</code> parameter, and is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>
    /// <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_document()
    }
    /// <p>Currently the only valid value for this parameter is <code>DATA_PROTECTION_POLICY</code>.</p>
    pub fn policy_type(mut self, input: crate::types::PolicyType) -> Self {
        self.inner = self.inner.policy_type(input);
        self
    }
    /// <p>Currently the only valid value for this parameter is <code>DATA_PROTECTION_POLICY</code>.</p>
    pub fn set_policy_type(mut self, input: ::std::option::Option<crate::types::PolicyType>) -> Self {
        self.inner = self.inner.set_policy_type(input);
        self
    }
    /// <p>Currently the only valid value for this parameter is <code>DATA_PROTECTION_POLICY</code>.</p>
    pub fn get_policy_type(&self) -> &::std::option::Option<crate::types::PolicyType> {
        self.inner.get_policy_type()
    }
    /// <p>Currently the only valid value for this parameter is <code>ALL</code>, which specifies that the data protection policy applies to all log groups in the account. If you omit this parameter, the default of <code>ALL</code> is used.</p>
    pub fn scope(mut self, input: crate::types::Scope) -> Self {
        self.inner = self.inner.scope(input);
        self
    }
    /// <p>Currently the only valid value for this parameter is <code>ALL</code>, which specifies that the data protection policy applies to all log groups in the account. If you omit this parameter, the default of <code>ALL</code> is used.</p>
    pub fn set_scope(mut self, input: ::std::option::Option<crate::types::Scope>) -> Self {
        self.inner = self.inner.set_scope(input);
        self
    }
    /// <p>Currently the only valid value for this parameter is <code>ALL</code>, which specifies that the data protection policy applies to all log groups in the account. If you omit this parameter, the default of <code>ALL</code> is used.</p>
    pub fn get_scope(&self) -> &::std::option::Option<crate::types::Scope> {
        self.inner.get_scope()
    }
}
