// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetAlarmStateOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for SetAlarmStateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SetAlarmStateOutput {
    /// Creates a new builder-style object to manufacture [`SetAlarmStateOutput`](crate::operation::set_alarm_state::SetAlarmStateOutput).
    pub fn builder() -> crate::operation::set_alarm_state::builders::SetAlarmStateOutputBuilder {
        crate::operation::set_alarm_state::builders::SetAlarmStateOutputBuilder::default()
    }
}

/// A builder for [`SetAlarmStateOutput`](crate::operation::set_alarm_state::SetAlarmStateOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SetAlarmStateOutputBuilder {
    _request_id: Option<String>,
}
impl SetAlarmStateOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SetAlarmStateOutput`](crate::operation::set_alarm_state::SetAlarmStateOutput).
    pub fn build(self) -> crate::operation::set_alarm_state::SetAlarmStateOutput {
        crate::operation::set_alarm_state::SetAlarmStateOutput {
            _request_id: self._request_id,
        }
    }
}
