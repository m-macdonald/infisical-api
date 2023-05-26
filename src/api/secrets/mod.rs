use serde_json::Value;
use time::{
    serde::iso8601, 
    OffsetDateTime
};
use serde::{Serialize, Deserialize};

use crate::enums::SecretType;
use crate::utils::aes256gcm::{decrypt, Encryption};
use crate::error::{self, api, Result};
use crate::api::JsonProcessorExt;

mod models;
pub use models::*;

pub async fn create_secret(
    client: &reqwest::Client,
    request: models::CreateSecretRequest
) -> Result<CreateSecretResponse> {
    let endpoint = format!("{}/v3/secrets/{}", request.base_url, request.secret.name);

    Ok(client
        .post(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<CreateSecretResponse>()
        .await?)
}

#[deprecated]
pub async fn create_project_secrets(
    client: &reqwest::Client,
    request: models::CreateSecretsRequest,
) -> Result<models::CreateSecretsResponse> {
    let endpoint = format!("{}/v2/secrets", request.base_url);

    Ok(client
        .post(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<CreateSecretsResponse>()
        .await?)
}

pub async fn update_secret(
    client: &reqwest::Client,
    request: models::UpdateSecretRequest,
) -> Result<models::UpdateSecretResponse> {
    let endpoint = format!("{}/v3/secrets/{}", request.base_url, request.secret.name);

    Ok(client
        .patch(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<UpdateSecretResponse>()
        .await?)
}

#[deprecated]
pub async fn update_project_secrets(
    client: &reqwest::Client,
    request: UpdateSecretsRequest,
) -> Result<UpdateSecretsResponse> {
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
pub async fn get_secrets(
    client: &reqwest::Client,
    request: models::GetSecretsRequest,
) -> Result<models::GetSecretsResponse> {
    let endpoint = format!("{}/v3/secrets", request.base_url);

    println!("{:#?}", client.get(&endpoint).query(&request));

    Ok(client
        .get(endpoint)
        .query(&request)
        .send()
        .await?
        .infisical_json::<GetSecretsResponse>()
        .await?)
}

pub async fn get_secret(
    client: &reqwest::Client,
    request: GetSecretRequest,
) -> Result<GetSecretResponse> {
    let endpoint = format!("{}/v3/secrets/{}", request.base_url, request.secret_name);

    Ok(client
        .get(endpoint)
        .query(&request)
        .send()
        .await?
        .infisical_json::<GetSecretResponse>()
        .await?)
}

pub async fn delete_secret(
    client: &reqwest::Client,
    request: models::DeleteSecretRequest
) -> Result<DeleteSecretResponse> {
    let endpoint = format!("{}/v3/secrets/{}", request.base_url, request.secret_name);

    Ok(client
        .delete(endpoint)
        .json(&request)
        .send()
        .await?
        .infisical_json::<models::DeleteSecretResponse>()
        .await?)
}
