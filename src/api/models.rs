use std::collections::HashMap;

use serde::{Deserialize};
use serde_json::Value;
use time::{
    serde::iso8601, 
    OffsetDateTime
};

/// An enum that represents the possible return values from the Infisical API
///
/// Infisical returns a 200 response even for errors on their side, but do provide a JSON response
/// with traditional HTTP response codes and additional error information.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ErrorResponse),
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
