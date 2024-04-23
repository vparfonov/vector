// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutDeliverySourceOutput {
    /// <p>A structure containing information about the delivery source that was just created or updated.</p>
    pub delivery_source: ::std::option::Option<crate::types::DeliverySource>,
    _request_id: Option<String>,
}
impl PutDeliverySourceOutput {
    /// <p>A structure containing information about the delivery source that was just created or updated.</p>
    pub fn delivery_source(&self) -> ::std::option::Option<&crate::types::DeliverySource> {
        self.delivery_source.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for PutDeliverySourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutDeliverySourceOutput {
    /// Creates a new builder-style object to manufacture [`PutDeliverySourceOutput`](crate::operation::put_delivery_source::PutDeliverySourceOutput).
    pub fn builder() -> crate::operation::put_delivery_source::builders::PutDeliverySourceOutputBuilder {
        crate::operation::put_delivery_source::builders::PutDeliverySourceOutputBuilder::default()
    }
}

/// A builder for [`PutDeliverySourceOutput`](crate::operation::put_delivery_source::PutDeliverySourceOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutDeliverySourceOutputBuilder {
    pub(crate) delivery_source: ::std::option::Option<crate::types::DeliverySource>,
    _request_id: Option<String>,
}
impl PutDeliverySourceOutputBuilder {
    /// <p>A structure containing information about the delivery source that was just created or updated.</p>
    pub fn delivery_source(mut self, input: crate::types::DeliverySource) -> Self {
        self.delivery_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure containing information about the delivery source that was just created or updated.</p>
    pub fn set_delivery_source(mut self, input: ::std::option::Option<crate::types::DeliverySource>) -> Self {
        self.delivery_source = input;
        self
    }
    /// <p>A structure containing information about the delivery source that was just created or updated.</p>
    pub fn get_delivery_source(&self) -> &::std::option::Option<crate::types::DeliverySource> {
        &self.delivery_source
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutDeliverySourceOutput`](crate::operation::put_delivery_source::PutDeliverySourceOutput).
    pub fn build(self) -> crate::operation::put_delivery_source::PutDeliverySourceOutput {
        crate::operation::put_delivery_source::PutDeliverySourceOutput {
            delivery_source: self.delivery_source,
            _request_id: self._request_id,
        }
    }
}
