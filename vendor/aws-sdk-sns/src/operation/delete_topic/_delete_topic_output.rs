// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTopicOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteTopicOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTopicOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTopicOutput`](crate::operation::delete_topic::DeleteTopicOutput).
    pub fn builder() -> crate::operation::delete_topic::builders::DeleteTopicOutputBuilder {
        crate::operation::delete_topic::builders::DeleteTopicOutputBuilder::default()
    }
}

/// A builder for [`DeleteTopicOutput`](crate::operation::delete_topic::DeleteTopicOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteTopicOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteTopicOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTopicOutput`](crate::operation::delete_topic::DeleteTopicOutput).
    pub fn build(self) -> crate::operation::delete_topic::DeleteTopicOutput {
        crate::operation::delete_topic::DeleteTopicOutput {
            _request_id: self._request_id,
        }
    }
}
