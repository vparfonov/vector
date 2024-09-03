// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
// Loading the partition JSON is expensive since it involves many regex compilations,
// so cache the result so that it only need to be paid for the first constructed client.
pub(crate) static DEFAULT_PARTITION_RESOLVER: ::once_cell::sync::Lazy<crate::endpoint_lib::partition::PartitionResolver> =
    ::once_cell::sync::Lazy::new(|| match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
        Ok(partitions) => {
            ::tracing::debug!("loading custom partitions located at {partitions}");
            let partition_dot_json = std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect("valid JSON")
        }
        _ => {
            ::tracing::debug!("loading default partitions");
            crate::endpoint_lib::partition::PartitionResolver::new_from_json(b"{\"partitions\":[{\"id\":\"aws\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-east-1\",\"name\":\"aws\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^(us|eu|ap|sa|ca|me|af|il)\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"af-south-1\":{\"description\":\"Africa (Cape Town)\"},\"ap-east-1\":{\"description\":\"Asia Pacific (Hong Kong)\"},\"ap-northeast-1\":{\"description\":\"Asia Pacific (Tokyo)\"},\"ap-northeast-2\":{\"description\":\"Asia Pacific (Seoul)\"},\"ap-northeast-3\":{\"description\":\"Asia Pacific (Osaka)\"},\"ap-south-1\":{\"description\":\"Asia Pacific (Mumbai)\"},\"ap-south-2\":{\"description\":\"Asia Pacific (Hyderabad)\"},\"ap-southeast-1\":{\"description\":\"Asia Pacific (Singapore)\"},\"ap-southeast-2\":{\"description\":\"Asia Pacific (Sydney)\"},\"ap-southeast-3\":{\"description\":\"Asia Pacific (Jakarta)\"},\"ap-southeast-4\":{\"description\":\"Asia Pacific (Melbourne)\"},\"ap-southeast-5\":{\"description\":\"Asia Pacific (Malaysia)\"},\"aws-global\":{\"description\":\"AWS Standard global region\"},\"ca-central-1\":{\"description\":\"Canada (Central)\"},\"ca-west-1\":{\"description\":\"Canada West (Calgary)\"},\"eu-central-1\":{\"description\":\"Europe (Frankfurt)\"},\"eu-central-2\":{\"description\":\"Europe (Zurich)\"},\"eu-north-1\":{\"description\":\"Europe (Stockholm)\"},\"eu-south-1\":{\"description\":\"Europe (Milan)\"},\"eu-south-2\":{\"description\":\"Europe (Spain)\"},\"eu-west-1\":{\"description\":\"Europe (Ireland)\"},\"eu-west-2\":{\"description\":\"Europe (London)\"},\"eu-west-3\":{\"description\":\"Europe (Paris)\"},\"il-central-1\":{\"description\":\"Israel (Tel Aviv)\"},\"me-central-1\":{\"description\":\"Middle East (UAE)\"},\"me-south-1\":{\"description\":\"Middle East (Bahrain)\"},\"sa-east-1\":{\"description\":\"South America (Sao Paulo)\"},\"us-east-1\":{\"description\":\"US East (N. Virginia)\"},\"us-east-2\":{\"description\":\"US East (Ohio)\"},\"us-west-1\":{\"description\":\"US West (N. California)\"},\"us-west-2\":{\"description\":\"US West (Oregon)\"}}},{\"id\":\"aws-cn\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com.cn\",\"dualStackDnsSuffix\":\"api.amazonwebservices.com.cn\",\"implicitGlobalRegion\":\"cn-northwest-1\",\"name\":\"aws-cn\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^cn\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-cn-global\":{\"description\":\"AWS China global region\"},\"cn-north-1\":{\"description\":\"China (Beijing)\"},\"cn-northwest-1\":{\"description\":\"China (Ningxia)\"}}},{\"id\":\"aws-us-gov\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-gov-west-1\",\"name\":\"aws-us-gov\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-gov\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-us-gov-global\":{\"description\":\"AWS GovCloud (US) global region\"},\"us-gov-east-1\":{\"description\":\"AWS GovCloud (US-East)\"},\"us-gov-west-1\":{\"description\":\"AWS GovCloud (US-West)\"}}},{\"id\":\"aws-iso\",\"outputs\":{\"dnsSuffix\":\"c2s.ic.gov\",\"dualStackDnsSuffix\":\"c2s.ic.gov\",\"implicitGlobalRegion\":\"us-iso-east-1\",\"name\":\"aws-iso\",\"supportsDualStack\":false,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-iso\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-global\":{\"description\":\"AWS ISO (US) global region\"},\"us-iso-east-1\":{\"description\":\"US ISO East\"},\"us-iso-west-1\":{\"description\":\"US ISO WEST\"}}},{\"id\":\"aws-iso-b\",\"outputs\":{\"dnsSuffix\":\"sc2s.sgov.gov\",\"dualStackDnsSuffix\":\"sc2s.sgov.gov\",\"implicitGlobalRegion\":\"us-isob-east-1\",\"name\":\"aws-iso-b\",\"supportsDualStack\":false,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isob\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-b-global\":{\"description\":\"AWS ISOB (US) global region\"},\"us-isob-east-1\":{\"description\":\"US ISOB East (Ohio)\"}}},{\"id\":\"aws-iso-e\",\"outputs\":{\"dnsSuffix\":\"cloud.adc-e.uk\",\"dualStackDnsSuffix\":\"cloud.adc-e.uk\",\"implicitGlobalRegion\":\"eu-isoe-west-1\",\"name\":\"aws-iso-e\",\"supportsDualStack\":false,\"supportsFIPS\":true},\"regionRegex\":\"^eu\\\\-isoe\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"eu-isoe-west-1\":{\"description\":\"EU ISOE West\"}}},{\"id\":\"aws-iso-f\",\"outputs\":{\"dnsSuffix\":\"csp.hci.ic.gov\",\"dualStackDnsSuffix\":\"csp.hci.ic.gov\",\"implicitGlobalRegion\":\"us-isof-south-1\",\"name\":\"aws-iso-f\",\"supportsDualStack\":false,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isof\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{}}],\"version\":\"1.1\"}").expect("valid JSON")
        }
    });

pub(crate) mod diagnostic;

pub(crate) mod partition;

pub(crate) mod arn;

pub(crate) mod host;
