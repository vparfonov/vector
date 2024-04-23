// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMetricStreamInput {
    /// <p>The name of the metric stream to delete.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl DeleteMetricStreamInput {
    /// <p>The name of the metric stream to delete.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeleteMetricStreamInput {
    /// Creates a new builder-style object to manufacture [`DeleteMetricStreamInput`](crate::operation::delete_metric_stream::DeleteMetricStreamInput).
    pub fn builder() -> crate::operation::delete_metric_stream::builders::DeleteMetricStreamInputBuilder {
        crate::operation::delete_metric_stream::builders::DeleteMetricStreamInputBuilder::default()
    }
}

/// A builder for [`DeleteMetricStreamInput`](crate::operation::delete_metric_stream::DeleteMetricStreamInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteMetricStreamInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl DeleteMetricStreamInputBuilder {
    /// <p>The name of the metric stream to delete.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the metric stream to delete.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the metric stream to delete.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`DeleteMetricStreamInput`](crate::operation::delete_metric_stream::DeleteMetricStreamInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_metric_stream::DeleteMetricStreamInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_metric_stream::DeleteMetricStreamInput { name: self.name })
    }
}
