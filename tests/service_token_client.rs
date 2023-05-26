mod common;
use infisical_api::{
    api::secrets::{RawSecret, EncryptedSecret},
    enums::SecretType, ServiceTokenClient
};

use common::EnvVars;

async fn setup() -> (ServiceTokenClient, EnvVars) {
    let env_vars = common::setup().unwrap();

    let client = ServiceTokenClient::new(&env_vars.service_token)
        .await
        .unwrap();

    (client, env_vars)
}

#[tokio::test]
async fn get_encrypted_secrets() {
    let (client, env_vars) = setup().await;

    let _secrets = client.get_encrypted_secrets();

}

#[tokio::test]
async fn get_service_token_details() {
    let (client, env_vars) = setup().await;

    let _service_token_details = client.get_service_token_details().await.unwrap();
}

#[tokio::test]
async fn get_secrets() {
    let (client, env_vars) = setup().await;

    let _secrets = client.get_secrets().await.unwrap();
}

#[tokio::test]
async fn create_secret() {
    let (client, env_vars) = setup().await;

    let _secret = client.create_secret(
        RawSecret{
            name: "SERVICE_TOKEN_CREATE_SECRET".to_string(),
            type_name: SecretType::Shared,
            key: "SERVICE_TOKEN_CREATE_SECRET".to_string(),
            value: "service token create secret value".to_string(),
            comment: Some("Comment".to_string())
        }
        ).await.unwrap();
}

#[tokio::test]
async fn json() {
    let _json: infisical_api::api::ApiResponse<infisical_api::api::secrets::CreateSecretResponse> = serde_json::from_slice(b"{\"secret\":{\"version\":1,\"workspace\":\"646c09aca239863a2cd6a265\",\"type\":\"shared\",\"tags\":[],\"environment\":\"dev\",\"secretKeyCiphertext\":\"uEAVdZgpwmg9AACCcx6cLe9UYTstKOJql/cc\",\"secretKeyIV\":\"Yl3Ke6Ed2UzNuAAagbzUfg==\",\"secretKeyTag\":\"35nc34GR6OAV77jL0l2ntQ==\",\"secretValueCiphertext\":\"69/lPwO0MJ7mBs6GPgyfTX9V2yYRn4nXzcx6Msq674oQ\",\"secretValueIV\":\"6cZ7qqP5UGdIsS7x3T4CnA==\",\"secretValueTag\":\"Agd3rbUx81r1NKAd5HbzZA==\",\"secretCommentCiphertext\":\"fEQFRH00mQ==\",\"secretCommentIV\":\"5ai0m0q4nTLm5Gvk/AsMdw==\",\"secretCommentTag\":\"AND0+FZ1xxmh13MqEcgRYA==\",\"path\":\"/\",\"_id\":\"6470cb9e852edfb960f175cd\",\"createdAt\":\"2023-05-26T15:09:19.000Z\",\"updatedAt\":\"2023-05-26T15:09:19.000Z\",\"__v\":0}}").unwrap();
}
