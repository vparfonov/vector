// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Container for the parameters returned by <code> <code>ListElasticsearchInstanceTypes</code> </code> operation. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListElasticsearchInstanceTypesOutput {
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    pub elasticsearch_instance_types: ::std::option::Option<::std::vec::Vec<crate::types::EsPartitionInstanceType>>,
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListElasticsearchInstanceTypesOutput {
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.elasticsearch_instance_types.is_none()`.
    pub fn elasticsearch_instance_types(&self) -> &[crate::types::EsPartitionInstanceType] {
        self.elasticsearch_instance_types.as_deref().unwrap_or_default()
    }
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListElasticsearchInstanceTypesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListElasticsearchInstanceTypesOutput {
    /// Creates a new builder-style object to manufacture [`ListElasticsearchInstanceTypesOutput`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput).
    pub fn builder() -> crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesOutputBuilder {
        crate::operation::list_elasticsearch_instance_types::builders::ListElasticsearchInstanceTypesOutputBuilder::default()
    }
}

/// A builder for [`ListElasticsearchInstanceTypesOutput`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListElasticsearchInstanceTypesOutputBuilder {
    pub(crate) elasticsearch_instance_types: ::std::option::Option<::std::vec::Vec<crate::types::EsPartitionInstanceType>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListElasticsearchInstanceTypesOutputBuilder {
    /// Appends an item to `elasticsearch_instance_types`.
    ///
    /// To override the contents of this collection use [`set_elasticsearch_instance_types`](Self::set_elasticsearch_instance_types).
    ///
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    pub fn elasticsearch_instance_types(mut self, input: crate::types::EsPartitionInstanceType) -> Self {
        let mut v = self.elasticsearch_instance_types.unwrap_or_default();
        v.push(input);
        self.elasticsearch_instance_types = ::std::option::Option::Some(v);
        self
    }
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    pub fn set_elasticsearch_instance_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EsPartitionInstanceType>>) -> Self {
        self.elasticsearch_instance_types = input;
        self
    }
    /// <p> List of instance types supported by Amazon Elasticsearch service for given <code> <code>ElasticsearchVersion</code> </code> </p>
    pub fn get_elasticsearch_instance_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EsPartitionInstanceType>> {
        &self.elasticsearch_instance_types
    }
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>In case if there are more results available NextToken would be present, make further request to the same API with received NextToken to paginate remaining results. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListElasticsearchInstanceTypesOutput`](crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput).
    pub fn build(self) -> crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput {
        crate::operation::list_elasticsearch_instance_types::ListElasticsearchInstanceTypesOutput {
            elasticsearch_instance_types: self.elasticsearch_instance_types,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
