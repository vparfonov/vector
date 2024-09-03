// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Status of the advanced options for the specified Elasticsearch domain. Currently, the following advanced options are available:</p>
/// <ul>
/// <li>Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</li>
/// <li>Option to specify the percentage of heap space that is allocated to field data. By default, this setting is unbounded.</li>
/// </ul>
/// <p>For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options">Configuring Advanced Options</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AdvancedOptionsStatus {
    /// <p>Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    pub options: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    /// <p>Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    pub status: ::std::option::Option<crate::types::OptionStatus>,
}
impl AdvancedOptionsStatus {
    /// <p>Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    pub fn options(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.options
    }
    /// <p>Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::OptionStatus> {
        self.status.as_ref()
    }
}
impl AdvancedOptionsStatus {
    /// Creates a new builder-style object to manufacture [`AdvancedOptionsStatus`](crate::types::AdvancedOptionsStatus).
    pub fn builder() -> crate::types::builders::AdvancedOptionsStatusBuilder {
        crate::types::builders::AdvancedOptionsStatusBuilder::default()
    }
}

/// A builder for [`AdvancedOptionsStatus`](crate::types::AdvancedOptionsStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AdvancedOptionsStatusBuilder {
    pub(crate) options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) status: ::std::option::Option<crate::types::OptionStatus>,
}
impl AdvancedOptionsStatusBuilder {
    /// Adds a key-value pair to `options`.
    ///
    /// To override the contents of this collection use [`set_options`](Self::set_options).
    ///
    /// <p>Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    pub fn options(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.options.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.options = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    pub fn set_options(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.options = input;
        self
    }
    /// <p>Specifies the status of advanced options for the specified Elasticsearch domain.</p>
    pub fn get_options(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.options
    }
    /// <p>Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::OptionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::OptionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies the status of <code>OptionStatus</code> for advanced options for the specified Elasticsearch domain.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::OptionStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`AdvancedOptionsStatus`](crate::types::AdvancedOptionsStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`options`](crate::types::builders::AdvancedOptionsStatusBuilder::options)
    pub fn build(self) -> ::std::result::Result<crate::types::AdvancedOptionsStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AdvancedOptionsStatus {
            options: self.options.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "options",
                    "options was not specified but it is required when building AdvancedOptionsStatus",
                )
            })?,
            status: self.status,
        })
    }
}
