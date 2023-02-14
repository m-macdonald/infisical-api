use time::{OffsetDateTime, serde::iso8601};
use serde::{ Deserialize, Serialize };


#[derive(Deserialize)]
pub struct GetMyUserResponse {
    pub user: User
}

#[derive(Deserialize)]
pub struct User {
    #[serde(alias = "seenIps")]
    pub seen_ips: Vec<String>,
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
    pub audit: Audit
}

#[derive(Deserialize)]
pub struct GetOrganizationsResponse {
    pub organizations: Vec<Organization>
}

#[derive(Deserialize)]
pub struct Organization {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    #[serde(alias = "customerId")]
    pub customer_id: String
}

pub struct GetOrganizationMembershipsRequest {
    pub organization_id: String
}

#[derive(Deserialize)]
pub struct GetOrganizationMembershipsResponse {
    pub memberships: Vec<OrganizationMembership>
}

#[derive(Deserialize)]
pub struct OrganizationMembership {
    pub user: User,
    pub organization: String,
    pub role: String,
    pub status: String
}

pub struct UpdateOrganizationMembershipRequest {
    pub organization_id: String,
    pub membership_id: String,
    pub role: String
}

pub struct UpdateOrganizationMembershipResponse {
    pub membership: OrganizationMembership
}

pub struct DeleteOrganizationMembershipRequest {
    pub organization_id: String,
    pub membership_id: String,
}

pub struct DeleteOrganizationMembershipResponse {
    pub membership: OrganizationMembership
}

pub struct GetProjectsRequest{
    pub organization_id: String
}

#[derive(Deserialize)]
pub struct GetProjectsResponse {
    pub workspaces: Vec<Workspace>    
}

#[derive(Deserialize)]
pub struct Workspace {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    pub organization: String,
    pub environments: Vec<Environment>
}

#[derive(Deserialize)]
pub struct Environment {
    pub name: String,
    pub slug: String
}

pub struct GetProjectMembershipsRequest {
    pub workspace_id: String
}

#[derive(Deserialize)]
pub struct GetProjectMembershipsResponse {
    pub memberships: Vec<ProjectMembership>
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
    pub denied_permissions: Vec<String>
}

pub struct UpdateProjectMembershipRequest {
    pub workspace_id: String,
    pub membership_id: String,
    pub role: String
}

pub struct UpdateProjectMembershipResponse {
    pub membership: ProjectMembership
}

pub struct DeleteProjectMembershipRequest {
    pub workspace_id: String,
    pub membership_id: String,
}

pub struct DeleteProjectMembershipResponse {
    pub membership: ProjectMembership
}

pub struct GetProjectKeyRequest {
    pub workspace_id: String
}

#[derive(Deserialize)]
pub struct GetProjectKeyResponse {
    #[serde(alias = "encryptedKey")]
    pub encrypted_key: String,
    pub nonce: String,
    pub sender: Sender,
    pub reciever: String,
    pub workspace: String
}

#[derive(Deserialize)]
pub struct Sender {
    #[serde(alias = "publicKey")]
    pub public_key: String
}

pub struct GetProjectLogsRequest {
    pub workspace_id: String,
    pub user_id: String,
    pub offset: String,
    pub limit: String,
    pub sort_by: String,
    pub action_names: String,
}

#[derive(Deserialize)]
pub struct GetProjectLogsResponse {
    pub logs: Vec<ProjectLog>
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
    pub audit: Audit
}

#[derive(Deserialize)]
pub struct ProjectLogAction {
    pub name: String,
    pub user: String,
    pub workspace: String,
    pub payload: Vec<ProjectLogActionPayload>
}

#[derive(Deserialize)]
pub struct ProjectLogActionPayload {
    #[serde(alias = "oldSecretVersion")]
    pub old_secret_version: String,
    #[serde(alias = "newSecretVersion")]
    pub new_secret_version: String
}

pub struct GetProjectSnapshotsRequest {
    pub workspace_id: String,
    pub offset: String,
    pub limit: String
}

#[derive(Deserialize)]
pub struct GetProjectSnapshotsResponse {
    #[serde(alias = "secretSnapshots")]
    pub secret_snapshots: Vec<SecretSnapshot>
}

#[derive(Deserialize)]
pub struct SecretSnapshot {
    pub workspace: String,
    pub version: u8,
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<ProjectSecretVersion>
}

#[derive(Deserialize)]
pub struct ProjectSecretVersion {
    #[serde(alias = "_id")]
    pub id: String 
}

pub struct RollbackProjectToSnapshotRequest {
    pub workspace_id: String,
    pub version: u8
}

#[derive(Deserialize)]
pub struct RollbackProjectToSnapshotResponse {
    pub secrets: Vec<EncryptedSecret>
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
    pub audit: Audit
} 

pub struct CreateProjectSecretsRequest {
    pub workspace_id: String,
    pub environment: String,
    pub secrets: Vec<SecretToCreate>
}

#[derive(Serialize)]
pub struct SecretToCreate {
    #[serde(rename = "type")]
    pub secret_type: String,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue
}

#[derive(Deserialize)]
pub struct CreateProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret> 
}

pub struct UpdateSecretsRequest {
    pub secrets: Vec<SecretToUpdate>
}

#[derive(Serialize)]
pub struct SecretToUpdate {
    pub id: String,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue
}

#[derive(Deserialize)]
pub struct UpdateSecretsResponse {
    pub secrets: Vec<EncryptedSecret>
}

#[derive(Serialize)]
pub struct GetProjectSecretsRequest {
    #[serde(rename = "workspaceId" )]
    pub workspace_id: String,
    pub environment: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret>
}

pub struct DeleteProjectSecretsRequest {
    pub secret_ids: Vec<String>
}

#[derive(Deserialize)]
pub struct DeleteProjectSecretsResponse {
    pub secrets: Vec<EncryptedSecret>
}

pub struct GetProjectSecretVersionsRequest {
    pub secret_id: String,
    pub offset: String,
    pub limit: String
}

#[derive(Deserialize)]
pub struct GetProjectSecretVersionsResponse {
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<SecretVersion>
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
    pub audit: Audit
}

pub struct RollbackProjectSecretToVersionRequest {
    pub secret_id: String,
    pub version: u8
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
    pub user: User,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(flatten)]
    pub comment: EncryptedComment,
    #[serde(flatten)]
    pub audit: Audit
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedKey {
    #[serde(alias = "secretKeyCiphertext")]
    pub secret_key_ciphertext: String,
    #[serde(alias = "secretKeyIV")]
    pub secret_key_iv: String,
    #[serde(alias = "secretKeyTag")]
    pub secret_key_tag: String,
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedValue {
    #[serde(alias = "secretValueCiphertext")]
    pub secret_value_ciphertext: String,
    #[serde(alias = "secretValueIV")]
    pub secret_value_iv: String,
    #[serde(alias = "secretValueTag")]
    pub secret_value_tag: String,
}

#[derive(Deserialize, Serialize)]
pub struct EncryptedComment {
    #[serde(alias = "secretCommentCiphertext")]
    pub secret_comment_ciphertext: String,
    #[serde(alias = "secretCommentIV")]
    pub secret_comment_iv: String,
    #[serde(alias = "secretCommentTag")]
    pub secret_comment_tag: String,
}

#[derive(Deserialize)]
pub struct Audit {
    #[serde(alias = "updatedAt", with = "iso8601")]
    pub updated_at: OffsetDateTime,
    #[serde(alias = "createdAt", with = "iso8601")]
    pub created_at: OffsetDateTime
}
