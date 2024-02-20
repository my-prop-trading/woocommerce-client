use service_sdk::my_logger::{LogEventCtx, LOGGER};

use crate::{CreateProduct, Product, WooCommerceHttpError, WooHttpClient};

#[allow(async_fn_in_trait)]
pub trait ProductClient {
    async fn create_product(
        &self,
        product: &CreateProduct,
    ) -> Result<Product, WooCommerceHttpError>;

    async fn get_products(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Product>, WooCommerceHttpError>;

    async fn update_product(&self, product: &Product) -> Result<(), WooCommerceHttpError>;
}

impl ProductClient for WooHttpClient {
    async fn create_product(
        &self,
        product: &CreateProduct,
    ) -> Result<Product, WooCommerceHttpError> {
        let url = format!("{}/wc/v3/products", self.base_url);
        let res = self.client.post(&url).json(product).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::create_product",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let product: Result<Product, reqwest::Error> = res.json().await;

                return Ok(product?);
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

    async fn get_products(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Product>, WooCommerceHttpError> {
        let url = format!(
            "{}/wc/v3/products?page={}&per_page={}",
            self.base_url, page, per_page
        );
        let res = self.client.get(&url).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::get_products",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let products = res.json().await;
                return Ok(products?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::get_products",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e.into());
            }
        }
    }

    async fn update_product(&self, product: &Product) -> Result<(), WooCommerceHttpError> {
        let url = format!("{}/wc/v3/products/{}", self.base_url, product.id);
        let res = self.client.put(&url).json(product).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::update_product",
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
                        "WooHttpClient::update_product",
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

    use crate::MetaData;

    use super::*;

    // This test requires a valid consumer key and secret
    //#[tokio::test]
    async fn _test_post_product() {
        let client = WooHttpClient::new("ck_*", "cs_*", "https://checkout*/wp-json");
        let resp = client
            .create_product(&CreateProduct {
                name: "test_post_product".to_string(),
                product_type: "simple".to_string(),
                regular_price: "100.00".to_string(),
                price: "100.00".to_string(),
                virtual_type: true,
                description: "test_post_product".to_string(),
                short_description: "test_post_product".to_string(),
                categories: vec![],
                images: vec![],
                meta_data: vec![
                    MetaData {
                        id: 379,
                        key: "minimum_allowed_quantity".to_string(),
                        value: Value::String("1".to_string()),
                    },
                    MetaData {
                        id: 380,
                        key: "maximum_allowed_quantity".to_string(),
                        value: Value::String("1".to_string()),
                    },
                ],
            })
            .await
            .unwrap();

        println!("Response: {:?}", resp);
        let products = client.get_products(1, 10).await.unwrap();
        println!("Products: {:?}", products);
        for mut item in products {
            item.name = format!("{}-updated", item.name);
            println!("Product: {:?}", item);
            client.update_product(&item).await.unwrap();
        }
    }
}
