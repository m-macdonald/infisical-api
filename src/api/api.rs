use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::api::models;
use crate::error::{self, api, Result};

use super::models::ApiResponse;

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
        }
    }
}

pub async fn get_my_user(
    client: &reqwest::Client,
    request: models::GetMyUserRequest,
) -> Result<models::GetMyUserResponse> {
    let endpoint = format!("{}/v2/users/me", request.base_url);
    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetMyUserResponse>()
        .await?)
}

pub async fn get_my_organizations(
    client: &reqwest::Client,
    request: models::GetMyOrganizationsRequest,
) -> Result<models::GetOrganizationsResponse> {
    let endpoint = format!("{}/v2/users/me/organizations", request.base_url);
    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetOrganizationsResponse>()
        .await?)
}

pub async fn get_organization_memberships(
    client: &reqwest::Client,
    request: models::GetOrganizationMembershipsRequest,
) -> Result<models::GetOrganizationMembershipsResponse> {
    let endpoint = format!(
        "{}/v2/organizations/{}/memberships",
        request.base_url, request.organization_id
    );

    let res = client.get(&endpoint).send().await?;
    println!("{}", res.text().await?);

    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetOrganizationMembershipsResponse>()
        .await?)
}

pub async fn update_organization_membership(
    client: &reqwest::Client,
    request: models::UpdateOrganizationMembershipRequest,
) -> Result<models::UpdateOrganizationMembershipResponse> {
    let endpoint = format!(
        "{}/v2/organizations/{}/memberships/{}",
        request.base_url, request.organization_id, request.membership_id
    );
    Ok(client
        .patch(endpoint)
        .body(request.role)
        .send()
        .await?
        .infisical_json::<models::UpdateOrganizationMembershipResponse>()
        .await?)
}

pub async fn delete_organization_membership(
    client: &reqwest::Client,
    request: models::DeleteOrganizationMembershipRequest,
) -> Result<models::UpdateOrganizationMembershipResponse> {
    let endpoint = format!(
        "{}/v2/organizations/{}/memberships/{}",
        request.base_url, request.organization_id, request.membership_id
    );
    Ok(client
        .delete(endpoint)
        .send()
        .await?
        .infisical_json::<models::UpdateOrganizationMembershipResponse>()
        .await?)
}

pub async fn get_organization_projects(
    client: &reqwest::Client,
    request: models::GetProjectsRequest,
) -> Result<models::GetProjectsResponse> {
    let endpoint = format!(
        "{}/v2/organizations/{}/workspaces",
        request.base_url, request.organization_id
    );

    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::GetProjectsResponse>()
        .await?)
}

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

pub async fn create_project_secrets(
    client: &reqwest::Client,
    request: models::CreateProjectSecretsRequest,
) -> Result<models::CreateProjectSecretsResponse> {
    let endpoint = format!("{}/v2/secrets", request.base_url);

    Ok(client
        .post(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<models::CreateProjectSecretsResponse>()
        .await?)
}

pub async fn update_project_secrets(
    client: &reqwest::Client,
    request: models::UpdateSecretsRequest,
) -> Result<models::UpdateSecretsResponse> {
    let endpoint = format!("{}/v2/secrets", request.base_url);

    Ok(client
        .patch(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<models::UpdateSecretsResponse>()
        .await?)
}

/// Gets all of the secrets belonging the workspace provided in the request
pub async fn get_project_secrets(
    client: &reqwest::Client,
    request: models::GetProjectSecretsRequest,
) -> Result<models::GetProjectSecretsResponse> {
    let endpoint = format!("{}/v2/secrets", request.base_url);

    Ok(client
        .get(endpoint)
        .query(&request)
        .send()
        .await?
        .infisical_json::<models::GetProjectSecretsResponse>()
        .await?)
}

pub async fn get_service_token(
    client: &reqwest::Client,
    request: models::GetServiceTokensRequest,
) -> Result<models::ServiceToken> {
    let endpoint = format!("{}/v2/service-token", request.base_url);

    let res = client.get(&endpoint).send().await?;

    Ok(client
        .get(endpoint)
        .send()
        .await?
        .infisical_json::<models::ServiceToken>()
        .await?)
}
