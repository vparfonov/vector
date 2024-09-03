// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterStreamConsumerInput {
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    pub consumer_name: ::std::option::Option<::std::string::String>,
}
impl RegisterStreamConsumerInput {
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    pub fn consumer_name(&self) -> ::std::option::Option<&str> {
        self.consumer_name.as_deref()
    }
}
impl RegisterStreamConsumerInput {
    /// Creates a new builder-style object to manufacture [`RegisterStreamConsumerInput`](crate::operation::register_stream_consumer::RegisterStreamConsumerInput).
    pub fn builder() -> crate::operation::register_stream_consumer::builders::RegisterStreamConsumerInputBuilder {
        crate::operation::register_stream_consumer::builders::RegisterStreamConsumerInputBuilder::default()
    }
}

/// A builder for [`RegisterStreamConsumerInput`](crate::operation::register_stream_consumer::RegisterStreamConsumerInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RegisterStreamConsumerInputBuilder {
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) consumer_name: ::std::option::Option<::std::string::String>,
}
impl RegisterStreamConsumerInputBuilder {
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    /// This field is required.
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    /// This field is required.
    pub fn consumer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.consumer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    pub fn set_consumer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.consumer_name = input;
        self
    }
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    pub fn get_consumer_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.consumer_name
    }
    /// Consumes the builder and constructs a [`RegisterStreamConsumerInput`](crate::operation::register_stream_consumer::RegisterStreamConsumerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_stream_consumer::RegisterStreamConsumerInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::register_stream_consumer::RegisterStreamConsumerInput {
            stream_arn: self.stream_arn,
            consumer_name: self.consumer_name,
        })
    }
}
