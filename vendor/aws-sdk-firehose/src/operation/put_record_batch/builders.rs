// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_record_batch::_put_record_batch_output::PutRecordBatchOutputBuilder;

pub use crate::operation::put_record_batch::_put_record_batch_input::PutRecordBatchInputBuilder;

impl crate::operation::put_record_batch::builders::PutRecordBatchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_record_batch::PutRecordBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_record_batch::PutRecordBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_record_batch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutRecordBatch`.
///
/// <p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <code>PutRecord</code>. Applications using these operations are referred to as producers.</p>
/// <p>Firehose accumulates and publishes a particular metric for a customer account in one minute intervals. It is possible that the bursts of incoming bytes/records ingested to a delivery stream last only for a few seconds. Due to this, the actual spikes in the traffic might not be fully visible in the customer's 1 minute CloudWatch metrics.</p>
/// <p>For information about service quota, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Firehose Quota</a>.</p>
/// <p>Each <code>PutRecordBatch</code> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before base64 encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p>
/// <p>You must specify the name of the delivery stream and the data record when using <code>PutRecord</code>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it could be a segment from a log file, geographic location data, website clickstream data, and so on.</p>
/// <p>Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p>
/// <p>The <code>PutRecordBatch</code> response includes a count of failed records, <code>FailedPutCount</code>, and an array of responses, <code>RequestResponses</code>. Even if the <code>PutRecordBatch</code> call succeeds, the value of <code>FailedPutCount</code> may be greater than 0, indicating that there are records for which the operation didn't succeed. Each entry in the <code>RequestResponses</code> array provides additional information about the processed record. It directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <code>RequestResponses</code> includes both successfully and unsuccessfully processed records. Firehose tries to process all records in each <code>PutRecordBatch</code> request. A single record failure does not stop the processing of subsequent records.</p>
/// <p>A successfully processed record includes a <code>RecordId</code> value, which is unique for the record. An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error, and is one of the following values: <code>ServiceUnavailableException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the error.</p>
/// <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <code>FailedPutCount</code> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p>
/// <p>If <code>PutRecordBatch</code> throws <code>ServiceUnavailableException</code>, the API is automatically reinvoked (retried) 3 times. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p>
/// <p>Re-invoking the Put API operations (for example, PutRecord and PutRecordBatch) can result in data duplicates. For larger data assets, allow for a longer time out before retrying Put API operations.</p>
/// <p>Data records sent to Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p><important>
/// <p>Don't concatenate two or more base64 strings to form the data fields of your records. Instead, concatenate the raw data, then perform base64 encoding.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutRecordBatchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_record_batch::builders::PutRecordBatchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_record_batch::PutRecordBatchOutput,
        crate::operation::put_record_batch::PutRecordBatchError,
    > for PutRecordBatchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_record_batch::PutRecordBatchOutput,
            crate::operation::put_record_batch::PutRecordBatchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutRecordBatchFluentBuilder {
    /// Creates a new `PutRecordBatchFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutRecordBatch as a reference.
    pub fn as_input(&self) -> &crate::operation::put_record_batch::builders::PutRecordBatchInputBuilder {
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
        crate::operation::put_record_batch::PutRecordBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_record_batch::PutRecordBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_record_batch::PutRecordBatch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_record_batch::PutRecordBatch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_record_batch::PutRecordBatchOutput,
        crate::operation::put_record_batch::PutRecordBatchError,
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
    /// <p>The name of the delivery stream.</p>
    pub fn delivery_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.delivery_stream_name(input.into());
        self
    }
    /// <p>The name of the delivery stream.</p>
    pub fn set_delivery_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_delivery_stream_name(input);
        self
    }
    /// <p>The name of the delivery stream.</p>
    pub fn get_delivery_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_delivery_stream_name()
    }
    ///
    /// Appends an item to `Records`.
    ///
    /// To override the contents of this collection use [`set_records`](Self::set_records).
    ///
    /// <p>One or more records.</p>
    pub fn records(mut self, input: crate::types::Record) -> Self {
        self.inner = self.inner.records(input);
        self
    }
    /// <p>One or more records.</p>
    pub fn set_records(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Record>>) -> Self {
        self.inner = self.inner.set_records(input);
        self
    }
    /// <p>One or more records.</p>
    pub fn get_records(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Record>> {
        self.inner.get_records()
    }
}
