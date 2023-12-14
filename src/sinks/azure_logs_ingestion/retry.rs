use std::marker::PhantomData;
use http::{header, StatusCode};
use crate::http::HttpError;
use reqwest::blocking::Client;
use vrl::core::Value;
use crate::sinks::util::retries::RetryLogic;

// use aws_sdk_cloudwatchlogs::error::{
//     CreateLogStreamErrorKind, DescribeLogStreamsErrorKind, PutLogEventsErrorKind,
// };
//use aws_sdk_cloudwatchlogs::types::SdkError;

//use crate::aws::is_retriable_error;
//use crate::sinks::{azure_logs_ingestion::service::AzureLogsIngestionError, util::retries::RetryLogic};
use crate::sinks::prelude::RetryAction;

#[derive(Debug)]
pub struct AzureLogIngestionRetryLogic<F,T> {
    func: F,
    request: PhantomData<T>
}

impl<F, T> AzureLogIngestionRetryLogic<F, T>
    where
        F: Fn(&T) -> StatusCode + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
{
    pub const fn new(func: F) -> AzureLogIngestionRetryLogic<F, T> {
        AzureLogIngestionRetryLogic {
            func,
            request: PhantomData,
        }
    }
}

impl<F, T> Clone for AzureLogIngestionRetryLogic<F, T>
    where
        F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            request: PhantomData,
        }
    }
}


impl<F, T> RetryLogic for AzureLogIngestionRetryLogic<F, T>
    where
        F: Fn(&T) -> StatusCode + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
{
    type Error = HttpError;
    type Response = T;

    fn is_retriable_error(&self, _error: &Self::Error) -> bool {
        true
    }

    fn should_retry_response(&self, response: &T) -> RetryAction {
        let status = (self.func)(response);

        match status {
            StatusCode::TOO_MANY_REQUESTS => RetryAction::Retry("too many requests".into()),
            StatusCode::NOT_IMPLEMENTED => {
                RetryAction::DontRetry("endpoint not implemented".into())
            }
            StatusCode::UNAUTHORIZED => {
                // Implement token renewal logic here
                // For example, assuming `self` contains necessary references to handle token renewal:
                let scope = match reqwest::Url::parse("https://monitor.azure.com//.default") {
                    Ok(scope) => scope,
                    Err(err) => {
                        println!("Error parsing URL: {:?}", err);
                        return RetryAction::DontRetry("Error parsing URL".into());
                    }
                };
                let body = format!(
                    "client_id={}&scope={}&client_secret={}&grant_type=client_credentials",
                    "app_id", scope, "app_secret"
                );

                let client = Client::new();
                let uri = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", "tenant_id");
                let response = match client
                    .post(&uri)
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(body)
                    .send() {
                    Ok(response) => response,
                    Err(err) => {
                        println!("Error response {:?}", err);
                        return RetryAction::DontRetry("Error response".into());
                    }
                };

                let json: Value = match response.json() {
                    Ok(json) => json,
                    Err(err) => {
                        println!("Error parse json {:?}", err);
                        return RetryAction::DontRetry("Error parse json".into());
                    }
                };
                if let Some(access_token) = json.get("access_token") {
                    if let Some(token) = access_token.as_str() {
                        println!("<<<<<<<<<<<<<>>>>>>>>>>>>>>>>>>>");
                        println!("{}", token.to_string());
                        println!("<<<<<<<<<<<<<1>>>>>>>>>>>>>>>>>>>");

                    } else {
                        println!("<<<<<<<<<<<<<0>>>>>>>>>>>>>>>>>>>")
                    }
                } else {
                    println!("<<<<<<<<<<<<<1>>>>>>>>>>>>>>>>>>>")
                }
                RetryAction::Retry("Unauthorized - Retrying with new token".into())
            }
            _ if status.is_server_error() => {
                RetryAction::Retry(format!("Http Status: {}", status).into())
            }
            _ if status.is_success() => RetryAction::Successful,
            _ => RetryAction::DontRetry(format!("Http status: {}", status).into()),
        }
    }
}


//#[cfg(test)]
// mod test {
//     use aws_sdk_cloudwatchlogs::error::PutLogEventsError;
//     use aws_sdk_cloudwatchlogs::types::SdkError;
//     use aws_smithy_http::body::SdkBody;
//     use aws_smithy_http::operation::Response;
//
//     use crate::sinks::azure_logs_ingestion::retry::AzureLogIngestionRetryLogic;
//     use crate::sinks::azure_logs_ingestion::service::AzureLogsIngestionError;
//     use crate::sinks::util::retries::RetryLogic;
//
//     #[test]
//     fn test_throttle_retry() {
//         let retry_logic: AzureLogIngestionRetryLogic<()> = AzureLogIngestionRetryLogic::new();
//
//         let meta_err = aws_smithy_types::Error::builder()
//             .code("ThrottlingException")
//             .message("Rate exceeded for logStreamName log-test-1.us-east-1.compute.internal")
//             .request_id("0ac34e43-f6ff-4e1b-96be-7d03b2be8376")
//             .build();
//
//         let mut http_response = http::Response::new(SdkBody::from("{\"__type\":\"ThrottlingException\",\"message\":\"Rate exceeded for logStreamName log-test-1.us-east-1.compute.internal\"}"));
//         *http_response.status_mut() = http::StatusCode::BAD_REQUEST;
//         let raw = Response::new(http_response);
//
//         let err = AzureLogsIngestionError::Put(SdkError::service_error(
//             PutLogEventsError::unhandled(meta_err),
//             raw,
//         ));
//         assert!(retry_logic.is_retriable_error(&err));
//     }
// }
