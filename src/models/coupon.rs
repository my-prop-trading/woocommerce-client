use serde::{Deserialize, Serialize};

use crate::{MetaData, Links};

#[derive(Serialize, Deserialize)]
pub struct CreateCoupon {
    pub code: String,
    pub amount: String,
    pub discount_type: DiscountType,
    pub description: String,
    pub date_expires_gmt: Option<String>,
    pub individual_use: bool,
    pub product_ids: Vec<i32>,
    #[serde(default)]
    pub usage_limit: Option<i32>,
    #[serde(default)]
    pub usage_limit_per_user: Option<i32>,
    #[serde(default)]
    pub limit_usage_to_x_items: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Coupon {
    pub id: i32,
    pub code: String,
    pub amount: String,
    pub date_created: String,
    pub date_created_gmt: String,
    pub date_modified: String,
    pub date_modified_gmt: String,
    pub discount_type: DiscountType,
    pub description: String,
    pub date_expires: Option<String>,
    pub date_expires_gmt: Option<String>,
    pub usage_count: i32,
    pub individual_use: bool,
    pub product_ids: Vec<i32>,
    pub excluded_product_ids: Vec<i32>,
    #[serde(default)]
    pub usage_limit: Option<i32>,
    #[serde(default)]
    pub usage_limit_per_user: Option<i32>,
    #[serde(default)]
    pub limit_usage_to_x_items: Option<i32>,
    pub free_shipping: bool,
    pub product_categories: Vec<i32>,
    pub excluded_product_categories: Vec<i32>,
    pub exclude_sale_items: bool,
    pub minimum_amount: String,
    pub maximum_amount: String,
    pub email_restrictions: Vec<String>,
    pub used_by: Vec<String>,
    pub meta_data: Vec<MetaData>,
    pub _links: Links,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum DiscountType {
    #[default]
    #[serde(rename = "fixed_cart")]
    FixedCart,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "fixed_product")]
    FixedProduct,
}