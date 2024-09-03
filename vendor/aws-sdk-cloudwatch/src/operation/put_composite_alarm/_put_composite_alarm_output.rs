// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutCompositeAlarmOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for PutCompositeAlarmOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutCompositeAlarmOutput {
    /// Creates a new builder-style object to manufacture [`PutCompositeAlarmOutput`](crate::operation::put_composite_alarm::PutCompositeAlarmOutput).
    pub fn builder() -> crate::operation::put_composite_alarm::builders::PutCompositeAlarmOutputBuilder {
        crate::operation::put_composite_alarm::builders::PutCompositeAlarmOutputBuilder::default()
    }
}

/// A builder for [`PutCompositeAlarmOutput`](crate::operation::put_composite_alarm::PutCompositeAlarmOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutCompositeAlarmOutputBuilder {
    _request_id: Option<String>,
}
impl PutCompositeAlarmOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutCompositeAlarmOutput`](crate::operation::put_composite_alarm::PutCompositeAlarmOutput).
    pub fn build(self) -> crate::operation::put_composite_alarm::PutCompositeAlarmOutput {
        crate::operation::put_composite_alarm::PutCompositeAlarmOutput {
            _request_id: self._request_id,
        }
    }
}
