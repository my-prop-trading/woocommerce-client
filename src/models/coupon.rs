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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCoupon {
    pub id: i32,
    // CODE SHOULD NOT BE UPDATED YOU WILL GET AN ERROR
    //pub code: String,
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

// impl into from coupon to update coupon
impl Into<UpdateCoupon> for Coupon {
    fn into(self) -> UpdateCoupon {
        UpdateCoupon {
            id: self.id,
            amount: self.amount,
            date_created: self.date_created,
            date_created_gmt: self.date_created_gmt,
            date_modified: self.date_modified,
            date_modified_gmt: self.date_modified_gmt,
            discount_type: self.discount_type,
            description: self.description,
            date_expires: self.date_expires,
            date_expires_gmt: self.date_expires_gmt,
            usage_count: self.usage_count,
            individual_use: self.individual_use,
            product_ids: self.product_ids,
            excluded_product_ids: self.excluded_product_ids,
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories,
            excluded_product_categories: self.excluded_product_categories,
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            email_restrictions: self.email_restrictions,
            used_by: self.used_by,
            meta_data: self.meta_data,
            _links: self._links,
        }
    }
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