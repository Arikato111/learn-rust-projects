use serde_derive::Deserialize;
use serde_derive::Serialize;

// generate type from "https://transform.tools/json-to-rust-serde"

pub type Person = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    #[serde(rename = "usr_name")]
    pub usr_name: String,
    #[serde(rename = "usr_address")]
    pub usr_address: String,
    #[serde(rename = "usr_date")]
    pub usr_date: String,
    #[serde(rename = "usr_email")]
    pub usr_email: String,
    #[serde(rename = "usr_tel")]
    pub usr_tel: String,
    #[serde(rename = "usr_username")]
    pub usr_username: String,
    #[serde(rename = "usr_view")]
    pub usr_view: i64,
    #[serde(rename = "usr_regis_date")]
    pub usr_regis_date: String,
    #[serde(rename = "usr_img")]
    pub usr_img: String,
}
