// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IncreaseStreamRetentionPeriodOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for IncreaseStreamRetentionPeriodOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl IncreaseStreamRetentionPeriodOutput {
    /// Creates a new builder-style object to manufacture [`IncreaseStreamRetentionPeriodOutput`](crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput).
    pub fn builder() -> crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodOutputBuilder {
        crate::operation::increase_stream_retention_period::builders::IncreaseStreamRetentionPeriodOutputBuilder::default()
    }
}

/// A builder for [`IncreaseStreamRetentionPeriodOutput`](crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct IncreaseStreamRetentionPeriodOutputBuilder {
    _request_id: Option<String>,
}
impl IncreaseStreamRetentionPeriodOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`IncreaseStreamRetentionPeriodOutput`](crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput).
    pub fn build(self) -> crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput {
        crate::operation::increase_stream_retention_period::IncreaseStreamRetentionPeriodOutput {
            _request_id: self._request_id,
        }
    }
}
