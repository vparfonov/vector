// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the encryption for a destination in Amazon S3.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EncryptionConfiguration {
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    pub no_encryption_config: ::std::option::Option<crate::types::NoEncryptionConfig>,
    /// <p>The encryption key.</p>
    pub kms_encryption_config: ::std::option::Option<crate::types::KmsEncryptionConfig>,
}
impl EncryptionConfiguration {
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    pub fn no_encryption_config(&self) -> ::std::option::Option<&crate::types::NoEncryptionConfig> {
        self.no_encryption_config.as_ref()
    }
    /// <p>The encryption key.</p>
    pub fn kms_encryption_config(&self) -> ::std::option::Option<&crate::types::KmsEncryptionConfig> {
        self.kms_encryption_config.as_ref()
    }
}
impl EncryptionConfiguration {
    /// Creates a new builder-style object to manufacture [`EncryptionConfiguration`](crate::types::EncryptionConfiguration).
    pub fn builder() -> crate::types::builders::EncryptionConfigurationBuilder {
        crate::types::builders::EncryptionConfigurationBuilder::default()
    }
}

/// A builder for [`EncryptionConfiguration`](crate::types::EncryptionConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EncryptionConfigurationBuilder {
    pub(crate) no_encryption_config: ::std::option::Option<crate::types::NoEncryptionConfig>,
    pub(crate) kms_encryption_config: ::std::option::Option<crate::types::KmsEncryptionConfig>,
}
impl EncryptionConfigurationBuilder {
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    pub fn no_encryption_config(mut self, input: crate::types::NoEncryptionConfig) -> Self {
        self.no_encryption_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    pub fn set_no_encryption_config(mut self, input: ::std::option::Option<crate::types::NoEncryptionConfig>) -> Self {
        self.no_encryption_config = input;
        self
    }
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    pub fn get_no_encryption_config(&self) -> &::std::option::Option<crate::types::NoEncryptionConfig> {
        &self.no_encryption_config
    }
    /// <p>The encryption key.</p>
    pub fn kms_encryption_config(mut self, input: crate::types::KmsEncryptionConfig) -> Self {
        self.kms_encryption_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption key.</p>
    pub fn set_kms_encryption_config(mut self, input: ::std::option::Option<crate::types::KmsEncryptionConfig>) -> Self {
        self.kms_encryption_config = input;
        self
    }
    /// <p>The encryption key.</p>
    pub fn get_kms_encryption_config(&self) -> &::std::option::Option<crate::types::KmsEncryptionConfig> {
        &self.kms_encryption_config
    }
    /// Consumes the builder and constructs a [`EncryptionConfiguration`](crate::types::EncryptionConfiguration).
    pub fn build(self) -> crate::types::EncryptionConfiguration {
        crate::types::EncryptionConfiguration {
            no_encryption_config: self.no_encryption_config,
            kms_encryption_config: self.kms_encryption_config,
        }
    }
}
