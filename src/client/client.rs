use std::env;

use base64::Engine;
use reqwest::header;
use service_sdk::my_logger::{LogEventCtx, LOGGER};

use crate::{CreateOrder, Order};

pub struct WooHttpClient {
    base_url: String,
    client: reqwest::Client,
    debug: bool,
}

impl WooHttpClient {
    pub fn new(consumer_key: &str, consumer_secret: &str, base_url: &str) -> Self {
        assert!(consumer_key.len() > 0);
        assert!(consumer_secret.len() > 0);

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

    pub async fn create_order(&self, order: &CreateOrder) -> Result<Order, reqwest::Error> {
        let url = format!("{}/wc/v3/orders", self.base_url);
        let res = self.client.post(&url).json(order).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::create_order",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let order: Result<Order, reqwest::Error> = res.json().await;

                return Ok(order?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::create_order",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e);
            }
        }
    }

    pub async fn update_order(&self, order: &Order) -> Result<(), reqwest::Error> {
        let url = format!("{}/wc/v3/orders/{}", self.base_url, order.id);
        let res = self.client.put(&url).json(order).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::create_order",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }
                return Ok(());
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::create_order",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e);
            }
        }
    }

    pub async fn get_order(&self, order_id: i32) -> Result<Order, reqwest::Error> {
        let url = format!("{}/wc/v3/orders/{}", self.base_url, order_id);
        let res = self.client.get(&url).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::create_order",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let order = res.json().await;
                return Ok(order?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::create_order",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CreateLineItem, MetaData};

    use super::*;

    #[tokio::test]
    async fn test_post_order() {
        let client = WooHttpClient::new(
            "consumer_key",
            "consumer_secret",
            "https://example.com",
        );
        let resp = client
            .create_order(&CreateOrder {
                payment_method: "Stripe".to_string(),
                payment_method_title: "Credit Card (Stripe)".to_string(),
                set_paid: false,
                billing: crate::ContactDetails {
                    first_name: "John".to_string(),
                    last_name: "Doe".to_string(),
                    company: "Boring LTD".to_string(),
                    address_1: "123 Main St".to_string(),
                    address_2: "".to_string(),
                    city: "New York".to_string(),
                    state: "NY".to_string(),
                    postcode: "10001".to_string(),
                    country: "US".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: "(555) 555-5555".to_string(),
                },
                meta_data: vec![MetaData {
                    id: 1,
                    key: "Some".to_string(),
                    value: "Value".to_string(),
                }],
                line_items: vec![CreateLineItem {
                    product_id: 60,
                    quantity: 1,
                }],
                customer_id: 0,
            })
            .await
            .unwrap();

        println!("Response: {:?}", resp);
        let order = client.get_order(resp.id).await.unwrap();
        println!("Order: {:?}", order);
    }
}
