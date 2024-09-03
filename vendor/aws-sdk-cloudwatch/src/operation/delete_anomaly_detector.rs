// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `DeleteAnomalyDetector`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteAnomalyDetector;
impl DeleteAnomalyDetector {
    /// Creates a new `DeleteAnomalyDetector`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorInput,
    ) -> ::std::result::Result<
        crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("cloudwatch", "DeleteAnomalyDetector", input, runtime_plugins, stop_point)
            .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteAnomalyDetector {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteAnomalyDetector");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            DeleteAnomalyDetectorRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            DeleteAnomalyDetectorResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "DeleteAnomalyDetector",
            "cloudwatch",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteAnomalyDetector")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(DeleteAnomalyDetectorEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct DeleteAnomalyDetectorResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DeleteAnomalyDetectorResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_delete_anomaly_detector::de_delete_anomaly_detector_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_delete_anomaly_detector::de_delete_anomaly_detector_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct DeleteAnomalyDetectorRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for DeleteAnomalyDetectorRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_delete_anomaly_detector_input::ser_delete_anomaly_detector_input_input_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct DeleteAnomalyDetectorEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DeleteAnomalyDetectorEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "DeleteAnomalyDetectorEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<DeleteAnomalyDetectorInput>()
            .ok_or("failed to downcast to DeleteAnomalyDetectorInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

// The get_* functions below are generated from JMESPath expressions in the
// operationContextParams trait. They target the operation's input shape.

/// Error type for the `DeleteAnomalyDetectorError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum DeleteAnomalyDetectorError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(crate::types::error::InternalServiceFault),
    /// <p>Parameters were used together that cannot be used together.</p>
    InvalidParameterCombinationException(crate::types::error::InvalidParameterCombinationException),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValueException(crate::types::error::InvalidParameterValueException),
    /// <p>An input parameter that is required is missing.</p>
    MissingRequiredParameterException(crate::types::error::MissingRequiredParameterException),
    /// <p>The named resource does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DeleteAnomalyDetectorError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl DeleteAnomalyDetectorError {
    /// Creates the `DeleteAnomalyDetectorError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `DeleteAnomalyDetectorError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServiceFault(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterCombinationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterValueException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::MissingRequiredParameterException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `DeleteAnomalyDetectorError::InternalServiceFault`.
    pub fn is_internal_service_fault(&self) -> bool {
        matches!(self, Self::InternalServiceFault(_))
    }
    /// Returns `true` if the error kind is `DeleteAnomalyDetectorError::InvalidParameterCombinationException`.
    pub fn is_invalid_parameter_combination_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterCombinationException(_))
    }
    /// Returns `true` if the error kind is `DeleteAnomalyDetectorError::InvalidParameterValueException`.
    pub fn is_invalid_parameter_value_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterValueException(_))
    }
    /// Returns `true` if the error kind is `DeleteAnomalyDetectorError::MissingRequiredParameterException`.
    pub fn is_missing_required_parameter_exception(&self) -> bool {
        matches!(self, Self::MissingRequiredParameterException(_))
    }
    /// Returns `true` if the error kind is `DeleteAnomalyDetectorError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
}
impl ::std::error::Error for DeleteAnomalyDetectorError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalServiceFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterCombinationException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterValueException(_inner) => ::std::option::Option::Some(_inner),
            Self::MissingRequiredParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for DeleteAnomalyDetectorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalServiceFault(_inner) => _inner.fmt(f),
            Self::InvalidParameterCombinationException(_inner) => _inner.fmt(f),
            Self::InvalidParameterValueException(_inner) => _inner.fmt(f),
            Self::MissingRequiredParameterException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for DeleteAnomalyDetectorError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteAnomalyDetectorError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServiceFault(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterCombinationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterValueException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::MissingRequiredParameterException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for DeleteAnomalyDetectorError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::delete_anomaly_detector::DeleteAnomalyDetectorError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::delete_anomaly_detector::_delete_anomaly_detector_output::DeleteAnomalyDetectorOutput;

pub use crate::operation::delete_anomaly_detector::_delete_anomaly_detector_input::DeleteAnomalyDetectorInput;

mod _delete_anomaly_detector_input;

mod _delete_anomaly_detector_output;

/// Builders
pub mod builders;
