// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetTopicAttributesOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for SetTopicAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SetTopicAttributesOutput {
    /// Creates a new builder-style object to manufacture [`SetTopicAttributesOutput`](crate::operation::set_topic_attributes::SetTopicAttributesOutput).
    pub fn builder() -> crate::operation::set_topic_attributes::builders::SetTopicAttributesOutputBuilder {
        crate::operation::set_topic_attributes::builders::SetTopicAttributesOutputBuilder::default()
    }
}

/// A builder for [`SetTopicAttributesOutput`](crate::operation::set_topic_attributes::SetTopicAttributesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SetTopicAttributesOutputBuilder {
    _request_id: Option<String>,
}
impl SetTopicAttributesOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SetTopicAttributesOutput`](crate::operation::set_topic_attributes::SetTopicAttributesOutput).
    pub fn build(self) -> crate::operation::set_topic_attributes::SetTopicAttributesOutput {
        crate::operation::set_topic_attributes::SetTopicAttributesOutput {
            _request_id: self._request_id,
        }
    }
}
