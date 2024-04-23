// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateElasticsearchDomainInput {
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub elasticsearch_version: ::std::option::Option<::std::string::String>,
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    pub elasticsearch_cluster_config: ::std::option::Option<crate::types::ElasticsearchClusterConfig>,
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    pub ebs_options: ::std::option::Option<crate::types::EbsOptions>,
    /// <p> IAM access policy as a JSON-formatted string.</p>
    pub access_policies: ::std::option::Option<::std::string::String>,
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    pub snapshot_options: ::std::option::Option<crate::types::SnapshotOptions>,
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub vpc_options: ::std::option::Option<crate::types::VpcOptions>,
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub cognito_options: ::std::option::Option<crate::types::CognitoOptions>,
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub encryption_at_rest_options: ::std::option::Option<crate::types::EncryptionAtRestOptions>,
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub node_to_node_encryption_options: ::std::option::Option<crate::types::NodeToNodeEncryptionOptions>,
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub advanced_options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub log_publishing_options: ::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>>,
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub domain_endpoint_options: ::std::option::Option<crate::types::DomainEndpointOptions>,
    /// <p>Specifies advanced security options.</p>
    pub advanced_security_options: ::std::option::Option<crate::types::AdvancedSecurityOptionsInput>,
    /// <p>Specifies Auto-Tune options.</p>
    pub auto_tune_options: ::std::option::Option<crate::types::AutoTuneOptionsInput>,
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub tag_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateElasticsearchDomainInput {
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn elasticsearch_version(&self) -> ::std::option::Option<&str> {
        self.elasticsearch_version.as_deref()
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    pub fn elasticsearch_cluster_config(&self) -> ::std::option::Option<&crate::types::ElasticsearchClusterConfig> {
        self.elasticsearch_cluster_config.as_ref()
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    pub fn ebs_options(&self) -> ::std::option::Option<&crate::types::EbsOptions> {
        self.ebs_options.as_ref()
    }
    /// <p> IAM access policy as a JSON-formatted string.</p>
    pub fn access_policies(&self) -> ::std::option::Option<&str> {
        self.access_policies.as_deref()
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    pub fn snapshot_options(&self) -> ::std::option::Option<&crate::types::SnapshotOptions> {
        self.snapshot_options.as_ref()
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn vpc_options(&self) -> ::std::option::Option<&crate::types::VpcOptions> {
        self.vpc_options.as_ref()
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn cognito_options(&self) -> ::std::option::Option<&crate::types::CognitoOptions> {
        self.cognito_options.as_ref()
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn encryption_at_rest_options(&self) -> ::std::option::Option<&crate::types::EncryptionAtRestOptions> {
        self.encryption_at_rest_options.as_ref()
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn node_to_node_encryption_options(&self) -> ::std::option::Option<&crate::types::NodeToNodeEncryptionOptions> {
        self.node_to_node_encryption_options.as_ref()
    }
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn advanced_options(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.advanced_options.as_ref()
    }
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn log_publishing_options(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>> {
        self.log_publishing_options.as_ref()
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn domain_endpoint_options(&self) -> ::std::option::Option<&crate::types::DomainEndpointOptions> {
        self.domain_endpoint_options.as_ref()
    }
    /// <p>Specifies advanced security options.</p>
    pub fn advanced_security_options(&self) -> ::std::option::Option<&crate::types::AdvancedSecurityOptionsInput> {
        self.advanced_security_options.as_ref()
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn auto_tune_options(&self) -> ::std::option::Option<&crate::types::AutoTuneOptionsInput> {
        self.auto_tune_options.as_ref()
    }
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_list.is_none()`.
    pub fn tag_list(&self) -> &[crate::types::Tag] {
        self.tag_list.as_deref().unwrap_or_default()
    }
}
impl CreateElasticsearchDomainInput {
    /// Creates a new builder-style object to manufacture [`CreateElasticsearchDomainInput`](crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainInput).
    pub fn builder() -> crate::operation::create_elasticsearch_domain::builders::CreateElasticsearchDomainInputBuilder {
        crate::operation::create_elasticsearch_domain::builders::CreateElasticsearchDomainInputBuilder::default()
    }
}

/// A builder for [`CreateElasticsearchDomainInput`](crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateElasticsearchDomainInputBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) elasticsearch_version: ::std::option::Option<::std::string::String>,
    pub(crate) elasticsearch_cluster_config: ::std::option::Option<crate::types::ElasticsearchClusterConfig>,
    pub(crate) ebs_options: ::std::option::Option<crate::types::EbsOptions>,
    pub(crate) access_policies: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_options: ::std::option::Option<crate::types::SnapshotOptions>,
    pub(crate) vpc_options: ::std::option::Option<crate::types::VpcOptions>,
    pub(crate) cognito_options: ::std::option::Option<crate::types::CognitoOptions>,
    pub(crate) encryption_at_rest_options: ::std::option::Option<crate::types::EncryptionAtRestOptions>,
    pub(crate) node_to_node_encryption_options: ::std::option::Option<crate::types::NodeToNodeEncryptionOptions>,
    pub(crate) advanced_options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) log_publishing_options: ::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>>,
    pub(crate) domain_endpoint_options: ::std::option::Option<crate::types::DomainEndpointOptions>,
    pub(crate) advanced_security_options: ::std::option::Option<crate::types::AdvancedSecurityOptionsInput>,
    pub(crate) auto_tune_options: ::std::option::Option<crate::types::AutoTuneOptionsInput>,
    pub(crate) tag_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateElasticsearchDomainInputBuilder {
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    /// This field is required.
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_name
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn elasticsearch_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.elasticsearch_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn set_elasticsearch_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.elasticsearch_version = input;
        self
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn get_elasticsearch_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.elasticsearch_version
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    pub fn elasticsearch_cluster_config(mut self, input: crate::types::ElasticsearchClusterConfig) -> Self {
        self.elasticsearch_cluster_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    pub fn set_elasticsearch_cluster_config(mut self, input: ::std::option::Option<crate::types::ElasticsearchClusterConfig>) -> Self {
        self.elasticsearch_cluster_config = input;
        self
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p>
    pub fn get_elasticsearch_cluster_config(&self) -> &::std::option::Option<crate::types::ElasticsearchClusterConfig> {
        &self.elasticsearch_cluster_config
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    pub fn ebs_options(mut self, input: crate::types::EbsOptions) -> Self {
        self.ebs_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    pub fn set_ebs_options(mut self, input: ::std::option::Option<crate::types::EbsOptions>) -> Self {
        self.ebs_options = input;
        self
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p>
    pub fn get_ebs_options(&self) -> &::std::option::Option<crate::types::EbsOptions> {
        &self.ebs_options
    }
    /// <p> IAM access policy as a JSON-formatted string.</p>
    pub fn access_policies(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.access_policies = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> IAM access policy as a JSON-formatted string.</p>
    pub fn set_access_policies(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.access_policies = input;
        self
    }
    /// <p> IAM access policy as a JSON-formatted string.</p>
    pub fn get_access_policies(&self) -> &::std::option::Option<::std::string::String> {
        &self.access_policies
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    pub fn snapshot_options(mut self, input: crate::types::SnapshotOptions) -> Self {
        self.snapshot_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    pub fn set_snapshot_options(mut self, input: ::std::option::Option<crate::types::SnapshotOptions>) -> Self {
        self.snapshot_options = input;
        self
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p>
    pub fn get_snapshot_options(&self) -> &::std::option::Option<crate::types::SnapshotOptions> {
        &self.snapshot_options
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn vpc_options(mut self, input: crate::types::VpcOptions) -> Self {
        self.vpc_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn set_vpc_options(mut self, input: ::std::option::Option<crate::types::VpcOptions>) -> Self {
        self.vpc_options = input;
        self
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn get_vpc_options(&self) -> &::std::option::Option<crate::types::VpcOptions> {
        &self.vpc_options
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn cognito_options(mut self, input: crate::types::CognitoOptions) -> Self {
        self.cognito_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn set_cognito_options(mut self, input: ::std::option::Option<crate::types::CognitoOptions>) -> Self {
        self.cognito_options = input;
        self
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn get_cognito_options(&self) -> &::std::option::Option<crate::types::CognitoOptions> {
        &self.cognito_options
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn encryption_at_rest_options(mut self, input: crate::types::EncryptionAtRestOptions) -> Self {
        self.encryption_at_rest_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn set_encryption_at_rest_options(mut self, input: ::std::option::Option<crate::types::EncryptionAtRestOptions>) -> Self {
        self.encryption_at_rest_options = input;
        self
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn get_encryption_at_rest_options(&self) -> &::std::option::Option<crate::types::EncryptionAtRestOptions> {
        &self.encryption_at_rest_options
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn node_to_node_encryption_options(mut self, input: crate::types::NodeToNodeEncryptionOptions) -> Self {
        self.node_to_node_encryption_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn set_node_to_node_encryption_options(mut self, input: ::std::option::Option<crate::types::NodeToNodeEncryptionOptions>) -> Self {
        self.node_to_node_encryption_options = input;
        self
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn get_node_to_node_encryption_options(&self) -> &::std::option::Option<crate::types::NodeToNodeEncryptionOptions> {
        &self.node_to_node_encryption_options
    }
    /// Adds a key-value pair to `advanced_options`.
    ///
    /// To override the contents of this collection use [`set_advanced_options`](Self::set_advanced_options).
    ///
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn advanced_options(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.advanced_options.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.advanced_options = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn set_advanced_options(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.advanced_options = input;
        self
    }
    /// <p> Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn get_advanced_options(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.advanced_options
    }
    /// Adds a key-value pair to `log_publishing_options`.
    ///
    /// To override the contents of this collection use [`set_log_publishing_options`](Self::set_log_publishing_options).
    ///
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn log_publishing_options(mut self, k: crate::types::LogType, v: crate::types::LogPublishingOption) -> Self {
        let mut hash_map = self.log_publishing_options.unwrap_or_default();
        hash_map.insert(k, v);
        self.log_publishing_options = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn set_log_publishing_options(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>>,
    ) -> Self {
        self.log_publishing_options = input;
        self
    }
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn get_log_publishing_options(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>> {
        &self.log_publishing_options
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn domain_endpoint_options(mut self, input: crate::types::DomainEndpointOptions) -> Self {
        self.domain_endpoint_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn set_domain_endpoint_options(mut self, input: ::std::option::Option<crate::types::DomainEndpointOptions>) -> Self {
        self.domain_endpoint_options = input;
        self
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn get_domain_endpoint_options(&self) -> &::std::option::Option<crate::types::DomainEndpointOptions> {
        &self.domain_endpoint_options
    }
    /// <p>Specifies advanced security options.</p>
    pub fn advanced_security_options(mut self, input: crate::types::AdvancedSecurityOptionsInput) -> Self {
        self.advanced_security_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies advanced security options.</p>
    pub fn set_advanced_security_options(mut self, input: ::std::option::Option<crate::types::AdvancedSecurityOptionsInput>) -> Self {
        self.advanced_security_options = input;
        self
    }
    /// <p>Specifies advanced security options.</p>
    pub fn get_advanced_security_options(&self) -> &::std::option::Option<crate::types::AdvancedSecurityOptionsInput> {
        &self.advanced_security_options
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn auto_tune_options(mut self, input: crate::types::AutoTuneOptionsInput) -> Self {
        self.auto_tune_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn set_auto_tune_options(mut self, input: ::std::option::Option<crate::types::AutoTuneOptionsInput>) -> Self {
        self.auto_tune_options = input;
        self
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn get_auto_tune_options(&self) -> &::std::option::Option<crate::types::AutoTuneOptionsInput> {
        &self.auto_tune_options
    }
    /// Appends an item to `tag_list`.
    ///
    /// To override the contents of this collection use [`set_tag_list`](Self::set_tag_list).
    ///
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn tag_list(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tag_list.unwrap_or_default();
        v.push(input);
        self.tag_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn set_tag_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tag_list = input;
        self
    }
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn get_tag_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tag_list
    }
    /// Consumes the builder and constructs a [`CreateElasticsearchDomainInput`](crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainInput {
            domain_name: self.domain_name,
            elasticsearch_version: self.elasticsearch_version,
            elasticsearch_cluster_config: self.elasticsearch_cluster_config,
            ebs_options: self.ebs_options,
            access_policies: self.access_policies,
            snapshot_options: self.snapshot_options,
            vpc_options: self.vpc_options,
            cognito_options: self.cognito_options,
            encryption_at_rest_options: self.encryption_at_rest_options,
            node_to_node_encryption_options: self.node_to_node_encryption_options,
            advanced_options: self.advanced_options,
            log_publishing_options: self.log_publishing_options,
            domain_endpoint_options: self.domain_endpoint_options,
            advanced_security_options: self.advanced_security_options,
            auto_tune_options: self.auto_tune_options,
            tag_list: self.tag_list,
        })
    }
}
