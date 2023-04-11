use dotenvy::dotenv;
use std::env::{var, VarError};

pub fn setup() -> Result<EnvVars, VarError> {
    dotenv().unwrap();

    Ok(EnvVars {
        workspace_id: var("WORKSPACE_ID")?,
        api_key: var("API_KEY")?,
        secret: var("SECRET")?,
        environment: var("ENVIRONMENT")?,
        organization_id: var("ORGANIZATION_ID")?,
        service_token: var("SERVICE_TOKEN")?,
    })
}

pub struct EnvVars {
    pub workspace_id: String,
    pub api_key: String,
    pub secret: String,
    pub environment: String,
    pub organization_id: String,
    pub service_token: String,
}
