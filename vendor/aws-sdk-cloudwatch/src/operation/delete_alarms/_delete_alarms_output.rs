// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAlarmsOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteAlarmsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteAlarmsOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAlarmsOutput`](crate::operation::delete_alarms::DeleteAlarmsOutput).
    pub fn builder() -> crate::operation::delete_alarms::builders::DeleteAlarmsOutputBuilder {
        crate::operation::delete_alarms::builders::DeleteAlarmsOutputBuilder::default()
    }
}

/// A builder for [`DeleteAlarmsOutput`](crate::operation::delete_alarms::DeleteAlarmsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteAlarmsOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteAlarmsOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAlarmsOutput`](crate::operation::delete_alarms::DeleteAlarmsOutput).
    pub fn build(self) -> crate::operation::delete_alarms::DeleteAlarmsOutput {
        crate::operation::delete_alarms::DeleteAlarmsOutput {
            _request_id: self._request_id,
        }
    }
}
