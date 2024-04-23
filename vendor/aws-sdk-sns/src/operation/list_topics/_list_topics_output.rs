// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Response for ListTopics action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTopicsOutput {
    /// <p>A list of topic ARNs.</p>
    pub topics: ::std::option::Option<::std::vec::Vec<crate::types::Topic>>,
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTopicsOutput {
    /// <p>A list of topic ARNs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.topics.is_none()`.
    pub fn topics(&self) -> &[crate::types::Topic] {
        self.topics.as_deref().unwrap_or_default()
    }
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListTopicsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTopicsOutput {
    /// Creates a new builder-style object to manufacture [`ListTopicsOutput`](crate::operation::list_topics::ListTopicsOutput).
    pub fn builder() -> crate::operation::list_topics::builders::ListTopicsOutputBuilder {
        crate::operation::list_topics::builders::ListTopicsOutputBuilder::default()
    }
}

/// A builder for [`ListTopicsOutput`](crate::operation::list_topics::ListTopicsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListTopicsOutputBuilder {
    pub(crate) topics: ::std::option::Option<::std::vec::Vec<crate::types::Topic>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTopicsOutputBuilder {
    /// Appends an item to `topics`.
    ///
    /// To override the contents of this collection use [`set_topics`](Self::set_topics).
    ///
    /// <p>A list of topic ARNs.</p>
    pub fn topics(mut self, input: crate::types::Topic) -> Self {
        let mut v = self.topics.unwrap_or_default();
        v.push(input);
        self.topics = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of topic ARNs.</p>
    pub fn set_topics(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Topic>>) -> Self {
        self.topics = input;
        self
    }
    /// <p>A list of topic ARNs.</p>
    pub fn get_topics(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Topic>> {
        &self.topics
    }
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListTopicsOutput`](crate::operation::list_topics::ListTopicsOutput).
    pub fn build(self) -> crate::operation::list_topics::ListTopicsOutput {
        crate::operation::list_topics::ListTopicsOutput {
            topics: self.topics,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
