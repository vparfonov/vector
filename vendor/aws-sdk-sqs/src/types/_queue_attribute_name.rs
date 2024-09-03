// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `QueueAttributeName`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let queueattributename = unimplemented!();
/// match queueattributename {
///     QueueAttributeName::All => { /* ... */ },
///     QueueAttributeName::ApproximateNumberOfMessages => { /* ... */ },
///     QueueAttributeName::ApproximateNumberOfMessagesDelayed => { /* ... */ },
///     QueueAttributeName::ApproximateNumberOfMessagesNotVisible => { /* ... */ },
///     QueueAttributeName::ContentBasedDeduplication => { /* ... */ },
///     QueueAttributeName::CreatedTimestamp => { /* ... */ },
///     QueueAttributeName::DeduplicationScope => { /* ... */ },
///     QueueAttributeName::DelaySeconds => { /* ... */ },
///     QueueAttributeName::FifoQueue => { /* ... */ },
///     QueueAttributeName::FifoThroughputLimit => { /* ... */ },
///     QueueAttributeName::KmsDataKeyReusePeriodSeconds => { /* ... */ },
///     QueueAttributeName::KmsMasterKeyId => { /* ... */ },
///     QueueAttributeName::LastModifiedTimestamp => { /* ... */ },
///     QueueAttributeName::MaximumMessageSize => { /* ... */ },
///     QueueAttributeName::MessageRetentionPeriod => { /* ... */ },
///     QueueAttributeName::Policy => { /* ... */ },
///     QueueAttributeName::QueueArn => { /* ... */ },
///     QueueAttributeName::ReceiveMessageWaitTimeSeconds => { /* ... */ },
///     QueueAttributeName::RedriveAllowPolicy => { /* ... */ },
///     QueueAttributeName::RedrivePolicy => { /* ... */ },
///     QueueAttributeName::SqsManagedSseEnabled => { /* ... */ },
///     QueueAttributeName::VisibilityTimeout => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `queueattributename` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `QueueAttributeName::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `QueueAttributeName::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `QueueAttributeName::NewFeature` is defined.
/// Specifically, when `queueattributename` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `QueueAttributeName::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum QueueAttributeName {
    #[allow(missing_docs)] // documentation missing in model
    All,
    #[allow(missing_docs)] // documentation missing in model
    ApproximateNumberOfMessages,
    #[allow(missing_docs)] // documentation missing in model
    ApproximateNumberOfMessagesDelayed,
    #[allow(missing_docs)] // documentation missing in model
    ApproximateNumberOfMessagesNotVisible,
    #[allow(missing_docs)] // documentation missing in model
    ContentBasedDeduplication,
    #[allow(missing_docs)] // documentation missing in model
    CreatedTimestamp,
    #[allow(missing_docs)] // documentation missing in model
    DeduplicationScope,
    #[allow(missing_docs)] // documentation missing in model
    DelaySeconds,
    #[allow(missing_docs)] // documentation missing in model
    FifoQueue,
    #[allow(missing_docs)] // documentation missing in model
    FifoThroughputLimit,
    #[allow(missing_docs)] // documentation missing in model
    KmsDataKeyReusePeriodSeconds,
    #[allow(missing_docs)] // documentation missing in model
    KmsMasterKeyId,
    #[allow(missing_docs)] // documentation missing in model
    LastModifiedTimestamp,
    #[allow(missing_docs)] // documentation missing in model
    MaximumMessageSize,
    #[allow(missing_docs)] // documentation missing in model
    MessageRetentionPeriod,
    #[allow(missing_docs)] // documentation missing in model
    Policy,
    #[allow(missing_docs)] // documentation missing in model
    QueueArn,
    #[allow(missing_docs)] // documentation missing in model
    ReceiveMessageWaitTimeSeconds,
    #[allow(missing_docs)] // documentation missing in model
    RedriveAllowPolicy,
    #[allow(missing_docs)] // documentation missing in model
    RedrivePolicy,
    #[allow(missing_docs)] // documentation missing in model
    SqsManagedSseEnabled,
    #[allow(missing_docs)] // documentation missing in model
    VisibilityTimeout,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for QueueAttributeName {
    fn from(s: &str) -> Self {
        match s {
            "All" => QueueAttributeName::All,
            "ApproximateNumberOfMessages" => QueueAttributeName::ApproximateNumberOfMessages,
            "ApproximateNumberOfMessagesDelayed" => QueueAttributeName::ApproximateNumberOfMessagesDelayed,
            "ApproximateNumberOfMessagesNotVisible" => QueueAttributeName::ApproximateNumberOfMessagesNotVisible,
            "ContentBasedDeduplication" => QueueAttributeName::ContentBasedDeduplication,
            "CreatedTimestamp" => QueueAttributeName::CreatedTimestamp,
            "DeduplicationScope" => QueueAttributeName::DeduplicationScope,
            "DelaySeconds" => QueueAttributeName::DelaySeconds,
            "FifoQueue" => QueueAttributeName::FifoQueue,
            "FifoThroughputLimit" => QueueAttributeName::FifoThroughputLimit,
            "KmsDataKeyReusePeriodSeconds" => QueueAttributeName::KmsDataKeyReusePeriodSeconds,
            "KmsMasterKeyId" => QueueAttributeName::KmsMasterKeyId,
            "LastModifiedTimestamp" => QueueAttributeName::LastModifiedTimestamp,
            "MaximumMessageSize" => QueueAttributeName::MaximumMessageSize,
            "MessageRetentionPeriod" => QueueAttributeName::MessageRetentionPeriod,
            "Policy" => QueueAttributeName::Policy,
            "QueueArn" => QueueAttributeName::QueueArn,
            "ReceiveMessageWaitTimeSeconds" => QueueAttributeName::ReceiveMessageWaitTimeSeconds,
            "RedriveAllowPolicy" => QueueAttributeName::RedriveAllowPolicy,
            "RedrivePolicy" => QueueAttributeName::RedrivePolicy,
            "SqsManagedSseEnabled" => QueueAttributeName::SqsManagedSseEnabled,
            "VisibilityTimeout" => QueueAttributeName::VisibilityTimeout,
            other => QueueAttributeName::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for QueueAttributeName {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(QueueAttributeName::from(s))
    }
}
impl QueueAttributeName {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            QueueAttributeName::All => "All",
            QueueAttributeName::ApproximateNumberOfMessages => "ApproximateNumberOfMessages",
            QueueAttributeName::ApproximateNumberOfMessagesDelayed => "ApproximateNumberOfMessagesDelayed",
            QueueAttributeName::ApproximateNumberOfMessagesNotVisible => "ApproximateNumberOfMessagesNotVisible",
            QueueAttributeName::ContentBasedDeduplication => "ContentBasedDeduplication",
            QueueAttributeName::CreatedTimestamp => "CreatedTimestamp",
            QueueAttributeName::DeduplicationScope => "DeduplicationScope",
            QueueAttributeName::DelaySeconds => "DelaySeconds",
            QueueAttributeName::FifoQueue => "FifoQueue",
            QueueAttributeName::FifoThroughputLimit => "FifoThroughputLimit",
            QueueAttributeName::KmsDataKeyReusePeriodSeconds => "KmsDataKeyReusePeriodSeconds",
            QueueAttributeName::KmsMasterKeyId => "KmsMasterKeyId",
            QueueAttributeName::LastModifiedTimestamp => "LastModifiedTimestamp",
            QueueAttributeName::MaximumMessageSize => "MaximumMessageSize",
            QueueAttributeName::MessageRetentionPeriod => "MessageRetentionPeriod",
            QueueAttributeName::Policy => "Policy",
            QueueAttributeName::QueueArn => "QueueArn",
            QueueAttributeName::ReceiveMessageWaitTimeSeconds => "ReceiveMessageWaitTimeSeconds",
            QueueAttributeName::RedriveAllowPolicy => "RedriveAllowPolicy",
            QueueAttributeName::RedrivePolicy => "RedrivePolicy",
            QueueAttributeName::SqsManagedSseEnabled => "SqsManagedSseEnabled",
            QueueAttributeName::VisibilityTimeout => "VisibilityTimeout",
            QueueAttributeName::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "All",
            "ApproximateNumberOfMessages",
            "ApproximateNumberOfMessagesDelayed",
            "ApproximateNumberOfMessagesNotVisible",
            "ContentBasedDeduplication",
            "CreatedTimestamp",
            "DeduplicationScope",
            "DelaySeconds",
            "FifoQueue",
            "FifoThroughputLimit",
            "KmsDataKeyReusePeriodSeconds",
            "KmsMasterKeyId",
            "LastModifiedTimestamp",
            "MaximumMessageSize",
            "MessageRetentionPeriod",
            "Policy",
            "QueueArn",
            "ReceiveMessageWaitTimeSeconds",
            "RedriveAllowPolicy",
            "RedrivePolicy",
            "SqsManagedSseEnabled",
            "VisibilityTimeout",
        ]
    }
}
impl ::std::convert::AsRef<str> for QueueAttributeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl QueueAttributeName {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for QueueAttributeName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            QueueAttributeName::All => write!(f, "All"),
            QueueAttributeName::ApproximateNumberOfMessages => write!(f, "ApproximateNumberOfMessages"),
            QueueAttributeName::ApproximateNumberOfMessagesDelayed => write!(f, "ApproximateNumberOfMessagesDelayed"),
            QueueAttributeName::ApproximateNumberOfMessagesNotVisible => write!(f, "ApproximateNumberOfMessagesNotVisible"),
            QueueAttributeName::ContentBasedDeduplication => write!(f, "ContentBasedDeduplication"),
            QueueAttributeName::CreatedTimestamp => write!(f, "CreatedTimestamp"),
            QueueAttributeName::DeduplicationScope => write!(f, "DeduplicationScope"),
            QueueAttributeName::DelaySeconds => write!(f, "DelaySeconds"),
            QueueAttributeName::FifoQueue => write!(f, "FifoQueue"),
            QueueAttributeName::FifoThroughputLimit => write!(f, "FifoThroughputLimit"),
            QueueAttributeName::KmsDataKeyReusePeriodSeconds => write!(f, "KmsDataKeyReusePeriodSeconds"),
            QueueAttributeName::KmsMasterKeyId => write!(f, "KmsMasterKeyId"),
            QueueAttributeName::LastModifiedTimestamp => write!(f, "LastModifiedTimestamp"),
            QueueAttributeName::MaximumMessageSize => write!(f, "MaximumMessageSize"),
            QueueAttributeName::MessageRetentionPeriod => write!(f, "MessageRetentionPeriod"),
            QueueAttributeName::Policy => write!(f, "Policy"),
            QueueAttributeName::QueueArn => write!(f, "QueueArn"),
            QueueAttributeName::ReceiveMessageWaitTimeSeconds => write!(f, "ReceiveMessageWaitTimeSeconds"),
            QueueAttributeName::RedriveAllowPolicy => write!(f, "RedriveAllowPolicy"),
            QueueAttributeName::RedrivePolicy => write!(f, "RedrivePolicy"),
            QueueAttributeName::SqsManagedSseEnabled => write!(f, "SqsManagedSseEnabled"),
            QueueAttributeName::VisibilityTimeout => write!(f, "VisibilityTimeout"),
            QueueAttributeName::Unknown(value) => write!(f, "{}", value),
        }
    }
}
