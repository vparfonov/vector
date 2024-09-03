// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_queue::_tag_queue_output::TagQueueOutputBuilder;

pub use crate::operation::tag_queue::_tag_queue_input::TagQueueInputBuilder;

impl crate::operation::tag_queue::builders::TagQueueInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::tag_queue::TagQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_queue::TagQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.tag_queue();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TagQueue`.
///
/// <p>Add cost allocation tags to the specified Amazon SQS queue. For an overview, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-tags.html">Tagging Your Amazon SQS Queues</a> in the <i>Amazon SQS Developer Guide</i>.</p>
/// <p>When you use queue tags, keep the following guidelines in mind:</p>
/// <ul>
/// <li>
/// <p>Adding more than 50 tags to a queue isn't recommended.</p></li>
/// <li>
/// <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p></li>
/// <li>
/// <p>Tags are case-sensitive.</p></li>
/// <li>
/// <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p></li>
/// </ul>
/// <p>For a full list of tag restrictions, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-limits.html#limits-queues">Quotas related to queues</a> in the <i>Amazon SQS Developer Guide</i>.</p><note>
/// <p>Cross-account permissions don't apply to this action. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-customer-managed-policy-examples.html#grant-cross-account-permissions-to-role-and-user-name">Grant cross-account permissions to a role and a username</a> in the <i>Amazon SQS Developer Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TagQueueFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_queue::builders::TagQueueInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::tag_queue::TagQueueOutput, crate::operation::tag_queue::TagQueueError>
    for TagQueueFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::tag_queue::TagQueueOutput, crate::operation::tag_queue::TagQueueError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TagQueueFluentBuilder {
    /// Creates a new `TagQueueFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TagQueue as a reference.
    pub fn as_input(&self) -> &crate::operation::tag_queue::builders::TagQueueInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_queue::TagQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_queue::TagQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::tag_queue::TagQueue::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::tag_queue::TagQueue::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::tag_queue::TagQueueOutput, crate::operation::tag_queue::TagQueueError, Self>
    {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
