// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSecretVersionStageOutput {
    /// <p>The ARN of the secret that was updated.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the secret that was updated.</p>
    pub name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSecretVersionStageOutput {
    /// <p>The ARN of the secret that was updated.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the secret that was updated.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateSecretVersionStageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateSecretVersionStageOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSecretVersionStageOutput`](crate::operation::update_secret_version_stage::UpdateSecretVersionStageOutput).
    pub fn builder() -> crate::operation::update_secret_version_stage::builders::UpdateSecretVersionStageOutputBuilder {
        crate::operation::update_secret_version_stage::builders::UpdateSecretVersionStageOutputBuilder::default()
    }
}

/// A builder for [`UpdateSecretVersionStageOutput`](crate::operation::update_secret_version_stage::UpdateSecretVersionStageOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateSecretVersionStageOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSecretVersionStageOutputBuilder {
    /// <p>The ARN of the secret that was updated.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the secret that was updated.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the secret that was updated.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name of the secret that was updated.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the secret that was updated.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the secret that was updated.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSecretVersionStageOutput`](crate::operation::update_secret_version_stage::UpdateSecretVersionStageOutput).
    pub fn build(self) -> crate::operation::update_secret_version_stage::UpdateSecretVersionStageOutput {
        crate::operation::update_secret_version_stage::UpdateSecretVersionStageOutput {
            arn: self.arn,
            name: self.name,
            _request_id: self._request_id,
        }
    }
}
