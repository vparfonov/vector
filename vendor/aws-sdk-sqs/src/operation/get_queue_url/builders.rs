// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_queue_url::_get_queue_url_output::GetQueueUrlOutputBuilder;

pub use crate::operation::get_queue_url::_get_queue_url_input::GetQueueUrlInputBuilder;

impl crate::operation::get_queue_url::builders::GetQueueUrlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_queue_url::GetQueueUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_queue_url::GetQueueUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_queue_url();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetQueueUrl`.
///
/// <p>Returns the URL of an existing Amazon SQS queue.</p>
/// <p>To access a queue that belongs to another AWS account, use the <code>QueueOwnerAWSAccountId</code> parameter to specify the account ID of the queue's owner. The queue's owner must grant you permission to access the queue. For more information about shared queue access, see <code> <code>AddPermission</code> </code> or see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-writing-an-sqs-policy.html#write-messages-to-shared-queue">Allow Developers to Write Messages to a Shared Queue</a> in the <i>Amazon SQS Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetQueueUrlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_queue_url::builders::GetQueueUrlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_queue_url::GetQueueUrlOutput,
        crate::operation::get_queue_url::GetQueueUrlError,
    > for GetQueueUrlFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_queue_url::GetQueueUrlOutput,
            crate::operation::get_queue_url::GetQueueUrlError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetQueueUrlFluentBuilder {
    /// Creates a new `GetQueueUrlFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetQueueUrl as a reference.
    pub fn as_input(&self) -> &crate::operation::get_queue_url::builders::GetQueueUrlInputBuilder {
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
        crate::operation::get_queue_url::GetQueueUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_queue_url::GetQueueUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_queue_url::GetQueueUrl::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_queue_url::GetQueueUrl::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_queue_url::GetQueueUrlOutput,
        crate::operation::get_queue_url::GetQueueUrlError,
        Self,
    > {
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
    /// <p>The name of the queue whose URL must be fetched. Maximum 80 characters. Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_name(input.into());
        self
    }
    /// <p>The name of the queue whose URL must be fetched. Maximum 80 characters. Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_name(input);
        self
    }
    /// <p>The name of the queue whose URL must be fetched. Maximum 80 characters. Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_name()
    }
    /// <p>The Amazon Web Services account ID of the account that created the queue.</p>
    pub fn queue_owner_aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_owner_aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the account that created the queue.</p>
    pub fn set_queue_owner_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_owner_aws_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID of the account that created the queue.</p>
    pub fn get_queue_owner_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_owner_aws_account_id()
    }
}
