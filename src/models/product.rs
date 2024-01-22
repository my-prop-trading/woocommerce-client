use serde::{Deserialize, Serialize};

use crate::{MetaData, Links};

#[derive(Serialize, Deserialize)]
pub struct CreateProduct {
   pub name: String,
   #[serde(rename = "type")]
   pub product_type: String,
   pub regular_price: String,
   pub price: String,
   #[serde(rename = "virtual")]
   pub virtual_type: bool,
   pub description: String,
   pub short_description: String,
   pub categories: Vec<Category>,
   pub images: Vec<Image>,
   pub meta_data: Vec<MetaData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub permalink: String,
    pub date_created: String,
    pub date_created_gmt: String,
    pub date_modified: String,
    pub date_modified_gmt: String,
    #[serde(rename = "type")]
    pub product_type: String,
    pub status: String,
    pub featured: bool,
    pub catalog_visibility: String,
    pub description: String,
    pub short_description: String,
    pub sku: String,
    pub price: String,
    pub regular_price: String,
    
    #[serde(default)]
    pub sale_price: String,
    #[serde(default)]
    pub date_on_sale_from: Option<String>,
    #[serde(default)]
    pub date_on_sale_from_gmt: Option<String>,
    #[serde(default)]
    pub date_on_sale_to: Option<String>,
    #[serde(default)]
    pub date_on_sale_to_gmt: Option<String>,
     
    pub price_html: String,
    pub on_sale: bool,
    pub purchasable: bool,
    pub total_sales: i32,
    #[serde(rename = "virtual")]
    pub virtual_type: bool,
    
    pub downloadable: bool,
    pub downloads: Vec<Download>,
    pub download_limit: i32,
    pub download_expiry: i32,
    pub external_url: String,
    pub button_text: String,
    pub tax_status: String,
    pub tax_class: String,
    pub manage_stock: bool,
    #[serde(default)]
    pub stock_quantity: Option<i32>,
    pub stock_status: String,
    pub backorders: String,
    pub backorders_allowed: bool,
    pub backordered: bool,
    pub sold_individually: bool,
    pub weight: String,
    pub dimensions: Dimensions,
    pub shipping_required: bool,
    pub shipping_taxable: bool,
    pub shipping_class: String,
    pub shipping_class_id: i32,
    pub reviews_allowed: bool,
    pub average_rating: String,
    pub rating_count: i32,
    pub related_ids: Vec<i32>,
    pub upsell_ids: Vec<i32>,
    pub cross_sell_ids: Vec<i32>,
    pub parent_id: i32,
    pub purchase_note: String,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub images: Vec<Image>,
    pub attributes: Vec<Attribute>,
    pub default_attributes: Vec<Attribute>,
    pub variations: Vec<i32>,
    pub grouped_products: Vec<i32>,
    pub menu_order: i32,
    pub meta_data: Vec<MetaData>,
    pub _links: Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
    pub id: i32,
    pub date_created: String,
    pub date_created_gmt: String,
    pub date_modified: String,
    pub date_modified_gmt: String,
    pub src: String,
    pub name: String,
    pub alt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    pub length: String,
    pub width: String,
    pub height: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    // Assuming similar structure to Category
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
   pub id: i32,
   pub date_created: String,
   pub date_created_gmt: String,
   pub date_modified: String,
   pub date_modified_gmt: String,
   pub src: String,
   pub name: String,
   pub alt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub visible: bool,
    pub variation: bool,
    pub options: Vec<String>,
}
