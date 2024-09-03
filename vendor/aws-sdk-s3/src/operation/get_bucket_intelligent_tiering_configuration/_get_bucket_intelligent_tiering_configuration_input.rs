// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetBucketIntelligentTieringConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetBucketIntelligentTieringConfigurationInput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationInput).
    pub fn builder() -> crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationInputBuilder
    {
        crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetBucketIntelligentTieringConfigurationInput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBucketIntelligentTieringConfigurationInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetBucketIntelligentTieringConfigurationInputBuilder {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Consumes the builder and constructs a [`GetBucketIntelligentTieringConfigurationInput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationInput {
                bucket: self.bucket,
                id: self.id,
            },
        )
    }
}
