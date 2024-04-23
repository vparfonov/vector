// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutResourcePolicy`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_name(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_policy_name):<br>required: **false**<br><p>Name of the new policy. This parameter is required.</p><br>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_policy_document):<br>required: **false**<br><p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>  <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>  <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>  <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>  <p></p>  <p> <code>{ "Version": "2012-10-17", "Statement": [ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": [ "route53.amazonaws.com" ] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } ] }</code> </p><br>
    /// - On success, responds with [`PutResourcePolicyOutput`](crate::operation::put_resource_policy::PutResourcePolicyOutput) with field(s):
    ///   - [`resource_policy(Option<ResourcePolicy>)`](crate::operation::put_resource_policy::PutResourcePolicyOutput::resource_policy): <p>The new policy.</p>
    /// - On failure, responds with [`SdkError<PutResourcePolicyError>`](crate::operation::put_resource_policy::PutResourcePolicyError)
    pub fn put_resource_policy(&self) -> crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder {
        crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::new(self.handle.clone())
    }
}
