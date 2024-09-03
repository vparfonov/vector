// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDeliveryStreams`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of delivery streams to list. The default value is 10.</p><br>
    ///   - [`delivery_stream_type(DeliveryStreamType)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::delivery_stream_type) / [`set_delivery_stream_type(Option<DeliveryStreamType>)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::set_delivery_stream_type):<br>required: **false**<br><p>The delivery stream type. This can be one of the following values:</p> <ul>  <li>   <p><code>DirectPut</code>: Provider applications access the delivery stream directly.</p></li>  <li>   <p><code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p></li> </ul> <p>This parameter is optional. If this parameter is omitted, delivery streams of all types are returned.</p><br>
    ///   - [`exclusive_start_delivery_stream_name(impl Into<String>)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::exclusive_start_delivery_stream_name) / [`set_exclusive_start_delivery_stream_name(Option<String>)`](crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::set_exclusive_start_delivery_stream_name):<br>required: **false**<br><p>The list of delivery streams returned by this call to <code>ListDeliveryStreams</code> will start with the delivery stream whose name comes alphabetically immediately after the name you specify in <code>ExclusiveStartDeliveryStreamName</code>.</p><br>
    /// - On success, responds with [`ListDeliveryStreamsOutput`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput) with field(s):
    ///   - [`delivery_stream_names(Vec::<String>)`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput::delivery_stream_names): <p>The names of the delivery streams.</p>
    ///   - [`has_more_delivery_streams(bool)`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput::has_more_delivery_streams): <p>Indicates whether there are more delivery streams available to list.</p>
    /// - On failure, responds with [`SdkError<ListDeliveryStreamsError>`](crate::operation::list_delivery_streams::ListDeliveryStreamsError)
    pub fn list_delivery_streams(&self) -> crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder {
        crate::operation::list_delivery_streams::builders::ListDeliveryStreamsFluentBuilder::new(self.handle.clone())
    }
}
