// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies that you want Firehose to convert data from the JSON format to the Parquet or ORC format before writing it to Amazon S3. Firehose uses the serializer and deserializer that you specify, in addition to the column information from the Amazon Web Services Glue table, to deserialize your input data from JSON and then serialize it to the Parquet or ORC format. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/record-format-conversion.html">Firehose Record Format Conversion</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataFormatConversionConfiguration {
    /// <p>Specifies the Amazon Web Services Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub schema_configuration: ::std::option::Option<crate::types::SchemaConfiguration>,
    /// <p>Specifies the deserializer that you want Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub input_format_configuration: ::std::option::Option<crate::types::InputFormatConfiguration>,
    /// <p>Specifies the serializer that you want Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub output_format_configuration: ::std::option::Option<crate::types::OutputFormatConfiguration>,
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    pub enabled: ::std::option::Option<bool>,
}
impl DataFormatConversionConfiguration {
    /// <p>Specifies the Amazon Web Services Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn schema_configuration(&self) -> ::std::option::Option<&crate::types::SchemaConfiguration> {
        self.schema_configuration.as_ref()
    }
    /// <p>Specifies the deserializer that you want Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn input_format_configuration(&self) -> ::std::option::Option<&crate::types::InputFormatConfiguration> {
        self.input_format_configuration.as_ref()
    }
    /// <p>Specifies the serializer that you want Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn output_format_configuration(&self) -> ::std::option::Option<&crate::types::OutputFormatConfiguration> {
        self.output_format_configuration.as_ref()
    }
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl DataFormatConversionConfiguration {
    /// Creates a new builder-style object to manufacture [`DataFormatConversionConfiguration`](crate::types::DataFormatConversionConfiguration).
    pub fn builder() -> crate::types::builders::DataFormatConversionConfigurationBuilder {
        crate::types::builders::DataFormatConversionConfigurationBuilder::default()
    }
}

/// A builder for [`DataFormatConversionConfiguration`](crate::types::DataFormatConversionConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DataFormatConversionConfigurationBuilder {
    pub(crate) schema_configuration: ::std::option::Option<crate::types::SchemaConfiguration>,
    pub(crate) input_format_configuration: ::std::option::Option<crate::types::InputFormatConfiguration>,
    pub(crate) output_format_configuration: ::std::option::Option<crate::types::OutputFormatConfiguration>,
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl DataFormatConversionConfigurationBuilder {
    /// <p>Specifies the Amazon Web Services Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn schema_configuration(mut self, input: crate::types::SchemaConfiguration) -> Self {
        self.schema_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the Amazon Web Services Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn set_schema_configuration(mut self, input: ::std::option::Option<crate::types::SchemaConfiguration>) -> Self {
        self.schema_configuration = input;
        self
    }
    /// <p>Specifies the Amazon Web Services Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn get_schema_configuration(&self) -> &::std::option::Option<crate::types::SchemaConfiguration> {
        &self.schema_configuration
    }
    /// <p>Specifies the deserializer that you want Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn input_format_configuration(mut self, input: crate::types::InputFormatConfiguration) -> Self {
        self.input_format_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the deserializer that you want Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn set_input_format_configuration(mut self, input: ::std::option::Option<crate::types::InputFormatConfiguration>) -> Self {
        self.input_format_configuration = input;
        self
    }
    /// <p>Specifies the deserializer that you want Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn get_input_format_configuration(&self) -> &::std::option::Option<crate::types::InputFormatConfiguration> {
        &self.input_format_configuration
    }
    /// <p>Specifies the serializer that you want Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn output_format_configuration(mut self, input: crate::types::OutputFormatConfiguration) -> Self {
        self.output_format_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the serializer that you want Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn set_output_format_configuration(mut self, input: ::std::option::Option<crate::types::OutputFormatConfiguration>) -> Self {
        self.output_format_configuration = input;
        self
    }
    /// <p>Specifies the serializer that you want Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    pub fn get_output_format_configuration(&self) -> &::std::option::Option<crate::types::OutputFormatConfiguration> {
        &self.output_format_configuration
    }
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`DataFormatConversionConfiguration`](crate::types::DataFormatConversionConfiguration).
    pub fn build(self) -> crate::types::DataFormatConversionConfiguration {
        crate::types::DataFormatConversionConfiguration {
            schema_configuration: self.schema_configuration,
            input_format_configuration: self.input_format_configuration,
            output_format_configuration: self.output_format_configuration,
            enabled: self.enabled,
        }
    }
}
