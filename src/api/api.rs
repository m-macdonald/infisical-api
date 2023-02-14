use crate::infisical::api::models;

use reqwest::Method;

pub async fn get_my_user(client: &reqwest::Client) -> Result<models::GetMyUserResponse, reqwest::Error> {
    let endpoint = format!("v2/users/me");
    client.request(Method::GET, endpoint).send().await.json::<models::GetMyUserResponse>()
}

pub async fn get_my_organizations(client: &reqwest::Client) -> Result<models::GetOrganizationsResponse, reqwest::Error> {
    let endpoint = format!("v2/users/me/organizations");
    client.request(Method::GET, endpoint).send().await.json::<models::GetOrganizationsResponse>()
}

pub async fn get_organization_memberships(client: &reqwest::Client, request: models::GetOrganizationMembershipsRequest) -> Result<models::GetOrganizationMembershipsResponse, reqwest::Error> {
    let endpoint = format!("v2/organizations/{}/memberships", request.organization_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetOrganizationMembershipsResponse>()
}

pub async fn update_organization_membership(client: &reqwest::Client, request: models::UpdateOrganizationMembershipRequest) -> Result<models::UpdateOrganizationMembershipResponse, reqwest::Error> {
    let endpoint = format!("v2/organizations/{}/memberships/{}", request.organization_id, request.membership_id);
    client.request(Method::PATCH, endpoint).body(request.role).send().await.json::<models::UpdateOrganizationMembershipResponse>()
}

pub async fn delete_organization_membership(client: &reqwest::Client, request: models::DeleteOrganizationMembershipRequest) -> Result<models::UpdateOrganizationMembershipResponse, reqwest::Error> {
    let endpoint = format!("v2/organizations/{}/memberships/{}", request.organization_id, request.membership_id);
    client.request(Method::DELETE, endpoint).send().await.json::<models::UpdateOrganizationMembershipResponse>()
}

pub async fn get_organization_projects(client: &reqwest::Client, request: models::GetProjectsRequest) -> Result<models::GetProjectsResponse, reqwest::Error> {
    let endpoint = format!("v2/organizations/{}/workspaces", request.organization_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetProjectsResponse>()
}

pub async fn get_project_memberships(client: &reqwest::Client, request: models::GetProjectMembershipsRequest) -> Result<models::GetProjectMembershipsResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/memberships", request.workspace_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetProjectMembershipsResponse>()
}

pub async fn update_project_membership(client: &reqwest::Client, request: models::UpdateProjectMembershipRequest) -> Result<models::UpdateProjectMembershipResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/memberships/{}", request.workspace_id, request.membership_id);
    client.request(Method::PATCH, endpoint).body(request.role).send().await.json::<models::UpdateProjectMembershipResponse>()
}

pub async fn delete_project_membership(client: &reqwest::Client, request: models::DeleteProjectMembershipRequest) -> Result<models::DeleteProjectMembershipResponse> {
    let endpoint = format!("v2/workspace/{}/memberships/{}", request.workspace_id, request.membership_id);
    client.request(Method::DELETE, endpoint).send().await.json::<models::DeleteProjectMembershipResponse>()
}

pub async fn get_project_key(client: &reqwest::Client, request: models::GetProjectKeyRequest) -> Result<models::GetProjectKeyResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/encrypted-key", request.workspace_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetProjectKeyResponse>()
}

pub async fn get_project_logs(client: &reqwest::Client, request: models::GetProjectLogsRequest) -> Result<models::GetProjectLogsResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/logs", request.workspace_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetProjectLogsResponse>()
}

pub async fn get_project_snapshots(client: &reqwest::Client, request: models::GetProjectSnapshotsRequest) -> Result<models::GetProjectSnapshotsResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/secret-snapshots", request.workspace_id);
    client.request(Method::GET, endpoint).send().await.json::<models::GetProjectSnapshotsResponse>()
}

pub async fn roll_back_to_snapshot(client: &reqwest::Client, request: models::RollbackProjectToSnapshotRequest) -> Result<models::RollbackProjectToSnapshotResponse, reqwest::Error> {
    let endpoint = format!("v2/workspace/{}/secret-snapshots/rollback", request.workspace_id);
    client.request(Method::POST, endpoint).body(request.version).send().await.json::<models::RollbackProjectToSnapshotResponse>()
}

pub async fn create_project_secrets(client: &reqwest::Client, request: models::CreateProjectSecretsRequest) -> Result<models::CreateProjectSecretsResponse, reqwest::Error> {
    let endpoint = format!("v2/secrets");
    client.request(Method::POST, endpoint).body(request).send().await.json::<models::CreateProjectSecretsResponse>()
}

pub async fn get_project_secrets(client: &reqwest::Client, request: models::GetProjectSecretsRequest) -> Result<models::GetProjectSecretsResponse, reqwest::Error> {
    let endpoint = format!("v2/secrets");
    client.request(Method::GET, endpoint).body(request).send().await.json::<models::GetProjectSecretsResponse>()
}
