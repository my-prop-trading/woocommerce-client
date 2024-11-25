use service_sdk::my_logger::{LogEventCtx, LOGGER};

use crate::{Coupon, CreateCoupon, UpdateCoupon, WooCommerceHttpError, WooHttpClient};

#[allow(async_fn_in_trait)]
pub trait CouponClient {
    async fn create_coupon(&self, coupon: &CreateCoupon) -> Result<Coupon, WooCommerceHttpError>;

    async fn get_coupons(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Coupon>, WooCommerceHttpError>;

    async fn update_coupon(&self, coupon: &UpdateCoupon) -> Result<Coupon, WooCommerceHttpError>;

    async fn delete_coupon(&self, id: i32) -> Result<Coupon, WooCommerceHttpError>;

    async fn get_coupon(&self, id: i32) -> Result<Option<Coupon>, WooCommerceHttpError>;
}

impl CouponClient for WooHttpClient {
    async fn create_coupon(&self, coupon: &CreateCoupon) -> Result<Coupon, WooCommerceHttpError> {
        let url = format!("{}/wc/v3/coupons", self.base_url);
        let res = self.client.post(&url).json(coupon).send().await;
        println!("Response: {:?}", res);
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::create_coupon",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let coupon: Result<Coupon, reqwest::Error> = res.json().await;

                return Ok(coupon?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_warning(
                        "WooHttpClient::create_coupon",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e.into());
            }
        }
    }

    async fn get_coupons(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Coupon>, WooCommerceHttpError> {
        let url = format!(
            "{}/wc/v3/coupons?page={}&per_page={}",
            self.base_url, page, per_page
        );
        let res = self.client.get(&url).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::get_coupons",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let coupons = res.json().await;
                return Ok(coupons?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::get_coupons",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e.into());
            }
        }
    }

    async fn update_coupon(&self, coupon: &UpdateCoupon) -> Result<Coupon, WooCommerceHttpError> {
        let url = format!("{}/wc/v3/coupons/{}", self.base_url, coupon.id);
        let res = self.client.put(&url).json(coupon).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::update_coupon",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let coupon = res.json().await;

                return Ok(coupon?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::update_coupon",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e.into());
            }
        }
    }

    async fn delete_coupon(&self, id: i32) -> Result<Coupon, WooCommerceHttpError> {
        let url = format!("{}/wc/v3/coupons/{}?force=true", self.base_url, id);
        let res = self.client.delete(&url).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::delete_coupon",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let coupon = res.json().await;

                return Ok(coupon?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::delete_coupon",
                        format!("Error: {:?}", e),
                        LogEventCtx::new(),
                    );
                }
                return Err(e.into());
            }
        }
    }

    async fn get_coupon(&self, id: i32) -> Result<Option<Coupon>, WooCommerceHttpError> {
        let url = format!("{}/wc/v3/coupons/{}", self.base_url, id);
        let res = self.client.get(&url).send().await;
        match res {
            Ok(res) => {
                if self.debug {
                    LOGGER.write_info(
                        "WooHttpClient::get_coupon",
                        format!("Response: {:?}", res),
                        LogEventCtx::new(),
                    );
                }

                let res = match self.check_for_failed_status_code(res).await {
                    crate::ResponseStatusCheck::Ok(res) => res,
                    crate::ResponseStatusCheck::Err(err) => return err,
                };

                let coupon = res.json().await;

                return Ok(coupon?);
            }
            Err(e) => {
                if self.debug {
                    LOGGER.write_error(
                        "WooHttpClient::get_coupon",
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
    use std::time::Duration;

    use service_sdk::{flurl::my_tls::tokio_rustls::rustls::client::AlwaysResolvesClientRawPublicKeys, rust_extensions::date_time::DateTimeAsMicroseconds};
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn test_coupon() {
        let client = WooHttpClient::new(
            std::env::var("WOO_CONSUMER_KEY").unwrap().as_str(),
            std::env::var("WOO_CONSUMER_SECRET").unwrap().as_str(),
            std::env::var("WOO_BASE_URL").unwrap().as_str(),
        );
        let now = DateTimeAsMicroseconds::now().add(Duration::from_secs(60 * 60 * 24 * 30));
        let resp = client
            .create_coupon(&CreateCoupon {
                amount: "10.0".to_string(),
                code: "test1234".to_string(),
                description: "test1".to_string(),
                discount_type: crate::DiscountType::Percent,
                date_expires_gmt: Some(now.to_rfc3339()),
                individual_use: false,
                //Ensure it exists!
                product_ids: vec![335],
                usage_limit: None,
                usage_limit_per_user: None,
                limit_usage_to_x_items: None,
            })
            .await
            .unwrap();

        println!("Coupon: {:?}", resp);
        let coupons: Vec<Coupon> = client.get_coupons(1, 10).await.unwrap();
        println!("Coupons: {:?}", coupons);

        for item in coupons {
            let mut item: UpdateCoupon = item.into();
            item.description = format!("{}-updated", item.description);
            println!("Coupon: {:?}", item);
            client.update_coupon(&item).await.unwrap();
            let res: Option<Coupon> = client.get_coupon(item.id).await.unwrap();
            println!("Coupon: {:?}", res);
            client.delete_coupon(res.unwrap().id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_existing_coupon() {
        env_logger::init();
        let client = WooHttpClient::new(
            std::env::var("WOO_CONSUMER_KEY").unwrap().as_str(),
            std::env::var("WOO_CONSUMER_SECRET").unwrap().as_str(),
            std::env::var("WOO_BASE_URL").unwrap().as_str(),
        );

       /*  let coupons = client.get_coupons(1, 10).await;
        println!("Coupons: {:?}", coupons);
        sleep(Duration::from_secs(5)).await;
        println!("End");
        return; 
        let coupons: Vec<Coupon> = coupons.unwrap();
        println!("Coupons: {:?}", coupons); 

        for item in coupons {
            //let mut item: UpdateCoupon = item.into();
            //item.description = format!("{}-updated", item.description);
            println!("Coupon: {:?}", item);
            //client.update_coupon(&item).await.unwrap();
            //let res: Option<Coupon> = client.get_coupon(item.id).await.unwrap();
            //println!("Coupon: {:?}", res);
            //client.delete_coupon(res.unwrap().id).await.unwrap();
        }*/

        let now = DateTimeAsMicroseconds::now().add(Duration::from_secs(60 * 60 * 24 * 30));
        let resp = client
            .create_coupon(&CreateCoupon {
                amount: "10.0".to_string(),
                code: "top1".to_string(),
                description: "test1".to_string(),
                discount_type: crate::DiscountType::Percent,
                date_expires_gmt: Some(now.to_rfc3339()),
                individual_use: false,
                //Ensure it exists!
                product_ids: vec![2636],
                usage_limit: None,
                usage_limit_per_user: None,
                limit_usage_to_x_items: None,
            })
            .await;

        println!("Coupon: {:?}", resp);
    }
}
