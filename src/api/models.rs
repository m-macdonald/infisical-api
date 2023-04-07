use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{serde::iso8601, OffsetDateTime};

use crate::error::Result;
use crate::utils::aes256gcm::{decrypt, Encryption};

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
    //    pub seen_ips: Vec<String>,
    #[serde(alias = "_id")]
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub iv: String,
    pub tag: String,
    #[serde(alias = "__v")]
    pub v: u8,
    #[serde(flatten)]
    pub audit: Audit,
}

pub struct GetMyOrganizationsRequest {
    /// The base url for the Infisical API
    pub base_url: String,
}

#[derive(Deserialize)]
pub struct GetOrganizationsResponse {
    pub organizations: Vec<Organization>,
}

#[derive(Deserialize)]
pub struct Organization {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    #[serde(alias = "customerId")]
    pub customer_id: String,
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
    pub user: User,
    pub organization: String,
    pub role: String,
    pub status: String,
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

pub struct GetProjectMembershipsRequest {
    pub base_url: String,
    pub workspace_id: String,
}

#[derive(Deserialize)]
pub struct GetProjectMembershipsResponse {
    pub memberships: Vec<ProjectMembership>,
}

#[derive(Deserialize)]
pub struct ProjectMembership {
    #[serde(alias = "_id")]
    pub id: String,
    pub role: String,
    pub user: User,
    pub workspace: String,
    #[serde(flatten)]
    pub audit: Audit,
    #[serde(alias = "deniedPermissions")]
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
pub struct Sender {
    #[serde(alias = "publicKey")]
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
pub struct ProjectLog {
    #[serde(alias = "_id")]
    pub id: String,
    // This user does not have all fields that the User struct expects
    pub user: User,
    pub workspace: String,
    #[serde(alias = "actionNames")]
    pub action_names: Vec<String>,
    pub actions: Vec<ProjectLogAction>,
    pub channel: String,
    #[serde(alias = "ipAddress")]
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

pub struct GetProjectSnapshotsRequest {
    pub base_url: String,
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
    pub workspace: String,
    pub version: u8,
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<ProjectSecretVersion>,
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
    pub secret_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(flatten)]
    pub encrypted_secret: EncryptedSecret,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Serialize)]
pub struct CreateProjectSecretsRequest {
    pub base_url: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    pub environment: String,
    pub secrets: Vec<SecretToCreate>,
}

#[derive(Serialize)]
pub struct SecretToCreate {
    #[serde(rename = "type")]
    pub secret_type: String,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(flatten)]
    pub comment: EncryptedComment,
}

#[derive(Deserialize)]
pub struct CreateProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

pub struct UpdateSecretsRequest {
    pub base_url: String,
    pub secrets: Vec<SecretToUpdate>,
}

#[derive(Serialize)]
pub struct SecretToUpdate {
    pub id: String,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
}

#[derive(Deserialize)]
pub struct UpdateSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectSecretsRequest {
    #[serde(skip)]
    pub base_url: String,
    pub workspace_id: String,
    pub environment: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

pub struct DeleteProjectSecretsRequest {
    pub base_url: String,
    pub secret_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct DeleteProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

pub struct GetProjectSecretVersionsRequest {
    pub base_url: String,
    pub secret_id: String,
    pub offset: String,
    pub limit: String,
}

#[derive(Deserialize)]
pub struct GetProjectSecretVersionsResponse {
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<SecretVersion>,
}

#[derive(Deserialize)]
pub struct SecretVersion {
    pub tags: Vec<String>,
    #[serde(alias = "_id")]
    pub id: String,
    pub secret: String,
    pub version: u8,
    pub workspace: String,
    #[serde(alias = "type")]
    pub secret_type: String,
    pub environment: String,
    #[serde(alias = "isDeleted")]
    pub is_deleted: bool,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(alias = "__v")]
    pub v: u8,
    #[serde(flatten)]
    pub audit: Audit,
}

pub struct RollbackProjectSecretToVersionRequest {
    pub base_url: String,
    pub secret_id: String,
    pub version: u8,
}

pub struct RollbackProjectSecretToVersionResponse {
    pub secret: EncryptedSecret,
}

#[derive(Deserialize)]
pub struct EncryptedSecret {
    #[serde(alias = "_id")]
    pub id: String,
    pub version: u8,
    pub workspace: String,
    #[serde(alias = "type")]
    pub secret_type: String,
    //   pub user: User,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<EncryptedComment>,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Debug)]
pub struct DecryptedSecret {
    pub id: String,
    pub version: u8,
    pub workspace: String,
    pub secret_type: String,
    //    pub user: User,
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub audit: Audit,
}

impl EncryptedSecret {
    pub fn decrypt(secret: &EncryptedSecret, private_key: &str) -> Result<DecryptedSecret> {
        let mut comment = None::<String>;
        let key = decrypt(
            &secret.key.ciphertext,
            &secret.key.iv,
            &secret.key.tag,
            private_key,
        )?;
        let value = decrypt(
            &secret.value.ciphertext,
            &secret.value.iv,
            &secret.value.tag,
            private_key,
        )?;

        if let Some(encrypted_comment) = &secret.comment {
            comment = Some(decrypt(
                &encrypted_comment.ciphertext,
                &encrypted_comment.iv,
                &encrypted_comment.tag,
                private_key,
            )?);
        }

        Ok(DecryptedSecret {
            id: secret.id.clone(),
            version: secret.version,
            workspace: secret.workspace.clone(),
            secret_type: secret.secret_type.clone(),
            //            user: secret.user.clone(),
            key,
            value,
            comment,
            audit: secret.audit.clone(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedKey {
    #[serde(rename = "secretKeyCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretKeyIV")]
    pub iv: String,
    #[serde(rename = "secretKeyTag")]
    pub tag: String,
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedValue {
    #[serde(rename = "secretValueCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretValueIV")]
    pub iv: String,
    #[serde(rename = "secretValueTag")]
    pub tag: String,
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedComment {
    #[serde(rename = "secretCommentCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretCommentIV")]
    pub iv: String,
    #[serde(rename = "secretCommentTag")]
    pub tag: String,
}

impl From<Encryption> for EncryptedComment {
    fn from(encryption: Encryption) -> EncryptedComment {
        EncryptedComment {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
}

impl From<Encryption> for EncryptedValue {
    fn from(encryption: Encryption) -> EncryptedValue {
        EncryptedValue {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
}

impl From<Encryption> for EncryptedKey {
    fn from(encryption: Encryption) -> EncryptedKey {
        EncryptedKey {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
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
