// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Optionally configure a Snowflake role. Otherwise the default user role will be used.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SnowflakeRoleConfiguration {
    /// <p>Enable Snowflake role</p>
    pub enabled: ::std::option::Option<bool>,
    /// <p>The Snowflake role you wish to configure</p>
    pub snowflake_role: ::std::option::Option<::std::string::String>,
}
impl SnowflakeRoleConfiguration {
    /// <p>Enable Snowflake role</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>The Snowflake role you wish to configure</p>
    pub fn snowflake_role(&self) -> ::std::option::Option<&str> {
        self.snowflake_role.as_deref()
    }
}
impl ::std::fmt::Debug for SnowflakeRoleConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SnowflakeRoleConfiguration");
        formatter.field("enabled", &self.enabled);
        formatter.field("snowflake_role", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SnowflakeRoleConfiguration {
    /// Creates a new builder-style object to manufacture [`SnowflakeRoleConfiguration`](crate::types::SnowflakeRoleConfiguration).
    pub fn builder() -> crate::types::builders::SnowflakeRoleConfigurationBuilder {
        crate::types::builders::SnowflakeRoleConfigurationBuilder::default()
    }
}

/// A builder for [`SnowflakeRoleConfiguration`](crate::types::SnowflakeRoleConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SnowflakeRoleConfigurationBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) snowflake_role: ::std::option::Option<::std::string::String>,
}
impl SnowflakeRoleConfigurationBuilder {
    /// <p>Enable Snowflake role</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable Snowflake role</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Enable Snowflake role</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// <p>The Snowflake role you wish to configure</p>
    pub fn snowflake_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snowflake_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Snowflake role you wish to configure</p>
    pub fn set_snowflake_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snowflake_role = input;
        self
    }
    /// <p>The Snowflake role you wish to configure</p>
    pub fn get_snowflake_role(&self) -> &::std::option::Option<::std::string::String> {
        &self.snowflake_role
    }
    /// Consumes the builder and constructs a [`SnowflakeRoleConfiguration`](crate::types::SnowflakeRoleConfiguration).
    pub fn build(self) -> crate::types::SnowflakeRoleConfiguration {
        crate::types::SnowflakeRoleConfiguration {
            enabled: self.enabled,
            snowflake_role: self.snowflake_role,
        }
    }
}
impl ::std::fmt::Debug for SnowflakeRoleConfigurationBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SnowflakeRoleConfigurationBuilder");
        formatter.field("enabled", &self.enabled);
        formatter.field("snowflake_role", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
