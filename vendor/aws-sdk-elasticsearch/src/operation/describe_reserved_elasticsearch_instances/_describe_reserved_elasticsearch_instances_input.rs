// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for parameters to <code>DescribeReservedElasticsearchInstances</code></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeReservedElasticsearchInstancesInput {
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    pub reserved_elasticsearch_instance_id: ::std::option::Option<::std::string::String>,
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeReservedElasticsearchInstancesInput {
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    pub fn reserved_elasticsearch_instance_id(&self) -> ::std::option::Option<&str> {
        self.reserved_elasticsearch_instance_id.as_deref()
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeReservedElasticsearchInstancesInput {
    /// Creates a new builder-style object to manufacture [`DescribeReservedElasticsearchInstancesInput`](crate::operation::describe_reserved_elasticsearch_instances::DescribeReservedElasticsearchInstancesInput).
    pub fn builder() -> crate::operation::describe_reserved_elasticsearch_instances::builders::DescribeReservedElasticsearchInstancesInputBuilder {
        crate::operation::describe_reserved_elasticsearch_instances::builders::DescribeReservedElasticsearchInstancesInputBuilder::default()
    }
}

/// A builder for [`DescribeReservedElasticsearchInstancesInput`](crate::operation::describe_reserved_elasticsearch_instances::DescribeReservedElasticsearchInstancesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeReservedElasticsearchInstancesInputBuilder {
    pub(crate) reserved_elasticsearch_instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeReservedElasticsearchInstancesInputBuilder {
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    pub fn reserved_elasticsearch_instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reserved_elasticsearch_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    pub fn set_reserved_elasticsearch_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reserved_elasticsearch_instance_id = input;
        self
    }
    /// <p>The reserved instance identifier filter value. Use this parameter to show only the reservation that matches the specified reserved Elasticsearch instance ID.</p>
    pub fn get_reserved_elasticsearch_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.reserved_elasticsearch_instance_id
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Set this value to limit the number of results returned. If not specified, defaults to 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>NextToken should be sent in case if earlier API call produced result containing NextToken. It is used for pagination.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`DescribeReservedElasticsearchInstancesInput`](crate::operation::describe_reserved_elasticsearch_instances::DescribeReservedElasticsearchInstancesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_reserved_elasticsearch_instances::DescribeReservedElasticsearchInstancesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_reserved_elasticsearch_instances::DescribeReservedElasticsearchInstancesInput {
                reserved_elasticsearch_instance_id: self.reserved_elasticsearch_instance_id,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
