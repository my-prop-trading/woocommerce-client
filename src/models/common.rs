use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MetaData {
        pub id: i32,
        pub key: String,
        pub value: String,
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