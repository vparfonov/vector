// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutQueryDefinitionInput {
    /// <p>A name for the query definition. If you are saving numerous query definitions, we recommend that you name them. This way, you can find the ones you want by using the first part of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>If you are updating a query definition, use this parameter to specify the ID of the query definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    /// <p>If you are creating a query definition, do not specify this parameter. CloudWatch generates a unique ID for the new query definition and include it in the response to this operation.</p>
    pub query_definition_id: ::std::option::Option<::std::string::String>,
    /// <p>Use this parameter to include specific log groups as part of your query definition.</p>
    /// <p>If you are updating a query definition and you omit this parameter, then the updated definition will contain no log groups.</p>
    pub log_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub query_string: ::std::option::Option<::std::string::String>,
    /// <p>Used as an idempotency token, to avoid returning an exception if the service receives the same request twice because of a network error.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl PutQueryDefinitionInput {
    /// <p>A name for the query definition. If you are saving numerous query definitions, we recommend that you name them. This way, you can find the ones you want by using the first part of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>If you are updating a query definition, use this parameter to specify the ID of the query definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    /// <p>If you are creating a query definition, do not specify this parameter. CloudWatch generates a unique ID for the new query definition and include it in the response to this operation.</p>
    pub fn query_definition_id(&self) -> ::std::option::Option<&str> {
        self.query_definition_id.as_deref()
    }
    /// <p>Use this parameter to include specific log groups as part of your query definition.</p>
    /// <p>If you are updating a query definition and you omit this parameter, then the updated definition will contain no log groups.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.log_group_names.is_none()`.
    pub fn log_group_names(&self) -> &[::std::string::String] {
        self.log_group_names.as_deref().unwrap_or_default()
    }
    /// <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub fn query_string(&self) -> ::std::option::Option<&str> {
        self.query_string.as_deref()
    }
    /// <p>Used as an idempotency token, to avoid returning an exception if the service receives the same request twice because of a network error.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl PutQueryDefinitionInput {
    /// Creates a new builder-style object to manufacture [`PutQueryDefinitionInput`](crate::operation::put_query_definition::PutQueryDefinitionInput).
    pub fn builder() -> crate::operation::put_query_definition::builders::PutQueryDefinitionInputBuilder {
        crate::operation::put_query_definition::builders::PutQueryDefinitionInputBuilder::default()
    }
}

/// A builder for [`PutQueryDefinitionInput`](crate::operation::put_query_definition::PutQueryDefinitionInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutQueryDefinitionInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) query_definition_id: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) query_string: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl PutQueryDefinitionInputBuilder {
    /// <p>A name for the query definition. If you are saving numerous query definitions, we recommend that you name them. This way, you can find the ones you want by using the first part of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the query definition. If you are saving numerous query definitions, we recommend that you name them. This way, you can find the ones you want by using the first part of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A name for the query definition. If you are saving numerous query definitions, we recommend that you name them. This way, you can find the ones you want by using the first part of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>If you are updating a query definition, use this parameter to specify the ID of the query definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    /// <p>If you are creating a query definition, do not specify this parameter. CloudWatch generates a unique ID for the new query definition and include it in the response to this operation.</p>
    pub fn query_definition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_definition_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If you are updating a query definition, use this parameter to specify the ID of the query definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    /// <p>If you are creating a query definition, do not specify this parameter. CloudWatch generates a unique ID for the new query definition and include it in the response to this operation.</p>
    pub fn set_query_definition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_definition_id = input;
        self
    }
    /// <p>If you are updating a query definition, use this parameter to specify the ID of the query definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    /// <p>If you are creating a query definition, do not specify this parameter. CloudWatch generates a unique ID for the new query definition and include it in the response to this operation.</p>
    pub fn get_query_definition_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.query_definition_id
    }
    /// Appends an item to `log_group_names`.
    ///
    /// To override the contents of this collection use [`set_log_group_names`](Self::set_log_group_names).
    ///
    /// <p>Use this parameter to include specific log groups as part of your query definition.</p>
    /// <p>If you are updating a query definition and you omit this parameter, then the updated definition will contain no log groups.</p>
    pub fn log_group_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.log_group_names.unwrap_or_default();
        v.push(input.into());
        self.log_group_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>Use this parameter to include specific log groups as part of your query definition.</p>
    /// <p>If you are updating a query definition and you omit this parameter, then the updated definition will contain no log groups.</p>
    pub fn set_log_group_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.log_group_names = input;
        self
    }
    /// <p>Use this parameter to include specific log groups as part of your query definition.</p>
    /// <p>If you are updating a query definition and you omit this parameter, then the updated definition will contain no log groups.</p>
    pub fn get_log_group_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.log_group_names
    }
    /// <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    /// This field is required.
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_string = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_string = input;
        self
    }
    /// <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub fn get_query_string(&self) -> &::std::option::Option<::std::string::String> {
        &self.query_string
    }
    /// <p>Used as an idempotency token, to avoid returning an exception if the service receives the same request twice because of a network error.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used as an idempotency token, to avoid returning an exception if the service receives the same request twice because of a network error.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Used as an idempotency token, to avoid returning an exception if the service receives the same request twice because of a network error.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Consumes the builder and constructs a [`PutQueryDefinitionInput`](crate::operation::put_query_definition::PutQueryDefinitionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_query_definition::PutQueryDefinitionInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::put_query_definition::PutQueryDefinitionInput {
            name: self.name,
            query_definition_id: self.query_definition_id,
            log_group_names: self.log_group_names,
            query_string: self.query_string,
            client_token: self.client_token,
        })
    }
}
