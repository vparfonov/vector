// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the current status of the Auto-Tune options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoTuneStatus {
    /// <p>Timestamp which tells Auto-Tune options creation date .</p>
    pub creation_date: ::aws_smithy_types::DateTime,
    /// <p>Timestamp which tells Auto-Tune options last updated time.</p>
    pub update_date: ::aws_smithy_types::DateTime,
    /// <p>Specifies the Auto-Tune options latest version.</p>
    pub update_version: i32,
    /// <p>Specifies the <code>AutoTuneState</code> for the Elasticsearch domain.</p>
    pub state: crate::types::AutoTuneState,
    /// <p>Specifies the error message while enabling or disabling the Auto-Tune options.</p>
    pub error_message: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    pub pending_deletion: ::std::option::Option<bool>,
}
impl AutoTuneStatus {
    /// <p>Timestamp which tells Auto-Tune options creation date .</p>
    pub fn creation_date(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_date
    }
    /// <p>Timestamp which tells Auto-Tune options last updated time.</p>
    pub fn update_date(&self) -> &::aws_smithy_types::DateTime {
        &self.update_date
    }
    /// <p>Specifies the Auto-Tune options latest version.</p>
    pub fn update_version(&self) -> i32 {
        self.update_version
    }
    /// <p>Specifies the <code>AutoTuneState</code> for the Elasticsearch domain.</p>
    pub fn state(&self) -> &crate::types::AutoTuneState {
        &self.state
    }
    /// <p>Specifies the error message while enabling or disabling the Auto-Tune options.</p>
    pub fn error_message(&self) -> ::std::option::Option<&str> {
        self.error_message.as_deref()
    }
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    pub fn pending_deletion(&self) -> ::std::option::Option<bool> {
        self.pending_deletion
    }
}
impl AutoTuneStatus {
    /// Creates a new builder-style object to manufacture [`AutoTuneStatus`](crate::types::AutoTuneStatus).
    pub fn builder() -> crate::types::builders::AutoTuneStatusBuilder {
        crate::types::builders::AutoTuneStatusBuilder::default()
    }
}

/// A builder for [`AutoTuneStatus`](crate::types::AutoTuneStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AutoTuneStatusBuilder {
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_version: ::std::option::Option<i32>,
    pub(crate) state: ::std::option::Option<crate::types::AutoTuneState>,
    pub(crate) error_message: ::std::option::Option<::std::string::String>,
    pub(crate) pending_deletion: ::std::option::Option<bool>,
}
impl AutoTuneStatusBuilder {
    /// <p>Timestamp which tells Auto-Tune options creation date .</p>
    /// This field is required.
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>Timestamp which tells Auto-Tune options creation date .</p>
    pub fn set_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>Timestamp which tells Auto-Tune options creation date .</p>
    pub fn get_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_date
    }
    /// <p>Timestamp which tells Auto-Tune options last updated time.</p>
    /// This field is required.
    pub fn update_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>Timestamp which tells Auto-Tune options last updated time.</p>
    pub fn set_update_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.update_date = input;
        self
    }
    /// <p>Timestamp which tells Auto-Tune options last updated time.</p>
    pub fn get_update_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.update_date
    }
    /// <p>Specifies the Auto-Tune options latest version.</p>
    pub fn update_version(mut self, input: i32) -> Self {
        self.update_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the Auto-Tune options latest version.</p>
    pub fn set_update_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.update_version = input;
        self
    }
    /// <p>Specifies the Auto-Tune options latest version.</p>
    pub fn get_update_version(&self) -> &::std::option::Option<i32> {
        &self.update_version
    }
    /// <p>Specifies the <code>AutoTuneState</code> for the Elasticsearch domain.</p>
    /// This field is required.
    pub fn state(mut self, input: crate::types::AutoTuneState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code>AutoTuneState</code> for the Elasticsearch domain.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::AutoTuneState>) -> Self {
        self.state = input;
        self
    }
    /// <p>Specifies the <code>AutoTuneState</code> for the Elasticsearch domain.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::AutoTuneState> {
        &self.state
    }
    /// <p>Specifies the error message while enabling or disabling the Auto-Tune options.</p>
    pub fn error_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the error message while enabling or disabling the Auto-Tune options.</p>
    pub fn set_error_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_message = input;
        self
    }
    /// <p>Specifies the error message while enabling or disabling the Auto-Tune options.</p>
    pub fn get_error_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_message
    }
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    pub fn pending_deletion(mut self, input: bool) -> Self {
        self.pending_deletion = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    pub fn set_pending_deletion(mut self, input: ::std::option::Option<bool>) -> Self {
        self.pending_deletion = input;
        self
    }
    /// <p>Indicates whether the Elasticsearch domain is being deleted.</p>
    pub fn get_pending_deletion(&self) -> &::std::option::Option<bool> {
        &self.pending_deletion
    }
    /// Consumes the builder and constructs a [`AutoTuneStatus`](crate::types::AutoTuneStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`creation_date`](crate::types::builders::AutoTuneStatusBuilder::creation_date)
    /// - [`update_date`](crate::types::builders::AutoTuneStatusBuilder::update_date)
    /// - [`state`](crate::types::builders::AutoTuneStatusBuilder::state)
    pub fn build(self) -> ::std::result::Result<crate::types::AutoTuneStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AutoTuneStatus {
            creation_date: self.creation_date.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_date",
                    "creation_date was not specified but it is required when building AutoTuneStatus",
                )
            })?,
            update_date: self.update_date.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "update_date",
                    "update_date was not specified but it is required when building AutoTuneStatus",
                )
            })?,
            update_version: self.update_version.unwrap_or_default(),
            state: self.state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state",
                    "state was not specified but it is required when building AutoTuneStatus",
                )
            })?,
            error_message: self.error_message,
            pending_deletion: self.pending_deletion,
        })
    }
}
