use crate::error::{self, api, Result};
use crate::api::JsonProcessorExt;

mod models;
pub use models::*;

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
