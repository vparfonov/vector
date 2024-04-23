// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::purge_queue::_purge_queue_output::PurgeQueueOutputBuilder;

pub use crate::operation::purge_queue::_purge_queue_input::PurgeQueueInputBuilder;

impl PurgeQueueInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::purge_queue::PurgeQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purge_queue::PurgeQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.purge_queue();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PurgeQueue`.
///
/// <p>Deletes available messages in a queue (including in-flight messages) specified by the <code>QueueURL</code> parameter.</p> <important>
/// <p>When you use the <code>PurgeQueue</code> action, you can't retrieve any messages deleted from a queue.</p>
/// <p>The message deletion process takes up to 60 seconds. We recommend waiting for 60 seconds regardless of your queue's size. </p>
/// </important>
/// <p>Messages sent to the queue <i>before</i> you call <code>PurgeQueue</code> might be received but are deleted within the next minute.</p>
/// <p>Messages sent to the queue <i>after</i> you call <code>PurgeQueue</code> might be deleted while the queue is being purged.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PurgeQueueFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::purge_queue::builders::PurgeQueueInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::purge_queue::PurgeQueueOutput,
        crate::operation::purge_queue::PurgeQueueError,
    > for PurgeQueueFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::purge_queue::PurgeQueueOutput,
            crate::operation::purge_queue::PurgeQueueError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PurgeQueueFluentBuilder {
    /// Creates a new `PurgeQueue`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PurgeQueue as a reference.
    pub fn as_input(&self) -> &crate::operation::purge_queue::builders::PurgeQueueInputBuilder {
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
        crate::operation::purge_queue::PurgeQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purge_queue::PurgeQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::purge_queue::PurgeQueue::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::purge_queue::PurgeQueue::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::purge_queue::PurgeQueueOutput,
        crate::operation::purge_queue::PurgeQueueError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The URL of the queue from which the <code>PurgeQueue</code> action deletes messages.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the queue from which the <code>PurgeQueue</code> action deletes messages.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the queue from which the <code>PurgeQueue</code> action deletes messages.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
}
