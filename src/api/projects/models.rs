use serde::{Serialize, Deserialize};
use time::{
    serde::iso8601, 
    OffsetDateTime
};

use crate::{
    api::{
        Audit,
        users::SimpleUser,
        secrets::EncryptedSecret,
    },
    enums::SecretType
};

pub struct GetProjectMembershipsRequest {
    pub base_url: String,
    pub workspace_id: String,
}

#[derive(Deserialize)]
pub struct GetProjectMembershipsResponse {
    pub memberships: Vec<ProjectMembership>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMembership {
    #[serde(alias = "_id")]
    pub id: String,
    pub role: String,
    pub user: SimpleUser,
    pub workspace: String,
    #[serde(flatten)]
    pub audit: Audit,
    pub denied_permissions: Vec<String>,
}

pub struct UpdateProjectMembershipRequest {
    pub base_url: String,
    pub workspace_id: String,
    pub membership_id: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateProjectMembershipResponse {
    pub membership: ProjectMembership,
}

pub struct DeleteProjectMembershipRequest {
    pub base_url: String,
    pub workspace_id: String,
    pub membership_id: String,
}

#[derive(Deserialize)]
pub struct DeleteProjectMembershipResponse {
    pub membership: ProjectMembership,
}

pub struct GetProjectKeyRequest {
    pub base_url: String,
    pub workspace_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectKeyResponse {
    pub encrypted_key: String,
    pub nonce: String,
    pub sender: Sender,
    pub receiver: String,
    pub workspace: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub public_key: String,
}

pub struct GetProjectLogsRequest {
    pub base_url: String,
    pub workspace_id: String,
    pub user_id: String,
    pub offset: String,
    pub limit: String,
    pub sort_by: String,
    pub action_names: String,
}

#[derive(Deserialize)]
pub struct GetProjectLogsResponse {
    pub logs: Vec<ProjectLog>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLog {
    #[serde(alias = "_id")]
    pub id: String,
    // This user does not have all fields that the User struct expects
    pub user: SimpleUser,
    pub workspace: String,
    pub action_names: Vec<String>,
    pub actions: Vec<ProjectLogAction>,
    pub channel: String,
    pub ip_address: String,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Deserialize)]
pub struct ProjectLogAction {
    pub name: String,
    pub user: String,
    pub workspace: String,
    pub payload: Vec<ProjectLogActionPayload>,
}

#[derive(Deserialize)]
pub struct ProjectLogActionPayload {
    #[serde(alias = "oldSecretVersion")]
    pub old_secret_version: String,
    #[serde(alias = "newSecretVersion")]
    pub new_secret_version: String,
}

#[derive(Serialize)]
pub struct GetProjectSnapshotsRequest {
    #[serde(skip)]
    pub base_url: String,
    #[serde(skip)]
    pub workspace_id: String,
    pub offset: String,
    pub limit: String,
}

#[derive(Deserialize)]
pub struct GetProjectSnapshotsResponse {
    #[serde(alias = "secretSnapshots")]
    pub secret_snapshots: Vec<SecretSnapshot>,
}

#[derive(Deserialize)]
pub struct SecretSnapshot {
    #[serde(alias = "_id")]
    pub id: String,
    pub workspace: String,
    pub version: u8,
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<String>,
}

#[derive(Deserialize)]
pub struct ProjectSecretVersion {
    #[serde(alias = "_id")]
    pub id: String,
}

pub struct RollbackProjectToSnapshotRequest {
    pub base_url: String,
    pub workspace_id: String,
    pub version: u8,
}

#[derive(Deserialize)]
pub struct RollbackProjectToSnapshotResponse {
    pub secrets: Vec<EncryptedSecret>,
}

#[derive(Deserialize)]
pub struct RollbackSecret {
    #[serde(alias = "_id")]
    pub id: String,
    pub version: u8,
    pub workspace: String,
    #[serde(alias = "type")]
    pub type_name: Option<SecretType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SimpleUser>,
    #[serde(flatten)]
    pub encrypted_secret: EncryptedSecret,
    #[serde(flatten)]
    pub audit: Audit,
}
