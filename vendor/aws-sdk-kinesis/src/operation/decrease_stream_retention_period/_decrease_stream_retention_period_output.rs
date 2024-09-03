// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecreaseStreamRetentionPeriodOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DecreaseStreamRetentionPeriodOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DecreaseStreamRetentionPeriodOutput {
    /// Creates a new builder-style object to manufacture [`DecreaseStreamRetentionPeriodOutput`](crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput).
    pub fn builder() -> crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodOutputBuilder {
        crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodOutputBuilder::default()
    }
}

/// A builder for [`DecreaseStreamRetentionPeriodOutput`](crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecreaseStreamRetentionPeriodOutputBuilder {
    _request_id: Option<String>,
}
impl DecreaseStreamRetentionPeriodOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DecreaseStreamRetentionPeriodOutput`](crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput).
    pub fn build(self) -> crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput {
        crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput {
            _request_id: self._request_id,
        }
    }
}
