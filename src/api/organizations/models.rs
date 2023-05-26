use serde::Deserialize;

use crate::enums::SecretType;
use crate::error::Result;
use crate::utils::aes256gcm::{decrypt, Encryption};
use crate::api::Audit;
use crate::api::users::SimpleUser;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    pub customer_id: String,
    #[serde(flatten)]
    pub audit: Audit,
}

pub struct GetOrganizationMembershipsRequest {
    /// The base url for the Infisical API
    pub base_url: String,
    pub organization_id: String,
}

#[derive(Deserialize)]
pub struct GetOrganizationMembershipsResponse {
    pub memberships: Vec<OrganizationMembership>,
}

#[derive(Deserialize)]
pub struct OrganizationMembership {
    #[serde(alias = "_id")]
    pub id: String,
    pub organization: String,
    pub role: String,
    pub status: String,
    pub user: SimpleUser,
    #[serde(flatten)]
    pub audit: Audit,
}

pub struct UpdateOrganizationMembershipRequest {
    /// The base url for the Infisical API
    pub base_url: String,
    pub organization_id: String,
    pub membership_id: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateOrganizationMembershipResponse {
    pub membership: OrganizationMembership,
}

pub struct DeleteOrganizationMembershipRequest {
    /// The base url for the Infisical API
    pub base_url: String,
    pub organization_id: String,
    pub membership_id: String,
}

pub struct DeleteOrganizationMembershipResponse {
    pub membership: OrganizationMembership,
}

pub struct GetProjectsRequest {
    /// The base url for the Infisical API
    pub base_url: String,
    pub organization_id: String,
}

#[derive(Deserialize)]
pub struct GetProjectsResponse {
    pub workspaces: Vec<Workspace>,
}

#[derive(Deserialize)]
pub struct Workspace {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    pub organization: String,
    pub environments: Vec<Environment>,
}

#[derive(Deserialize)]
pub struct Environment {
    pub name: String,
    pub slug: String,
}
