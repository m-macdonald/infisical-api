use std::env;

use infisical;
use tokio;
mod common;

// This file can run integration tests
#[tokio::test]
async fn basic() {
    common::setup();

    let workspace_id = env::var("WORKSPACE_ID").unwrap();

    let infisical_client =
        infisical::client::InfisicalClient::new(env::var("API_KEY").unwrap()).unwrap();

    let private_key = infisical_client
        .get_private_key(env::var("SECRET").unwrap())
        .await
        .unwrap();
    println!("private key: {}", private_key);

    let project_key = infisical_client
        .get_decrypted_project_key(&workspace_id, &private_key)
        .await
        .unwrap();
    println!("project_key: {project_key}");

    let decrypted_keys = infisical_client
        .get_decrypted_project_secrets(
            &workspace_id,
            &env::var("ENVIRONMENT").unwrap(),
            &project_key,
        )
        .await
        .unwrap();
    println!("Project keys:");
    for sec in decrypted_keys.iter() {
        println!("{}: {}", sec.key, sec.value);
    }
}
