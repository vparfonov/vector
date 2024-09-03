// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ValidateResourcePolicy`](crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`secret_id(impl Into<String>)`](crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder::secret_id) / [`set_secret_id(Option<String>)`](crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder::set_secret_id):<br>required: **false**<br><p>The ARN or name of the secret with the resource-based policy you want to validate.</p><br>
    ///   - [`resource_policy(impl Into<String>)`](crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder::resource_policy) / [`set_resource_policy(Option<String>)`](crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder::set_resource_policy):<br>required: **true**<br><p>A JSON-formatted string that contains an Amazon Web Services resource-based policy. The policy in the string identifies who can access or manage this secret and its versions. For example policies, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html">Permissions policy examples</a>.</p><br>
    /// - On success, responds with [`ValidateResourcePolicyOutput`](crate::operation::validate_resource_policy::ValidateResourcePolicyOutput) with field(s):
    ///   - [`policy_validation_passed(bool)`](crate::operation::validate_resource_policy::ValidateResourcePolicyOutput::policy_validation_passed): <p>True if your policy passes validation, otherwise false.</p>
    ///   - [`validation_errors(Option<Vec::<ValidationErrorsEntry>>)`](crate::operation::validate_resource_policy::ValidateResourcePolicyOutput::validation_errors): <p>Validation errors if your policy didn't pass validation.</p>
    /// - On failure, responds with [`SdkError<ValidateResourcePolicyError>`](crate::operation::validate_resource_policy::ValidateResourcePolicyError)
    pub fn validate_resource_policy(&self) -> crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder {
        crate::operation::validate_resource_policy::builders::ValidateResourcePolicyFluentBuilder::new(self.handle.clone())
    }
}
