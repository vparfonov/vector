// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableAlarmActionsInput {
    /// <p>The names of the alarms.</p>
    pub alarm_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl EnableAlarmActionsInput {
    /// <p>The names of the alarms.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.alarm_names.is_none()`.
    pub fn alarm_names(&self) -> &[::std::string::String] {
        self.alarm_names.as_deref().unwrap_or_default()
    }
}
impl EnableAlarmActionsInput {
    /// Creates a new builder-style object to manufacture [`EnableAlarmActionsInput`](crate::operation::enable_alarm_actions::EnableAlarmActionsInput).
    pub fn builder() -> crate::operation::enable_alarm_actions::builders::EnableAlarmActionsInputBuilder {
        crate::operation::enable_alarm_actions::builders::EnableAlarmActionsInputBuilder::default()
    }
}

/// A builder for [`EnableAlarmActionsInput`](crate::operation::enable_alarm_actions::EnableAlarmActionsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EnableAlarmActionsInputBuilder {
    pub(crate) alarm_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl EnableAlarmActionsInputBuilder {
    /// Appends an item to `alarm_names`.
    ///
    /// To override the contents of this collection use [`set_alarm_names`](Self::set_alarm_names).
    ///
    /// <p>The names of the alarms.</p>
    pub fn alarm_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.alarm_names.unwrap_or_default();
        v.push(input.into());
        self.alarm_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The names of the alarms.</p>
    pub fn set_alarm_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.alarm_names = input;
        self
    }
    /// <p>The names of the alarms.</p>
    pub fn get_alarm_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.alarm_names
    }
    /// Consumes the builder and constructs a [`EnableAlarmActionsInput`](crate::operation::enable_alarm_actions::EnableAlarmActionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::enable_alarm_actions::EnableAlarmActionsInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::enable_alarm_actions::EnableAlarmActionsInput {
            alarm_names: self.alarm_names,
        })
    }
}
