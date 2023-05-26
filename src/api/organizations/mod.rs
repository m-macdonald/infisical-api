use crate::error::Result;
use crate::api::JsonProcessorExt;

mod models;
pub use models::*;

pub async fn get_organization_memberships(
    client: &reqwest::Client,
    request: models::GetOrganizationMembershipsRequest,
) -> Result<models::GetOrganizationMembershipsResponse> {
    let endpoint = format!(
        "{}/v2/organizations/{}/memberships",
        request.base_url, request.organization_id
    );

    let res = client.get(&endpoint).send().await?;

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
