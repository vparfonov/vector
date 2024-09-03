// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input for <code>ListTagsForStream</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsForStreamInput {
    /// <p>The name of the stream.</p>
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    pub exclusive_start_tag_key: ::std::option::Option<::std::string::String>,
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    pub limit: ::std::option::Option<i32>,
    /// <p>The ARN of the stream.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
}
impl ListTagsForStreamInput {
    /// <p>The name of the stream.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    pub fn exclusive_start_tag_key(&self) -> ::std::option::Option<&str> {
        self.exclusive_start_tag_key.as_deref()
    }
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
}
impl ListTagsForStreamInput {
    /// Creates a new builder-style object to manufacture [`ListTagsForStreamInput`](crate::operation::list_tags_for_stream::ListTagsForStreamInput).
    pub fn builder() -> crate::operation::list_tags_for_stream::builders::ListTagsForStreamInputBuilder {
        crate::operation::list_tags_for_stream::builders::ListTagsForStreamInputBuilder::default()
    }
}

/// A builder for [`ListTagsForStreamInput`](crate::operation::list_tags_for_stream::ListTagsForStreamInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTagsForStreamInputBuilder {
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) exclusive_start_tag_key: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
}
impl ListTagsForStreamInputBuilder {
    /// <p>The name of the stream.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the stream.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// <p>The name of the stream.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_name
    }
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    pub fn exclusive_start_tag_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.exclusive_start_tag_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    pub fn set_exclusive_start_tag_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.exclusive_start_tag_key = input;
        self
    }
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    pub fn get_exclusive_start_tag_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.exclusive_start_tag_key
    }
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        &self.limit
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// Consumes the builder and constructs a [`ListTagsForStreamInput`](crate::operation::list_tags_for_stream::ListTagsForStreamInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_tags_for_stream::ListTagsForStreamInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_tags_for_stream::ListTagsForStreamInput {
            stream_name: self.stream_name,
            exclusive_start_tag_key: self.exclusive_start_tag_key,
            limit: self.limit,
            stream_arn: self.stream_arn,
        })
    }
}
