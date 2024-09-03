// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateShardCountOutput {
    /// <p>The name of the stream.</p>
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The current number of shards.</p>
    pub current_shard_count: ::std::option::Option<i32>,
    /// <p>The updated number of shards.</p>
    pub target_shard_count: ::std::option::Option<i32>,
    /// <p>The ARN of the stream.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateShardCountOutput {
    /// <p>The name of the stream.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>The current number of shards.</p>
    pub fn current_shard_count(&self) -> ::std::option::Option<i32> {
        self.current_shard_count
    }
    /// <p>The updated number of shards.</p>
    pub fn target_shard_count(&self) -> ::std::option::Option<i32> {
        self.target_shard_count
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateShardCountOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateShardCountOutput {
    /// Creates a new builder-style object to manufacture [`UpdateShardCountOutput`](crate::operation::update_shard_count::UpdateShardCountOutput).
    pub fn builder() -> crate::operation::update_shard_count::builders::UpdateShardCountOutputBuilder {
        crate::operation::update_shard_count::builders::UpdateShardCountOutputBuilder::default()
    }
}

/// A builder for [`UpdateShardCountOutput`](crate::operation::update_shard_count::UpdateShardCountOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateShardCountOutputBuilder {
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) current_shard_count: ::std::option::Option<i32>,
    pub(crate) target_shard_count: ::std::option::Option<i32>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateShardCountOutputBuilder {
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
    /// <p>The current number of shards.</p>
    pub fn current_shard_count(mut self, input: i32) -> Self {
        self.current_shard_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current number of shards.</p>
    pub fn set_current_shard_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.current_shard_count = input;
        self
    }
    /// <p>The current number of shards.</p>
    pub fn get_current_shard_count(&self) -> &::std::option::Option<i32> {
        &self.current_shard_count
    }
    /// <p>The updated number of shards.</p>
    pub fn target_shard_count(mut self, input: i32) -> Self {
        self.target_shard_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated number of shards.</p>
    pub fn set_target_shard_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.target_shard_count = input;
        self
    }
    /// <p>The updated number of shards.</p>
    pub fn get_target_shard_count(&self) -> &::std::option::Option<i32> {
        &self.target_shard_count
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateShardCountOutput`](crate::operation::update_shard_count::UpdateShardCountOutput).
    pub fn build(self) -> crate::operation::update_shard_count::UpdateShardCountOutput {
        crate::operation::update_shard_count::UpdateShardCountOutput {
            stream_name: self.stream_name,
            current_shard_count: self.current_shard_count,
            target_shard_count: self.target_shard_count,
            stream_arn: self.stream_arn,
            _request_id: self._request_id,
        }
    }
}
