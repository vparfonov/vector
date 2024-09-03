// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSecretInput {
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub secret_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSecretInput {
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn secret_id(&self) -> ::std::option::Option<&str> {
        self.secret_id.as_deref()
    }
}
impl DescribeSecretInput {
    /// Creates a new builder-style object to manufacture [`DescribeSecretInput`](crate::operation::describe_secret::DescribeSecretInput).
    pub fn builder() -> crate::operation::describe_secret::builders::DescribeSecretInputBuilder {
        crate::operation::describe_secret::builders::DescribeSecretInputBuilder::default()
    }
}

/// A builder for [`DescribeSecretInput`](crate::operation::describe_secret::DescribeSecretInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeSecretInputBuilder {
    pub(crate) secret_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSecretInputBuilder {
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    /// This field is required.
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_id = input;
        self
    }
    /// <p>The ARN or name of the secret.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn get_secret_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.secret_id
    }
    /// Consumes the builder and constructs a [`DescribeSecretInput`](crate::operation::describe_secret::DescribeSecretInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_secret::DescribeSecretInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_secret::DescribeSecretInput { secret_id: self.secret_id })
    }
}
