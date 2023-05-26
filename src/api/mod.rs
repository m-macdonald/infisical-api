use async_trait::async_trait;
use serde::{
    Deserialize,
    de::DeserializeOwned
};
use serde_json::Value;
use std::collections::HashMap;
use time::{
    OffsetDateTime,
    serde::iso8601
};

pub mod users;
pub mod organizations;
pub mod projects;
pub mod secrets;
pub mod service_tokens;
/*
pub use users::*;
pub use organizations::*;
pub use projects::*;
pub use secrets::*;
pub use service_tokens::*;
*/

use crate::error::{self, api, Result};
/// Trait to extend the json deserialization functionality of [reqwest::Response]
///
/// The Infiscal API returns 200 by default, even if there were errors with the request.
/// They instead include a JSON response with an error message and the true HTTP status code
/// This trait calls [reqwest::Response.json] internally and converts the Infisical error response
/// to an [infisical_rs::Error]
#[async_trait]
trait JsonProcessorExt {
    async fn infisical_json<T>(self) -> Result<T>
    where
        T: DeserializeOwned;
}

#[async_trait]
impl JsonProcessorExt for reqwest::Response {
    async fn infisical_json<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        match self.json::<ApiResponse<T>>().await? {
            ApiResponse::Ok(res) => Ok(res),
            ApiResponse::Err(err) => Err(error::api(err)),
            ApiResponse::Unknown(value) => panic!("{:#?}", value)
        }
    }
}

/// An enum that represents the possible return values from the Infisical API
///
/// Infisical returns a 200 response even for errors on their side, but do provide a JSON response
/// with traditional HTTP response codes and additional error information.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ErrorResponse),
    // A catch all just in case Infisical doesn't send us something we can deserialize as an
    // ErrorResponse or the given generic T
    Unknown(Value)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
}


#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(alias = "type")]
    pub type_name: String,
    pub message: String,
    pub context: HashMap<String, Value>,
    pub level: i16,
    pub level_name: String,
    pub status_code: i16,
    pub datetime_iso: String,
    pub application: String,
    pub extra: Vec<String>,
}
