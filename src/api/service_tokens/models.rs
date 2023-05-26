use serde::{Serialize, Deserialize};
use serde_json::Value;
use time::{
    serde::iso8601, 
    OffsetDateTime
};

use crate::enums::SecretType;
use crate::error::Result;
use crate::utils::aes256gcm::{decrypt, Encryption};
use crate::api::Audit;


pub struct GetServiceTokensRequest {
    pub base_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceTokensResponse {
    pub service_token_data: ServiceToken,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceToken {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    pub workspace: String,
    pub environment: String,
    // The response from the service token endpoint has changed the structure of this user a couple
    // times. It's a lower priority value so I'm omitting it until the endpoint stabilizes.
    // pub user: SimpleUser,
    #[serde(with = "iso8601::option", default)]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(with = "iso8601")]
    pub last_used: OffsetDateTime,
    pub encrypted_key: String,
    pub iv: String,
    pub tag: String,
    pub permissions: Vec<String>,
    #[serde(flatten)]
    pub audit: Audit,
    #[serde(alias = "__v")]
    pub v: u8,
}
