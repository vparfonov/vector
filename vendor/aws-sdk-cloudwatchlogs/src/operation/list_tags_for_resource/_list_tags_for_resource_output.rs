// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>The list of tags associated with the requested resource.&gt;</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutput {
    /// <p>The list of tags associated with the requested resource.&gt;</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ListTagsForResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
    pub fn builder() -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder::default()
    }
}

/// A builder for [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTagsForResourceOutputBuilder {
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutputBuilder {
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags associated with the requested resource.&gt;</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The list of tags associated with the requested resource.&gt;</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The list of tags associated with the requested resource.&gt;</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
    pub fn build(self) -> crate::operation::list_tags_for_resource::ListTagsForResourceOutput {
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput {
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
