// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the configuration for the domain cluster, such as the type and number of instances.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElasticsearchClusterConfig {
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    pub instance_type: ::std::option::Option<crate::types::EsPartitionInstanceType>,
    /// <p>The number of instances in the specified domain cluster.</p>
    pub instance_count: ::std::option::Option<i32>,
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    pub dedicated_master_enabled: ::std::option::Option<bool>,
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    pub zone_awareness_enabled: ::std::option::Option<bool>,
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    pub zone_awareness_config: ::std::option::Option<crate::types::ZoneAwarenessConfig>,
    /// <p>The instance type for a dedicated master node.</p>
    pub dedicated_master_type: ::std::option::Option<crate::types::EsPartitionInstanceType>,
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    pub dedicated_master_count: ::std::option::Option<i32>,
    /// <p>True to enable warm storage.</p>
    pub warm_enabled: ::std::option::Option<bool>,
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    pub warm_type: ::std::option::Option<crate::types::EsWarmPartitionInstanceType>,
    /// <p>The number of warm nodes in the cluster.</p>
    pub warm_count: ::std::option::Option<i32>,
    /// <p>Specifies the <code>ColdStorageOptions</code> config for Elasticsearch Domain</p>
    pub cold_storage_options: ::std::option::Option<crate::types::ColdStorageOptions>,
}
impl ElasticsearchClusterConfig {
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&crate::types::EsPartitionInstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The number of instances in the specified domain cluster.</p>
    pub fn instance_count(&self) -> ::std::option::Option<i32> {
        self.instance_count
    }
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    pub fn dedicated_master_enabled(&self) -> ::std::option::Option<bool> {
        self.dedicated_master_enabled
    }
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    pub fn zone_awareness_enabled(&self) -> ::std::option::Option<bool> {
        self.zone_awareness_enabled
    }
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    pub fn zone_awareness_config(&self) -> ::std::option::Option<&crate::types::ZoneAwarenessConfig> {
        self.zone_awareness_config.as_ref()
    }
    /// <p>The instance type for a dedicated master node.</p>
    pub fn dedicated_master_type(&self) -> ::std::option::Option<&crate::types::EsPartitionInstanceType> {
        self.dedicated_master_type.as_ref()
    }
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    pub fn dedicated_master_count(&self) -> ::std::option::Option<i32> {
        self.dedicated_master_count
    }
    /// <p>True to enable warm storage.</p>
    pub fn warm_enabled(&self) -> ::std::option::Option<bool> {
        self.warm_enabled
    }
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    pub fn warm_type(&self) -> ::std::option::Option<&crate::types::EsWarmPartitionInstanceType> {
        self.warm_type.as_ref()
    }
    /// <p>The number of warm nodes in the cluster.</p>
    pub fn warm_count(&self) -> ::std::option::Option<i32> {
        self.warm_count
    }
    /// <p>Specifies the <code>ColdStorageOptions</code> config for Elasticsearch Domain</p>
    pub fn cold_storage_options(&self) -> ::std::option::Option<&crate::types::ColdStorageOptions> {
        self.cold_storage_options.as_ref()
    }
}
impl ElasticsearchClusterConfig {
    /// Creates a new builder-style object to manufacture [`ElasticsearchClusterConfig`](crate::types::ElasticsearchClusterConfig).
    pub fn builder() -> crate::types::builders::ElasticsearchClusterConfigBuilder {
        crate::types::builders::ElasticsearchClusterConfigBuilder::default()
    }
}

/// A builder for [`ElasticsearchClusterConfig`](crate::types::ElasticsearchClusterConfig).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ElasticsearchClusterConfigBuilder {
    pub(crate) instance_type: ::std::option::Option<crate::types::EsPartitionInstanceType>,
    pub(crate) instance_count: ::std::option::Option<i32>,
    pub(crate) dedicated_master_enabled: ::std::option::Option<bool>,
    pub(crate) zone_awareness_enabled: ::std::option::Option<bool>,
    pub(crate) zone_awareness_config: ::std::option::Option<crate::types::ZoneAwarenessConfig>,
    pub(crate) dedicated_master_type: ::std::option::Option<crate::types::EsPartitionInstanceType>,
    pub(crate) dedicated_master_count: ::std::option::Option<i32>,
    pub(crate) warm_enabled: ::std::option::Option<bool>,
    pub(crate) warm_type: ::std::option::Option<crate::types::EsWarmPartitionInstanceType>,
    pub(crate) warm_count: ::std::option::Option<i32>,
    pub(crate) cold_storage_options: ::std::option::Option<crate::types::ColdStorageOptions>,
}
impl ElasticsearchClusterConfigBuilder {
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    pub fn instance_type(mut self, input: crate::types::EsPartitionInstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::EsPartitionInstanceType>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The instance type for an Elasticsearch cluster. UltraWarm instance types are not supported for data instances.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::EsPartitionInstanceType> {
        &self.instance_type
    }
    /// <p>The number of instances in the specified domain cluster.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of instances in the specified domain cluster.</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// <p>The number of instances in the specified domain cluster.</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        &self.instance_count
    }
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    pub fn dedicated_master_enabled(mut self, input: bool) -> Self {
        self.dedicated_master_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    pub fn set_dedicated_master_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dedicated_master_enabled = input;
        self
    }
    /// <p>A boolean value to indicate whether a dedicated master node is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-dedicatedmasternodes" target="_blank">About Dedicated Master Nodes</a> for more information.</p>
    pub fn get_dedicated_master_enabled(&self) -> &::std::option::Option<bool> {
        &self.dedicated_master_enabled
    }
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    pub fn zone_awareness_enabled(mut self, input: bool) -> Self {
        self.zone_awareness_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    pub fn set_zone_awareness_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.zone_awareness_enabled = input;
        self
    }
    /// <p>A boolean value to indicate whether zone awareness is enabled. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-managedomains.html#es-managedomains-zoneawareness" target="_blank">About Zone Awareness</a> for more information.</p>
    pub fn get_zone_awareness_enabled(&self) -> &::std::option::Option<bool> {
        &self.zone_awareness_enabled
    }
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    pub fn zone_awareness_config(mut self, input: crate::types::ZoneAwarenessConfig) -> Self {
        self.zone_awareness_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    pub fn set_zone_awareness_config(mut self, input: ::std::option::Option<crate::types::ZoneAwarenessConfig>) -> Self {
        self.zone_awareness_config = input;
        self
    }
    /// <p>Specifies the zone awareness configuration for a domain when zone awareness is enabled.</p>
    pub fn get_zone_awareness_config(&self) -> &::std::option::Option<crate::types::ZoneAwarenessConfig> {
        &self.zone_awareness_config
    }
    /// <p>The instance type for a dedicated master node.</p>
    pub fn dedicated_master_type(mut self, input: crate::types::EsPartitionInstanceType) -> Self {
        self.dedicated_master_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type for a dedicated master node.</p>
    pub fn set_dedicated_master_type(mut self, input: ::std::option::Option<crate::types::EsPartitionInstanceType>) -> Self {
        self.dedicated_master_type = input;
        self
    }
    /// <p>The instance type for a dedicated master node.</p>
    pub fn get_dedicated_master_type(&self) -> &::std::option::Option<crate::types::EsPartitionInstanceType> {
        &self.dedicated_master_type
    }
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    pub fn dedicated_master_count(mut self, input: i32) -> Self {
        self.dedicated_master_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    pub fn set_dedicated_master_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.dedicated_master_count = input;
        self
    }
    /// <p>Total number of dedicated master nodes, active and on standby, for the cluster.</p>
    pub fn get_dedicated_master_count(&self) -> &::std::option::Option<i32> {
        &self.dedicated_master_count
    }
    /// <p>True to enable warm storage.</p>
    pub fn warm_enabled(mut self, input: bool) -> Self {
        self.warm_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>True to enable warm storage.</p>
    pub fn set_warm_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.warm_enabled = input;
        self
    }
    /// <p>True to enable warm storage.</p>
    pub fn get_warm_enabled(&self) -> &::std::option::Option<bool> {
        &self.warm_enabled
    }
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    pub fn warm_type(mut self, input: crate::types::EsWarmPartitionInstanceType) -> Self {
        self.warm_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    pub fn set_warm_type(mut self, input: ::std::option::Option<crate::types::EsWarmPartitionInstanceType>) -> Self {
        self.warm_type = input;
        self
    }
    /// <p>The instance type for the Elasticsearch cluster's warm nodes.</p>
    pub fn get_warm_type(&self) -> &::std::option::Option<crate::types::EsWarmPartitionInstanceType> {
        &self.warm_type
    }
    /// <p>The number of warm nodes in the cluster.</p>
    pub fn warm_count(mut self, input: i32) -> Self {
        self.warm_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of warm nodes in the cluster.</p>
    pub fn set_warm_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.warm_count = input;
        self
    }
    /// <p>The number of warm nodes in the cluster.</p>
    pub fn get_warm_count(&self) -> &::std::option::Option<i32> {
        &self.warm_count
    }
    /// <p>Specifies the <code>ColdStorageOptions</code> config for Elasticsearch Domain</p>
    pub fn cold_storage_options(mut self, input: crate::types::ColdStorageOptions) -> Self {
        self.cold_storage_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code>ColdStorageOptions</code> config for Elasticsearch Domain</p>
    pub fn set_cold_storage_options(mut self, input: ::std::option::Option<crate::types::ColdStorageOptions>) -> Self {
        self.cold_storage_options = input;
        self
    }
    /// <p>Specifies the <code>ColdStorageOptions</code> config for Elasticsearch Domain</p>
    pub fn get_cold_storage_options(&self) -> &::std::option::Option<crate::types::ColdStorageOptions> {
        &self.cold_storage_options
    }
    /// Consumes the builder and constructs a [`ElasticsearchClusterConfig`](crate::types::ElasticsearchClusterConfig).
    pub fn build(self) -> crate::types::ElasticsearchClusterConfig {
        crate::types::ElasticsearchClusterConfig {
            instance_type: self.instance_type,
            instance_count: self.instance_count,
            dedicated_master_enabled: self.dedicated_master_enabled,
            zone_awareness_enabled: self.zone_awareness_enabled,
            zone_awareness_config: self.zone_awareness_config,
            dedicated_master_type: self.dedicated_master_type,
            dedicated_master_count: self.dedicated_master_count,
            warm_enabled: self.warm_enabled,
            warm_type: self.warm_type,
            warm_count: self.warm_count,
            cold_storage_options: self.cold_storage_options,
        }
    }
}
