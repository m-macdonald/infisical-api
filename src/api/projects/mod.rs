use serde_json::Value;

use crate::enums::SecretType;
use crate::utils::aes256gcm::{decrypt, Encryption};
use crate::error::{self, api, Result};
use crate::api::JsonProcessorExt;

mod models;
pub use models::*;

pub async fn get_project_memberships(
    client: &reqwest::Client,
    request: models::GetProjectMembershipsRequest,
) -> Result<models::GetProjectMembershipsResponse> {
    let endpoint = format!(
        "{}/v2/workspace/{}/memberships",
        request.base_url, request.workspace_id
    );
    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetProjectMembershipsResponse>()
        .await?)
}

pub async fn update_project_membership(
    client: &reqwest::Client,
    request: models::UpdateProjectMembershipRequest,
) -> Result<models::UpdateProjectMembershipResponse> {
    let endpoint = format!(
        "{}/v2/workspace/{}/memberships/{}",
        request.base_url, request.workspace_id, request.membership_id
    );
    Ok(client
        .patch(endpoint)
        .body(request.role)
        .send()
        .await?
        .infisical_json::<models::UpdateProjectMembershipResponse>()
        .await?)
}

pub async fn delete_project_membership(
    client: &reqwest::Client,
    request: models::DeleteProjectMembershipRequest,
) -> Result<models::DeleteProjectMembershipResponse> {
    let endpoint = format!(
        "{}/v2/workspace/{}/memberships/{}",
        request.base_url, request.workspace_id, request.membership_id
    );
    Ok(client
        .delete(endpoint)
        .send()
        .await?
        .infisical_json::<models::DeleteProjectMembershipResponse>()
        .await?)
}

pub async fn get_project_key(
    client: &reqwest::Client,
    request: models::GetProjectKeyRequest,
) -> Result<models::GetProjectKeyResponse> {
    let endpoint = format!(
        "{}/v2/workspace/{}/encrypted-key",
        request.base_url, request.workspace_id
    );
    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetProjectKeyResponse>()
        .await?)
}

pub async fn get_project_logs(
    client: &reqwest::Client,
    request: models::GetProjectLogsRequest,
) -> Result<models::GetProjectLogsResponse> {
    let endpoint = format!(
        "{}/v1/workspace/{}/logs",
        request.base_url, request.workspace_id
    );
    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetProjectLogsResponse>()
        .await?)
}

pub async fn get_project_snapshots(
    client: &reqwest::Client,
    request: models::GetProjectSnapshotsRequest,
) -> Result<models::GetProjectSnapshotsResponse> {
    let endpoint = format!(
        "{}/v1/workspace/{}/secret-snapshots",
        request.base_url, request.workspace_id
    );

    Ok(client
        .get(endpoint)
        .query(&request)
        .send()
        .await?
        .infisical_json::<models::GetProjectSnapshotsResponse>()
        .await?)
}

pub async fn roll_back_to_snapshot(
    client: &reqwest::Client,
    request: models::RollbackProjectToSnapshotRequest,
) -> Result<models::RollbackProjectToSnapshotResponse> {
    let endpoint = format!(
        "{}/v1/workspace/{}/secret-snapshots/rollback",
        request.base_url, request.workspace_id
    );
    Ok(client
        .post(endpoint)
        .body(request.version.to_string())
        .send()
        .await?
        .infisical_json::<models::RollbackProjectToSnapshotResponse>()
        .await?)
}
