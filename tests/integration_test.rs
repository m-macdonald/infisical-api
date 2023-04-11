use std::vec;

use infisical_api::{
    api::models::{RawSecret, SecretToUpdate},
    enums::SecretType,
    utils::aes256gcm::encrypt,
};
use tokio;
mod common;

#[tokio::test]
async fn get_decrypted_project_secrets() {
    let env_vars = common::setup().unwrap();

    let infisical_client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();

    let private_key = infisical_client
        .get_user_decrypted_private_key(&env_vars.secret)
        .await
        .unwrap();

    let project_key = infisical_client
        .get_decrypted_project_key(&env_vars.workspace_id, &private_key)
        .await
        .unwrap();

    let _decrypted_keys = infisical_client
        .get_decrypted_project_secrets(&env_vars.workspace_id, &env_vars.environment, &project_key)
        .await
        .unwrap();
}

#[tokio::test]
async fn add_secret() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let private_key = client
        .get_user_decrypted_private_key(&env_vars.secret)
        .await
        .unwrap();
    let project_key = client
        .get_decrypted_project_key(&env_vars.workspace_id, &private_key)
        .await
        .unwrap();

    let secret = SecretToUpdate {
        id: None,
        key: encrypt("EXTRA_SPECIAL_SECRET", &project_key)
            .unwrap()
            .into(),
        value: encrypt("THIS IS THE VALUE OF THE SECRET", &project_key)
            .unwrap()
            .into(),
        comment: encrypt("This is my comment for the secret", &project_key)
            .unwrap()
            .into(),
        type_name: Some(SecretType::Shared),
    };

    let _result = client
        .create_project_secrets(&env_vars.workspace_id, &env_vars.environment, vec![secret])
        .await
        .unwrap();
}

#[tokio::test]
async fn get_my_user() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _user = client.get_user().await.unwrap();
}

#[tokio::test]
async fn get_my_organizations() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _orgs = client.get_my_organizations().await.unwrap();
}

#[tokio::test]
async fn get_organization_projects() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _projects = client
        .get_organization_projects(&env_vars.organization_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_organization_memberships() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _memberships = client
        .get_organization_memberships(&env_vars.organization_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_project_memberships() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _proj_memberships = client
        .get_project_memberships(&env_vars.workspace_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_project_snapshots() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _snapshots = client
        .get_project_snapshots(&env_vars.workspace_id, "0", "25")
        .await
        .unwrap();
}

#[tokio::test]
async fn get_project_logs() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    let _logs = client.get_project_logs(&env_vars.workspace_id, "", "0", "25", "", "");
}

#[tokio::test]
async fn get_secret_versions() {
    let env_vars = common::setup().unwrap();

    let _client = infisical_api::ApiTokenClient::new(&env_vars.api_key).unwrap();
    // let _versions = client.get
}

#[tokio::test]
async fn get_service_token_details() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ServiceTokenClient::new(&env_vars.service_token)
        .await
        .unwrap();
    let _service_token_details = client.get_service_token_details();
}

#[tokio::test]
async fn get_secrets() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ServiceTokenClient::new(&env_vars.service_token)
        .await
        .unwrap();

    let _secrets = client.get_decrypted_secrets().await.unwrap();
}

#[tokio::test]
async fn create_secrets() {
    let env_vars = common::setup().unwrap();

    let client = infisical_api::ServiceTokenClient::new(&env_vars.service_token)
        .await
        .unwrap();

    let _secrets = client
        .create_secrets(vec![
            RawSecret {
                id: None,
                type_name: Some(SecretType::Shared),
                key: "SECRET_FROM_SERVICE_TOKEN".to_string(),
                value: "Secret Value".to_string(),
                comment: "This secret was added by ServiceTokenClient.".to_string(),
            },
            RawSecret {
                id: None,
                type_name: Some(SecretType::Shared),
                key: "ANOTHER_SECRET_FROM_SERVICE_TOKEN".to_string(),
                value: "Value #2".to_string(),
                comment: "Comment #2".to_string(),
            },
        ])
        .await
        .unwrap();
}
