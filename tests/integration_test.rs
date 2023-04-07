use std::vec;

use infisical_rs::{api::models::SecretToCreate, utils::aes256gcm::encrypt};
use tokio;
mod common;

#[tokio::test]
async fn get_decrypted_project_secrets() {
    let env_vars = common::setup().unwrap();

    let infisical_client = infisical_rs::Client::new(&env_vars.api_key).unwrap();

    let private_key = infisical_client
        .get_private_key(&env_vars.secret)
        .await
        .unwrap();
    println!("private key: {}", private_key);

    let project_key = infisical_client
        .get_decrypted_project_key(&env_vars.workspace_id, &private_key)
        .await
        .unwrap();
    println!("project_key: {project_key}");

    let decrypted_keys = infisical_client
        .get_decrypted_project_secrets(&env_vars.workspace_id, &env_vars.environment, &project_key)
        .await
        .unwrap();
    println!("Project keys:");
    for sec in decrypted_keys.iter() {
        println!("{}: {}", sec.key, sec.value);
    }
}

#[tokio::test]
async fn add_secret() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let private_key = client.get_private_key(&env_vars.secret).await.unwrap();
    let project_key = client
        .get_decrypted_project_key(&env_vars.workspace_id, &private_key)
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
        .create_project_secrets(&env_vars.workspace_id, &env_vars.environment, vec![secret])
        .await
        .unwrap();
}

#[tokio::test]
async fn get_my_user() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _user = client.get_user().await.unwrap();
}

#[tokio::test]
async fn get_my_organizations() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _orgs = client.get_my_organizations().await.unwrap();
}

#[tokio::test]
async fn get_organization_projects() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _projects = client
        .get_organization_projects(&env_vars.organization_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_organization_memberships() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _memberships = client
        .get_organization_memberships(&env_vars.organization_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_project_memberships() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _proj_memberships = client
        .get_project_memberships(&env_vars.workspace_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_project_snapshots() {
    let env_vars = common::setup().unwrap();

    let client = infisical_rs::Client::new(&env_vars.api_key).unwrap();
    let _snapshots = client
        .get_project_snapshots(&env_vars.workspace_id, "0", "25")
        .await
        .unwrap();
}
