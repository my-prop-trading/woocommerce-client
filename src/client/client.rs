use std::env;

use base64::Engine;
use reqwest::{header, Response};
use service_sdk::my_logger::{LogEventCtx, LOGGER};

#[derive(Debug)]
pub enum WooCommerceHttpError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    StringError(String),
}

#[derive(Debug)]
pub enum ResponseStatusCheck<T> {
    Ok(Response),
    Err(Result<T, WooCommerceHttpError>),
}

pub struct WooHttpClient {
    pub(crate) base_url: String,
    pub(crate) client: reqwest::Client,
    pub(crate) debug: bool,
}

impl WooHttpClient {
    pub fn new(consumer_key: &str, consumer_secret: &str, base_url: &str) -> Self {
        assert!(consumer_key.len() > 0);
        assert!(consumer_secret.len() > 0);
        assert!(base_url.len() > 0);

        let auth: String = format!("{}:{}", consumer_key, consumer_secret);
        let auth_encoded = base64::engine::general_purpose::STANDARD.encode(auth);
        let auth_header = format!("Basic {}", auth_encoded);
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("Authorization", auth_header.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let debug = env::var("DEBUG").unwrap_or("0".to_string()) == "1";

        WooHttpClient {
            client,
            base_url: base_url.to_string(),
            debug,
        }
    }

    pub(crate) async fn check_for_failed_status_code<T>(
        &self,
        res: reqwest::Response,
    ) -> ResponseStatusCheck<T> {
        // Process unsuccessful HTTP codes here.
        if !res.status().is_success() {
            let status = res.status();
            let body = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get response body".to_string());
            // Log the unsuccessful response
            if self.debug {
                LOGGER.write_error(
                    "WooHttpClient::create_coupon",
                    format!("Unsuccessful HTTP response: {} - {}", status, body),
                    LogEventCtx::new(),
                );
            }

            let err_detail_message = body;
            // You can customize the error handling based on the status code
            match status {
                _ => {
                    LOGGER.write_error(
                        "WooHttpClient::create_coupon",
                        &err_detail_message,
                        LogEventCtx::new(),
                    );
                }
            }

            // Return a generic error or customize it based on the status code
            return ResponseStatusCheck::Err(Err(WooCommerceHttpError::StringError(err_detail_message)));
        }

        return ResponseStatusCheck::Ok(res);
    }
}

impl From<reqwest::Error> for WooCommerceHttpError {
    fn from(err: reqwest::Error) -> Self {
        WooCommerceHttpError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for WooCommerceHttpError {
    fn from(err: serde_json::Error) -> Self {
        WooCommerceHttpError::SerdeError(err)
    }
}

impl From<String> for WooCommerceHttpError {
    fn from(err: String) -> Self {
        WooCommerceHttpError::StringError(err)
    }
}

#[cfg(test)]
mod tests {
    /*
    use serde_json::Value;

    use crate::{CreateLineItem, MetaData};

    use super::*;

    #[tokio::test]
    async fn test_post_product() {
    }
    */
}
