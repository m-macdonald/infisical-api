use serde::Deserialize;

use crate::api::Audit;
use crate::api::organizations::Organization;

/// Represents the expected request body for the `/v2/users/me` endpoint
pub struct GetMyUserRequest {
    /// The base url for the Infisical API
    pub base_url: String,
}

/// Represents the successful response for the `/v2/users/me` endpoint
#[derive(Deserialize)]
pub struct GetMyUserResponse {
    /// The Infisical user contained in the response
    pub user: User,
}

/// An Infisical user representation
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(alias = "_id")]
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub salt: String,
    pub iv: String,
    pub tag: String,
    #[serde(alias = "__v")]
    pub v: u8,
    pub devices: Vec<UserDevice>,
    pub encryption_version: Option<u8>,
    pub is_mfa_enabled: bool,
    pub mfa_methods: Vec<String>,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUser {
    #[serde(alias = "_id")]
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(alias = "__v")]
    pub v: u8,
    pub devices: Vec<UserDevice>,
    pub encryption_version: Option<u8>,
    pub is_mfa_enabled: bool,
    pub mfa_methods: Vec<String>,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDevice {
    pub ip: String,
    pub user_agent: String,
    #[serde(alias = "_id")]
    pub id: String,
}

pub struct GetMyOrganizationsRequest {
    /// The base url for the Infisical API
    pub base_url: String,
}

#[derive(Deserialize)]
pub struct GetOrganizationsResponse {
    pub organizations: Vec<Organization>,
}
