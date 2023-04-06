use std::{env, vec};

use infisical_rs::{api::models::SecretToCreate, utils::aes256gcm::encrypt};
use tokio;
mod common;

// This file can run integration tests
#[tokio::test]
async fn basic() {
    common::setup();
    let (workspace_id, api_key, secret, environment) = common::load_env_vars().unwrap();

    let infisical_client = infisical_rs::Client::new(&api_key).unwrap();

    let private_key = infisical_client.get_private_key(&secret).await.unwrap();
    println!("private key: {}", private_key);

    let project_key = infisical_client
        .get_decrypted_project_key(&workspace_id, &private_key)
        .await
        .unwrap();
    println!("project_key: {project_key}");

    let decrypted_keys = infisical_client
        .get_decrypted_project_secrets(&workspace_id, &environment, &project_key)
        .await
        .unwrap();
    println!("Project keys:");
    for sec in decrypted_keys.iter() {
        println!("{}: {}", sec.key, sec.value);
    }
}

#[tokio::test]
async fn add_secret() {
    common::setup();
    let (workspace_id, api_key, secret, environment) = common::load_env_vars().unwrap();

    let client = infisical_rs::Client::new(&api_key).unwrap();
    let private_key = client.get_private_key(&secret).await.unwrap();
    let project_key = client
        .get_decrypted_project_key(&workspace_id, &private_key)
        .await
        .unwrap();

    let secret = SecretToCreate {
        key: encrypt("EXTRA_SPECIAL_SECRET", &project_key)
            .unwrap()
            .into(),
        value: encrypt("THIS IS THE VALUE OF THE SECRET", &project_key)
            .unwrap()
            .into(),
        comment: encrypt("This is my comment for the secret", &project_key)
            .unwrap()
            .into(),
        secret_type: "shared".to_string(),
    };

    let _result = client
        .create_project_secrets(&workspace_id, &environment, vec![secret])
        .await
        .unwrap();
}
