// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for specifying the configuration for Amazon EventBridge.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EventBridgeConfiguration {}
impl EventBridgeConfiguration {
    /// Creates a new builder-style object to manufacture [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
    pub fn builder() -> crate::types::builders::EventBridgeConfigurationBuilder {
        crate::types::builders::EventBridgeConfigurationBuilder::default()
    }
}

/// A builder for [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EventBridgeConfigurationBuilder {}
impl EventBridgeConfigurationBuilder {
    /// Consumes the builder and constructs a [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
    pub fn build(self) -> crate::types::EventBridgeConfiguration {
        crate::types::EventBridgeConfiguration {}
    }
}
