use crate::api::{
    secrets::*,
    organizations::*,
    projects::*,
    users::*
};
use crate::error::{self, Result};
use crate::traits::Client;
use crate::utils;
use crate::enums::SecretType;
use onionsalt::crypto;
use reqwest::header;

/// `Client` provides a wrapper around the Infisical API that gives easy access to its endpoints
pub struct ApiTokenClient {
    http_client: reqwest::Client,
    api_base: String,
}

impl ApiTokenClient {
    /// Constructs a new `Client` using the default Infisical Cloud API endpoint and reqwest Client
    pub fn new(api_key: &str) -> Result<ApiTokenClient> {
        ApiTokenClientBuilder::new().build(api_key)
    }

    /// Creates a new `ClientBuilder` to allow for `Client` customization.
    ///
    /// This is the same as `ClientBuilder::new()`.
    pub fn builder() -> ApiTokenClientBuilder {
        ApiTokenClientBuilder::new()
    }

    pub async fn get_user(&self) -> Result<User> {
        let request = GetMyUserRequest {
            base_url: self.api_base.clone(),
        };

        let response = get_my_user(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.user)
    }

    pub async fn get_my_organizations(&self) -> Result<Vec<Organization>> {
        let request = GetMyOrganizationsRequest {
            base_url: self.api_base.clone(),
        };

        let response = get_my_organizations(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.organizations)
    }

    pub async fn get_organization_memberships(
        &self,
        organization_id: &str,
    ) -> Result<Vec<OrganizationMembership>> {
        let request = GetOrganizationMembershipsRequest {
            base_url: self.api_base.clone(),
            organization_id: organization_id.to_string(),
        };

        let response = get_organization_memberships(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.memberships)
    }

    pub async fn update_organization_membership(
        &self,
        organization_id: &str,
        membership_id: &str,
        role: &str,
    ) -> Result<OrganizationMembership> {
        let request = UpdateOrganizationMembershipRequest {
            base_url: self.api_base.clone(),
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
            role: role.to_string(),
        };

        let response = update_organization_membership(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn delete_organization_membership(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<OrganizationMembership> {
        let request = DeleteOrganizationMembershipRequest {
            base_url: self.api_base.clone(),
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
        };

        let response = delete_organization_membership(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn get_organization_projects(
        &self,
        organization_id: &str,
    ) -> Result<Vec<Workspace>> {
        let request = GetProjectsRequest {
            base_url: self.api_base.clone(),
            organization_id: organization_id.to_string(),
        };

        let response = get_organization_projects(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.workspaces)
    }

    pub async fn get_project_memberships(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ProjectMembership>> {
        let request = GetProjectMembershipsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
        };

        let response = get_project_memberships(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.memberships)
    }

    pub async fn update_project_membership(
        &self,
        workspace_id: &str,
        membership_id: &str,
        role: &str,
    ) -> Result<ProjectMembership> {
        let request = UpdateProjectMembershipRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            membership_id: membership_id.to_string(),
            role: role.to_string(),
        };

        let response = update_project_membership(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn delete_project_membership(
        &self,
        workspace_id: &str,
        membership_id: &str,
    ) -> Result<ProjectMembership> {
        let request = DeleteProjectMembershipRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            membership_id: membership_id.to_string(),
        };

        let response = delete_project_membership(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn get_encrypted_project_key(
        &self,
        workspace_id: &str,
    ) -> Result<GetProjectKeyResponse> {
        let request = GetProjectKeyRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
        };

        get_project_key(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)
    }

    pub async fn get_decrypted_project_key(
        &self,
        workspace_id: &str,
        private_key: &str,
    ) -> Result<String> {
        let response = self.get_encrypted_project_key(workspace_id).await?;

        let mut encrypted_project_key = vec![0; 16];
        encrypted_project_key.extend(utils::base64::decode(&response.encrypted_key));
        let project_nonce = utils::base64::decode(&response.nonce);
        let public_key = utils::base64::decode(&response.sender.public_key);
        let private_key = utils::base64::decode(&private_key);

        let project_nonce: [u8; 24] = project_nonce[..24]
            .try_into()
            .map_err(crate::error::decrypt)?;
        let public_key: [u8; 32] = public_key[..32].try_into().map_err(crate::error::decrypt)?;
        let private_key: [u8; 32] = private_key[..32]
            .try_into()
            .map_err(crate::error::decrypt)?;

        let project_nonce = crypto::Nonce(project_nonce);
        let public_key = crypto::PublicKey(public_key);
        let private_key = crypto::SecretKey(private_key);

        let mut project_key = Vec::with_capacity(encrypted_project_key.len());
        for _ in 0..encrypted_project_key.len() {
            project_key.push(0)
        }
        crypto::box_open(
            &mut project_key,
            &encrypted_project_key,
            &project_nonce,
            &public_key,
            &private_key,
        )?;
        project_key.drain(..32);
        Ok(String::from_utf8(project_key).map_err(crate::error::utf8)?)
    }

    pub async fn get_project_logs(
        &self,
        workspace_id: &str,
        user_id: &str,
        offset: &str,
        limit: &str,
        sort_by: &str,
        action_names: &str,
    ) -> Result<Vec<ProjectLog>> {
        let request = GetProjectLogsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            user_id: user_id.to_string(),
            offset: offset.to_string(),
            limit: limit.to_string(),
            sort_by: sort_by.to_string(),
            action_names: action_names.to_string(),
        };

        let response = get_project_logs(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.logs)
    }

    pub async fn get_project_snapshots(
        &self,
        workspace_id: &str,
        offset: &str,
        limit: &str,
    ) -> Result<Vec<SecretSnapshot>> {
        let request = GetProjectSnapshotsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            offset: offset.to_string(),
            limit: limit.to_string(),
        };

        let response = get_project_snapshots(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.secret_snapshots)
    }

    pub async fn roll_back_to_snapshot(
        &self,
        workspace_id: &str,
        version: u8,
    ) -> Result<Vec<EncryptedSecret>> {
        let request = RollbackProjectToSnapshotRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            version,
        };

        let response = roll_back_to_snapshot(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn create_secret(
        &self,
        workspace_id: &str,
        environment: &str,
        secret: SecretToUpdate,
    ) -> Result<EncryptedSecret> {
        let request = CreateSecretRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            environment: environment.to_string(),
            secret,
        };

        let response = create_secret(&self.http_client, request)
            .await
            .map_err(error::reqwest)?;

        Ok(response.secret)
    }

    #[deprecated]
    pub async fn create_project_secrets(
        &self,
        workspace_id: &str,
        environment: &str,
        secrets: Vec<SecretToUpdate>,
    ) -> Result<Vec<EncryptedSecret>> {
        let request = CreateSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            environment: environment.to_string(),
            secrets,
        };

        let response = create_project_secrets(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn update_secret(
        &self,
        secret: SecretToUpdate,
        secret_name: &str
    ) -> Result<EncryptedSecret> {
        let request = UpdateSecretRequest {
            base_url: self.api_base.clone(),
            secret,
        };

        let response = update_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    #[deprecated]
    pub async fn update_project_secrets(
        &self,
        secrets: Vec<SecretToUpdate>,
    ) -> Result<Vec<EncryptedSecret>> {
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
        workspace_id: &str,
        environment: &str,
    ) -> Result<EncryptedSecret> {
        let request = DeleteSecretRequest {
            base_url: self.api_base.clone(),
            secret_name: secret_name.to_string(),
            type_name: secret_type.clone(),
            workspace_id: workspace_id.to_string(),
            environment: environment.to_string()
        };

        let response = delete_secret(&self.http_client, request).await?;

        Ok(response.secret)
    }

    pub async fn get_encrypted_project_secrets(
        &self,
        workspace_id: &str,
        environment: &str,
    ) -> Result<Vec<EncryptedSecret>> {
        let request = GetSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            environment: environment.to_string(),
        };

        let response = get_secrets(&self.http_client, request)
            .await
            .map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn get_decrypted_project_secrets(
        &self,
        workspace_id: &str,
        environment: &str,
        private_key: &str,
    ) -> Result<Vec<DecryptedSecret>> {
        let encrypted_secrets: Vec<EncryptedSecret> = self
            .get_encrypted_project_secrets(workspace_id, environment)
            .await?;

        encrypted_secrets
            .iter()
            .map(|enc_secret| EncryptedSecret::decrypt(enc_secret, private_key))
            .collect()
    }

    pub async fn get_user_decrypted_private_key(&self, infisical_secret: &str) -> Result<String> {
        let user = self.get_user().await?;
        utils::aes256gcm::decrypt(
            &user.encrypted_private_key,
            &user.iv,
            &user.tag,
            &infisical_secret,
        )
    }
}

impl Client for ApiTokenClient {}

/// `ClientBuilder` can be used to create a `Client` with a custom API endpoint and/or [`Reqwest
/// Client`]
///
/// [`Reqwest Client`]: reqwest::Client
pub struct ApiTokenClientBuilder {
    api_base: String,
    reqwest_client_builder: Option<reqwest::ClientBuilder>,
}

impl Default for ApiTokenClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiTokenClientBuilder {
    pub fn new() -> ApiTokenClientBuilder {
        ApiTokenClientBuilder {
            api_base: String::from("https://app.infisical.com/api"),
            reqwest_client_builder: None,
        }
    }

    pub fn build(mut self, api_key: &str) -> Result<ApiTokenClient> {
        // If a custom client was not provided then we create our own default client
        if self.reqwest_client_builder.is_none() {
            self.reqwest_client_builder = Some(reqwest::ClientBuilder::new());
        }

        // Add the API key as a default header since every endpoint expects it
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::try_from(api_key).map_err(error::reqwest)?,
        );

        match self.reqwest_client_builder {
            Some(mut reqwest_client_builder) => {
                reqwest_client_builder = reqwest_client_builder.default_headers(headers);
                Ok(ApiTokenClient {
                    http_client: reqwest_client_builder
                        .build()
                        .map_err(crate::error::builder)?,
                    api_base: self.api_base.clone(),
                })
            }
            None => unreachable!("There will always be a reqwest_client_builder at this point"),
        }
    }

    pub fn api_base(mut self, value: &str) -> ApiTokenClientBuilder {
        self.api_base = value.to_string();
        self
    }

    /// Setter for the reqwest_client_builder struct member
    pub fn reqwest_client_builder(
        mut self,
        value: reqwest::ClientBuilder,
    ) -> ApiTokenClientBuilder {
        self.reqwest_client_builder = Some(value);
        self
    }
}
