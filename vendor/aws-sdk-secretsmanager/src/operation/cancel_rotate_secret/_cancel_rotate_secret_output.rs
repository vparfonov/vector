// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelRotateSecretOutput {
    /// <p>The ARN of the secret.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the secret.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the version of the secret created during the rotation. This version might not be complete, and should be evaluated for possible deletion. We recommend that you remove the <code>VersionStage</code> value <code>AWSPENDING</code> from this version so that Secrets Manager can delete it. Failing to clean up a cancelled rotation can block you from starting future rotations.</p>
    pub version_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CancelRotateSecretOutput {
    /// <p>The ARN of the secret.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the secret.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The unique identifier of the version of the secret created during the rotation. This version might not be complete, and should be evaluated for possible deletion. We recommend that you remove the <code>VersionStage</code> value <code>AWSPENDING</code> from this version so that Secrets Manager can delete it. Failing to clean up a cancelled rotation can block you from starting future rotations.</p>
    pub fn version_id(&self) -> ::std::option::Option<&str> {
        self.version_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CancelRotateSecretOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelRotateSecretOutput {
    /// Creates a new builder-style object to manufacture [`CancelRotateSecretOutput`](crate::operation::cancel_rotate_secret::CancelRotateSecretOutput).
    pub fn builder() -> crate::operation::cancel_rotate_secret::builders::CancelRotateSecretOutputBuilder {
        crate::operation::cancel_rotate_secret::builders::CancelRotateSecretOutputBuilder::default()
    }
}

/// A builder for [`CancelRotateSecretOutput`](crate::operation::cancel_rotate_secret::CancelRotateSecretOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CancelRotateSecretOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CancelRotateSecretOutputBuilder {
    /// <p>The ARN of the secret.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the secret.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name of the secret.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the secret.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the secret.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The unique identifier of the version of the secret created during the rotation. This version might not be complete, and should be evaluated for possible deletion. We recommend that you remove the <code>VersionStage</code> value <code>AWSPENDING</code> from this version so that Secrets Manager can delete it. Failing to clean up a cancelled rotation can block you from starting future rotations.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the version of the secret created during the rotation. This version might not be complete, and should be evaluated for possible deletion. We recommend that you remove the <code>VersionStage</code> value <code>AWSPENDING</code> from this version so that Secrets Manager can delete it. Failing to clean up a cancelled rotation can block you from starting future rotations.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>The unique identifier of the version of the secret created during the rotation. This version might not be complete, and should be evaluated for possible deletion. We recommend that you remove the <code>VersionStage</code> value <code>AWSPENDING</code> from this version so that Secrets Manager can delete it. Failing to clean up a cancelled rotation can block you from starting future rotations.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.version_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CancelRotateSecretOutput`](crate::operation::cancel_rotate_secret::CancelRotateSecretOutput).
    pub fn build(self) -> crate::operation::cancel_rotate_secret::CancelRotateSecretOutput {
        crate::operation::cancel_rotate_secret::CancelRotateSecretOutput {
            arn: self.arn,
            name: self.name,
            version_id: self.version_id,
            _request_id: self._request_id,
        }
    }
}
