use reqwest::header;

use crate::{
    api::{
        secrets::*,
        projects::*,
        service_tokens::*
   },
    error::{self, Result},
    traits::Client,
    utils,
    enums::SecretType
};

/// A variation on [`ApiTokenClient`] that provides more limited access to the Infisical API.
/// The `ServiceTokenClent` utilizes an Infisical Service Token that is restricted to a specific project and environment
/// within that project.
/// This is likely the right choice for an application that doesn't need to do project management
/// and is solely concerned with the project's secrets.
pub struct ServiceTokenClient {
    api_base: String,
    http_client: reqwest::Client,
    project_id: String,
    environment: String,
    key: String,
}

impl ServiceTokenClient {
    /// Constructs a new `ServiceTokenClient` using the default Infisical Cloud API endpoint and
    /// reqwest Client
    pub async fn new(service_token: &str) -> Result<Self> {
        Ok(ServiceTokenClientBuilder::new()
            .build(service_token)
            .await?)
    }

    /// Creates a new `ServiceTokenClientBuilder` to allow for `ServiceTokenClient` customization.
    pub fn builder() -> ServiceTokenClientBuilder {
        ServiceTokenClientBuilder::new()
    }

    pub async fn get_encrypted_secrets(&self) -> Result<Vec<EncryptedSecret>> {
        let request = GetSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id: self.project_id.clone(),
            environment: self.environment.clone(),
        };
        let response = get_secrets(&self.http_client, request)
            .await
            .map_err(error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn get_encrypted_secret(&self, secret_name: &str, secret_type: &SecretType) -> Result<EncryptedSecret> {
        let request = GetSecretRequest {
            base_url: self.api_base.clone(),
            secret_name: secret_name.to_string(),
            secret_type: secret_type.clone(),
            workspace_id: self.project_id.clone(),
            environment: self.environment.clone()
        };

        let response = get_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    pub async fn get_secrets(&self) -> Result<Vec<DecryptedSecret>> {
        let encrypted_secrets: Vec<EncryptedSecret> = self.get_encrypted_secrets().await?;
        let project_key = self.get_project_key().await?;

        encrypted_secrets
            .iter()
            .map(|enc_secret| EncryptedSecret::decrypt(enc_secret, &project_key))
            .collect()
    }

    pub async fn get_secret(&self, secret_name: &str, secret_type: &SecretType) -> Result<DecryptedSecret> {
        let encrypted_secret = self.get_encrypted_secret(secret_name, secret_type).await?;
        let project_key = self.get_project_key().await?;

        EncryptedSecret::decrypt(&encrypted_secret, &project_key)
    }

    pub async fn create_secret(
        &self,
        secret: RawSecret
    ) -> Result<EncryptedSecret> {
        let project_key = self.get_project_key().await?;
        let secret = self.encrypt_secret(&secret, &project_key)?;

        let request = CreateSecretRequest {
            base_url: self.api_base.clone(),
            workspace_id: self.project_id.clone(),
            environment: self.environment.clone(),
            secret
        };

        let response = create_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    pub async fn create_secrets(
        &self,
        secrets: Vec<RawSecret>,
    ) -> Result<Vec<EncryptedSecret>> {
        let project_key = self.get_project_key().await?;
        let secrets = self.encrypt_secrets(secrets, &project_key)?;

        let request = CreateSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id: self.project_id.clone(),
            environment: self.environment.clone(),
            secrets,
        };

        let response = create_project_secrets(&self.http_client, request)
            .await
            .map_err(error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn update_secret(
        &self,
        secret: &RawSecret
    ) -> Result<EncryptedSecret> {
        let project_key = self.get_project_key().await?;
        let secret = self.encrypt_secret(secret, &project_key)?;

        let request = UpdateSecretRequest {
            base_url: self.api_base.clone(),
            secret
        };

        let response = update_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    pub async fn update_secrets(
        &self,
        secrets: Vec<RawSecret>,
    ) -> Result<Vec<EncryptedSecret>> {
        let project_key = self.get_project_key().await?;
        let secrets = self.encrypt_secrets(secrets, &project_key)?;

        let request = UpdateSecretsRequest {
            base_url: self.api_base.clone(),
            secrets,
        };

        let response = update_project_secrets(&self.http_client, request).await?;

        Ok(response.secrets)
    }

    pub async fn delete_secret(
        &self,
        secret_name: &str,
        secret_type: &SecretType,
    ) -> Result<EncryptedSecret> {
        let request = DeleteSecretRequest {
            base_url: self.api_base.clone(),
            secret_name: secret_name.to_string(),
            type_name: secret_type.clone(),
            workspace_id: self.project_id.clone(),
            environment: self.environment.clone()
        };

        let response = delete_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    async fn get_project_key(&self) -> Result<String> {
        let service_token_details = self.get_service_token_details().await?;

        utils::aes256gcm::decrypt(
            &service_token_details.encrypted_key,
            &service_token_details.iv,
            &service_token_details.tag,
            &self.key,
        )
    }

    pub async fn get_service_token_details(&self) -> Result<ServiceToken> {
        let request = GetServiceTokensRequest {
            base_url: self.api_base.clone(),
        };

        let service_token_details = get_service_token(&self.http_client, request).await?;

        Ok(service_token_details)
    }
}

impl Client for ServiceTokenClient {}

pub struct ServiceTokenClientBuilder {
    api_base: String,
    reqwest_client_builder: Option<reqwest::ClientBuilder>,
}

impl Default for ServiceTokenClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTokenClientBuilder {
    pub fn new() -> ServiceTokenClientBuilder {
        ServiceTokenClientBuilder {
            api_base: String::from("https://app.infisical.com/api"),
            reqwest_client_builder: None,
        }
    }

    pub async fn build(mut self, service_token: &str) -> Result<ServiceTokenClient> {
        if self.reqwest_client_builder.is_none() {
            self.reqwest_client_builder = Some(reqwest::ClientBuilder::new());
        }

        let last_index = match service_token.rfind('.') {
            Some(index) => index,
            None => Err(error::malformed_service_token())?,
        };
        let key = &service_token[last_index + 1..];
        let token = &service_token[..last_index];
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::try_from(format!("Bearer {}", token)).map_err(error::reqwest)?,
        );

        match self.reqwest_client_builder {
            Some(mut reqwest_client_builder) => {
                reqwest_client_builder = reqwest_client_builder.default_headers(headers);
                let http_client = reqwest_client_builder.build().map_err(error::builder)?;
                let request = GetServiceTokensRequest {
                    base_url: self.api_base.clone(),
                };
                let service_token_details = get_service_token(&http_client, request).await?;

                Ok(ServiceTokenClient {
                    api_base: self.api_base.clone(),
                    http_client,
                    project_id: service_token_details.workspace,
                    environment: service_token_details.environment,
                    key: key.to_string(),
                })
            }
            None => unreachable!("There will always be a reqwest_client_builder at this point"),
        }
    }

    pub fn api_base(mut self, value: &str) -> ServiceTokenClientBuilder {
        self.api_base = value.to_string();
        self
    }

    pub fn reqwest_client_builder(mut self, value: reqwest::ClientBuilder) -> ServiceTokenClientBuilder {
        self.reqwest_client_builder = Some(value);
        self
    }
}
