use serde::{Deserialize, Serialize};

use crate::Links;

use super::common::MetaData;

//This is enough for us to create an order
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrder {
    /* pub payment_method: String,
    pub payment_method_title: String, */
    pub set_paid: bool,
    pub billing: ContactDetails,
    pub meta_data: Vec<MetaData>,
    pub line_items: Vec<CreateLineItem>,
    pub customer_id: i32,
    pub coupon_lines: Vec<CouponLineCreate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: i32,
    pub parent_id: i32,
    pub status: OrderStatus,
    pub currency: String,
    pub version: String,
    pub prices_include_tax: bool,
    pub date_created: String,
    pub date_modified: String,
    pub discount_total: String,
    pub discount_tax: String,
    pub shipping_total: String,
    pub shipping_tax: String,
    pub cart_tax: String,
    pub total: String,
    pub total_tax: String,
    pub customer_id: i32,
    pub order_key: String,
    pub billing: ContactDetails,
    pub shipping: ContactDetails,
    pub payment_method: String,
    pub payment_method_title: String,
    pub transaction_id: String,
    pub customer_ip_address: String,
    pub customer_user_agent: String,
    pub created_via: String,
    pub customer_note: String,
    pub date_completed: Option<String>,
    pub date_paid: Option<String>,
    pub cart_hash: String,
    pub number: String,
    pub meta_data: Vec<MetaData>,
    pub line_items: Vec<LineItem>,
    pub tax_lines: Vec<TaxLine>,
    pub shipping_lines: Vec<ShippingLine>,
    pub fee_lines: Vec<FeeLine>,
    pub coupon_lines: Vec<CouponLine>,
    pub refunds: Vec<Refund>,
    pub payment_url: String,
    pub is_editable: bool,
    pub needs_payment: bool,
    pub needs_processing: bool,
    pub date_created_gmt: String,
    pub date_modified_gmt: String,
    pub date_completed_gmt: Option<String>,
    pub date_paid_gmt: Option<String>,
    #[serde(default)]
    pub gift_cards: Option<Vec<GiftCard>>,
    pub currency_symbol: String,
    pub _links: Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDetails {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub address_1: String,
    pub address_2: String,
    pub city: String,
    pub state: String,
    pub postcode: String,
    pub country: String,
    #[serde(default)]
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineItem {
    pub id: i32,
    pub name: String,
    pub product_id: i32,
    pub variation_id: i32,
    pub quantity: i32,
    pub tax_class: String,
    pub subtotal: String,
    pub subtotal_tax: String,
    pub total: String,
    pub total_tax: String,
    pub taxes: Vec<Tax>,
    pub meta_data: Vec<MetaData>,
    pub sku: Option<String>,
    pub price: f64,
    /* pub image: Image,
    pub parent_name: Option<String>, */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLineItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub total: String,
    pub subtotal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub src: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxLine {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub rate_code: String,
    pub rate_id: i32,
    pub label: String,
    pub compound: bool,
    pub tax_total: String,
    pub shipping_tax_total: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingLine {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub method_title: String,
    pub method_id: String,
    pub total: String,
    pub total_tax: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeLine {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub name: String,
    pub tax_class: String,
    pub tax_status: String,
    pub total: String,
    pub total_tax: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CouponLine {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub code: String,
    pub discount: String,
    pub discount_tax: String,
    pub discount_type: String,
    pub nominal_amount: f64,                    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CouponLineCreate {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Refund {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub reason: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiftCard {
    // Assuming a simple structure; update as needed
    pub id: i32,
    pub amount: String,
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "on-hold")]
    OnHold,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "trash")]
    Trash,
    #[serde(rename = "checkout-draft")]
    CheckoutDraft
}