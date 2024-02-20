use service_sdk::my_logger::{LogEventCtx, LOGGER};

use crate::{CreateOrder, Order, WooCommerceHttpError, WooHttpClient};

#[allow(async_fn_in_trait)]
pub trait OrderClient {
    async fn create_order(&self, order: &CreateOrder) -> Result<Order, WooCommerceHttpError>;

    async fn update_order(&self, order: &Order) -> Result<(), WooCommerceHttpError>;

    async fn get_order(&self, order_id: i32) -> Result<Order, WooCommerceHttpError>;
}

impl OrderClient for WooHttpClient {
    async fn create_order(&self, order: &CreateOrder) -> Result<Order, WooCommerceHttpError> {
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

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

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
                return Err(e.into());
            }
        }
    }

    async fn update_order(&self, order: &Order) -> Result<(), WooCommerceHttpError> {
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

                let _ = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

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
                return Err(e.into());
            }
        }
    }

    async fn get_order(&self, order_id: i32) -> Result<Order, WooCommerceHttpError> {
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

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

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
                return Err(e.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{CreateLineItem, MetaData};

    use super::*;

    // This test requires a valid consumer key and secret
    //#[tokio::test]
    async fn _test_post_order() {
        let client = WooHttpClient::new("ck_*", "cs_*", "https://checkout*/wp-json");
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
                    value: Value::String("Value".to_string()),
                }],
                line_items: vec![CreateLineItem {
                    //product_id: 60,
                    product_id: 19,
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
