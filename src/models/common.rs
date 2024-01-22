use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub id: i32,
    pub key: String,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Vec<Link>,
    pub collection: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub href: String,
}
