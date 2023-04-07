use dotenvy::dotenv;
use std::env::{var, VarError};

// Using this as an example of how testing could be done
// This module provides an example of a setup that could be called and run from other tests

pub fn setup() {
    // This function can do setup for any functions that call it
    dotenv().unwrap();
}

pub fn load_env_vars() -> Result<(String, String, String, String, String), VarError> {
    let workspace_id = var("WORKSPACE_ID")?;
    let api_key = var("API_KEY")?;
    let secret = var("SECRET")?;
    let environment = var("ENVIRONMENT")?;
    let organization_id = var("ORGANIZATION_ID")?;

    Ok((workspace_id, api_key, secret, environment, organization_id))
}
