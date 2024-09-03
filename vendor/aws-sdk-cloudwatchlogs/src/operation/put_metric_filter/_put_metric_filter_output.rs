// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutMetricFilterOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for PutMetricFilterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutMetricFilterOutput {
    /// Creates a new builder-style object to manufacture [`PutMetricFilterOutput`](crate::operation::put_metric_filter::PutMetricFilterOutput).
    pub fn builder() -> crate::operation::put_metric_filter::builders::PutMetricFilterOutputBuilder {
        crate::operation::put_metric_filter::builders::PutMetricFilterOutputBuilder::default()
    }
}

/// A builder for [`PutMetricFilterOutput`](crate::operation::put_metric_filter::PutMetricFilterOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutMetricFilterOutputBuilder {
    _request_id: Option<String>,
}
impl PutMetricFilterOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutMetricFilterOutput`](crate::operation::put_metric_filter::PutMetricFilterOutput).
    pub fn build(self) -> crate::operation::put_metric_filter::PutMetricFilterOutput {
        crate::operation::put_metric_filter::PutMetricFilterOutput {
            _request_id: self._request_id,
        }
    }
}
