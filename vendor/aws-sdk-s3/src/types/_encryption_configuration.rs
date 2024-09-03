// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies encryption-related information for an Amazon S3 bucket that is a destination for replicated objects.</p><note>
/// <p>If you're specifying a customer managed KMS key, we recommend using a fully qualified KMS key ARN. If you use a KMS key alias instead, then KMS resolves the key within the requester’s account. This behavior can result in data that's encrypted with a KMS key that belongs to the requester, and not the bucket owner.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EncryptionConfiguration {
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub replica_kms_key_id: ::std::option::Option<::std::string::String>,
}
impl EncryptionConfiguration {
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn replica_kms_key_id(&self) -> ::std::option::Option<&str> {
        self.replica_kms_key_id.as_deref()
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
    pub(crate) replica_kms_key_id: ::std::option::Option<::std::string::String>,
}
impl EncryptionConfigurationBuilder {
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn replica_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.replica_kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn set_replica_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.replica_kms_key_id = input;
        self
    }
    /// <p>Specifies the ID (Key ARN or Alias ARN) of the customer managed Amazon Web Services KMS key stored in Amazon Web Services Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn get_replica_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.replica_kms_key_id
    }
    /// Consumes the builder and constructs a [`EncryptionConfiguration`](crate::types::EncryptionConfiguration).
    pub fn build(self) -> crate::types::EncryptionConfiguration {
        crate::types::EncryptionConfiguration {
            replica_kms_key_id: self.replica_kms_key_id,
        }
    }
}
