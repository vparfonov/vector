// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `AutoTuneState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let autotunestate = unimplemented!();
/// match autotunestate {
///     AutoTuneState::Disabled => { /* ... */ },
///     AutoTuneState::DisabledAndRollbackComplete => { /* ... */ },
///     AutoTuneState::DisabledAndRollbackError => { /* ... */ },
///     AutoTuneState::DisabledAndRollbackInProgress => { /* ... */ },
///     AutoTuneState::DisabledAndRollbackScheduled => { /* ... */ },
///     AutoTuneState::DisableInProgress => { /* ... */ },
///     AutoTuneState::Enabled => { /* ... */ },
///     AutoTuneState::EnableInProgress => { /* ... */ },
///     AutoTuneState::Error => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `autotunestate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `AutoTuneState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `AutoTuneState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `AutoTuneState::NewFeature` is defined.
/// Specifically, when `autotunestate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `AutoTuneState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>Specifies the Auto-Tune state for the Elasticsearch domain. For valid states see the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a>.</p>
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum AutoTuneState {
    #[allow(missing_docs)] // documentation missing in model
    Disabled,
    #[allow(missing_docs)] // documentation missing in model
    DisabledAndRollbackComplete,
    #[allow(missing_docs)] // documentation missing in model
    DisabledAndRollbackError,
    #[allow(missing_docs)] // documentation missing in model
    DisabledAndRollbackInProgress,
    #[allow(missing_docs)] // documentation missing in model
    DisabledAndRollbackScheduled,
    #[allow(missing_docs)] // documentation missing in model
    DisableInProgress,
    #[allow(missing_docs)] // documentation missing in model
    Enabled,
    #[allow(missing_docs)] // documentation missing in model
    EnableInProgress,
    #[allow(missing_docs)] // documentation missing in model
    Error,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for AutoTuneState {
    fn from(s: &str) -> Self {
        match s {
            "DISABLED" => AutoTuneState::Disabled,
            "DISABLED_AND_ROLLBACK_COMPLETE" => AutoTuneState::DisabledAndRollbackComplete,
            "DISABLED_AND_ROLLBACK_ERROR" => AutoTuneState::DisabledAndRollbackError,
            "DISABLED_AND_ROLLBACK_IN_PROGRESS" => AutoTuneState::DisabledAndRollbackInProgress,
            "DISABLED_AND_ROLLBACK_SCHEDULED" => AutoTuneState::DisabledAndRollbackScheduled,
            "DISABLE_IN_PROGRESS" => AutoTuneState::DisableInProgress,
            "ENABLED" => AutoTuneState::Enabled,
            "ENABLE_IN_PROGRESS" => AutoTuneState::EnableInProgress,
            "ERROR" => AutoTuneState::Error,
            other => AutoTuneState::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for AutoTuneState {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(AutoTuneState::from(s))
    }
}
impl AutoTuneState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            AutoTuneState::Disabled => "DISABLED",
            AutoTuneState::DisabledAndRollbackComplete => "DISABLED_AND_ROLLBACK_COMPLETE",
            AutoTuneState::DisabledAndRollbackError => "DISABLED_AND_ROLLBACK_ERROR",
            AutoTuneState::DisabledAndRollbackInProgress => "DISABLED_AND_ROLLBACK_IN_PROGRESS",
            AutoTuneState::DisabledAndRollbackScheduled => "DISABLED_AND_ROLLBACK_SCHEDULED",
            AutoTuneState::DisableInProgress => "DISABLE_IN_PROGRESS",
            AutoTuneState::Enabled => "ENABLED",
            AutoTuneState::EnableInProgress => "ENABLE_IN_PROGRESS",
            AutoTuneState::Error => "ERROR",
            AutoTuneState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "DISABLED",
            "DISABLED_AND_ROLLBACK_COMPLETE",
            "DISABLED_AND_ROLLBACK_ERROR",
            "DISABLED_AND_ROLLBACK_IN_PROGRESS",
            "DISABLED_AND_ROLLBACK_SCHEDULED",
            "DISABLE_IN_PROGRESS",
            "ENABLED",
            "ENABLE_IN_PROGRESS",
            "ERROR",
        ]
    }
}
impl ::std::convert::AsRef<str> for AutoTuneState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AutoTuneState {
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
impl ::std::fmt::Display for AutoTuneState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            AutoTuneState::Disabled => write!(f, "DISABLED"),
            AutoTuneState::DisabledAndRollbackComplete => write!(f, "DISABLED_AND_ROLLBACK_COMPLETE"),
            AutoTuneState::DisabledAndRollbackError => write!(f, "DISABLED_AND_ROLLBACK_ERROR"),
            AutoTuneState::DisabledAndRollbackInProgress => write!(f, "DISABLED_AND_ROLLBACK_IN_PROGRESS"),
            AutoTuneState::DisabledAndRollbackScheduled => write!(f, "DISABLED_AND_ROLLBACK_SCHEDULED"),
            AutoTuneState::DisableInProgress => write!(f, "DISABLE_IN_PROGRESS"),
            AutoTuneState::Enabled => write!(f, "ENABLED"),
            AutoTuneState::EnableInProgress => write!(f, "ENABLE_IN_PROGRESS"),
            AutoTuneState::Error => write!(f, "ERROR"),
            AutoTuneState::Unknown(value) => write!(f, "{}", value),
        }
    }
}
