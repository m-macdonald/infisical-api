use crate::error::Result;
use crate::api::JsonProcessorExt;

mod models;
pub use models::*;

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
