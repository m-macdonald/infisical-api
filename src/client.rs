use crate::utils;
use crate::api;
use crate::error::Result;

use onionsalt::crypto;
use reqwest::header;

pub struct InfisicalClient {
    http_client: reqwest::Client,
    api_base: String,
}

impl InfisicalClient {

    pub fn new(api_key: String) -> Result<InfisicalClient> {
       InfisicalClientBuilder::new().build(api_key)
    }

    pub fn builder() -> InfisicalClientBuilder {
        InfisicalClientBuilder::new()
    }

    pub async fn get_user(&self) ->  Result<api::models::User> {
        let request = api::models::GetMyUserRequest {
            base_url: self.api_base.clone()
        };

        let response = api::get_my_user(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.user)
    }

    pub async fn get_my_organizations(&self) -> Result<Vec<api::models::Organization>> {
        let request = api::models::GetMyOrganizationsRequest {
            base_url: self.api_base.clone()
        };

        let response = api::get_my_organizations(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.organizations)
    }

    pub async fn get_organization_memberships(&self, organization_id: String) -> Result<Vec<api::models::OrganizationMembership>> {
        let request = api::models::GetOrganizationMembershipsRequest {
            base_url: self.api_base.clone(), 
            organization_id
        };

        let response = api::get_organization_memberships(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.memberships)
    }

    pub async fn update_organization_membership(&self, organization_id: String, membership_id: String, role: String) -> Result<api::models::OrganizationMembership> {
        let request = api::models::UpdateOrganizationMembershipRequest {
            base_url: self.api_base.clone(),
            organization_id,
            membership_id,
            role
        };

        let response = api::update_organization_membership(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn delete_organization_membership(&self, organization_id: String, membership_id: String) -> Result<api::models::OrganizationMembership> {
        let request = api::models::DeleteOrganizationMembershipRequest {
            base_url: self.api_base.clone(),
            organization_id,
            membership_id
        };

        let response = api::delete_organization_membership(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn get_organization_projects(&self, organization_id: String) -> Result<Vec<api::models::Workspace>> {
        let request = api::models::GetProjectsRequest {
            base_url: self.api_base.clone(),
            organization_id
        };

        let response = api::get_organization_projects(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.workspaces)
    }

    pub async fn get_project_memberships(&self, workspace_id: String) -> Result<Vec<api::models::ProjectMembership>> {
        let request = api::models::GetProjectMembershipsRequest {
            base_url: self.api_base.clone(),
            workspace_id
        };

        let response = api::get_project_memberships(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.memberships)
    }

    pub async fn update_project_membership(&self, workspace_id: String, membership_id: String, role: String) -> Result<api::models::ProjectMembership> {
        let request = api::models::UpdateProjectMembershipRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            membership_id,
            role
        };

        let response = api::update_project_membership(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn delete_project_membership(&self, workspace_id: String, membership_id: String) -> Result<api::models::ProjectMembership> {
        let request = api::models::DeleteProjectMembershipRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            membership_id
        };

        let response = api::delete_project_membership(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.membership)
    }

    pub async fn get_encrypted_project_key(&self, workspace_id: &String) -> Result<api::models::GetProjectKeyResponse> {
        let request = api::models::GetProjectKeyRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.clone()
        };

        api::get_project_key(&self.http_client, request).await.map_err(crate::error::reqwest)
    }

    pub async fn get_decrypted_project_key(&self, workspace_id: &String, private_key: &String) -> Result<String> {
        // TODO: implement better error handling here 
        let response = self.get_encrypted_project_key(workspace_id).await?;

        let mut encrypted_project_key = vec![0; 16];
        encrypted_project_key.extend(utils::base64::decode(&response.encrypted_key));
        let project_nonce = utils::base64::decode(&response.nonce);
        let public_key = utils::base64::decode(&response.sender.public_key);
        let private_key = utils::base64::decode(&private_key);

        // TODO: This really needs better error handling
        let project_nonce: [u8; 24] = project_nonce[..24].try_into().map_err(crate::error::decrypt)?;
        let public_key: [u8; 32] = public_key[..32].try_into().map_err(crate::error::decrypt)?;
        let private_key: [u8; 32] = private_key[..32].try_into().map_err(crate::error::decrypt)?;
        
        let project_nonce = crypto::Nonce(project_nonce);
        let public_key = crypto::PublicKey(public_key);
        let private_key = crypto::SecretKey(private_key);

        let mut project_key = Vec::with_capacity(encrypted_project_key.len());
        for _ in 0..encrypted_project_key.len() {project_key.push(0)}
        crypto::box_open(&mut project_key, &encrypted_project_key, &project_nonce, &public_key, &private_key)?;
        project_key.drain(..32);
        Ok(String::from_utf8(project_key).map_err(crate::error::utf8)?)
    }

    pub async fn get_project_logs(&self, workspace_id: String, user_id: String, offset: String, limit: String, sort_by: String, action_names: String) -> Result<Vec<api::models::ProjectLog>> {
        let request = api::models::GetProjectLogsRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            user_id,
            offset,
            limit,
            sort_by,
            action_names
        };

        let response = api::get_project_logs(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.logs)
    }

    pub async fn get_project_snapshots(&self, workspace_id: String, offset: String, limit: String) -> Result<Vec<api::models::SecretSnapshot>> {
        let request = api::models::GetProjectSnapshotsRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            offset,
            limit
        };

        let response = api::get_project_snapshots(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.secret_snapshots)
    }

    pub async fn roll_back_to_snapshot(&self, workspace_id: String, version: u8) -> Result<Vec<api::models::EncryptedSecret>> {
        let request = api::models::RollbackProjectToSnapshotRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            version
        };

        let response = api::roll_back_to_snapshot(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn create_project_secrets(&self, workspace_id: String, environment: String, secrets: Vec<api::models::SecretToCreate>) -> Result<Vec<api::models::EncryptedSecret>> {
        let request = api::models::CreateProjectSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id,
            environment,
            secrets
        };

        let response = api::create_project_secrets(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn get_encrypted_project_secrets(&self, workspace_id: &str, environment: &str) -> Result<Vec<api::models::EncryptedSecret>> {
        let request = api::models::GetProjectSecretsRequest {
            base_url: self.api_base.clone(),
            workspace_id: workspace_id.to_string(),
            environment: environment.to_string(),
            content: String::from("")
        };

        let response = api::get_project_secrets(&self.http_client, request).await.map_err(crate::error::reqwest)?;

        Ok(response.secrets)
    }

    pub async fn get_decrypted_project_secrets(&self, workspace_id: &str, environment: &str, private_key: &str) -> Result<Vec<api::models::DecryptedSecret>> {
        // TODO: Add better error handling
        let encrypted_secrets = self.get_encrypted_project_secrets(workspace_id, environment).await?;

        encrypted_secrets.iter().map(|enc_secret| api::models::EncryptedSecret::decrypt(enc_secret, private_key)).collect()
    }

    pub async fn get_private_key(&self, infisical_secret: String) -> Result<String> {
        let user = self.get_user().await?;
        utils::aes256gcm::decrypt(&user.encrypted_private_key, &user.iv, &user.tag, &infisical_secret)
    }

/*    pub async fn old_get_project_key(&self, workspace_id: String, private_key: String) -> Result<String, Box<dyn Error>> {
        let request_builder = self.new_request(reqwest::Method::GET, format!("{}/{}/{}/encrypted-key", self.api_base, "workspace", workspace_id));
        let workspace = self.send_request(request_builder.build()?).await?.json::<models::WorkspaceEncryptedKey>().await?;
        // crypto::box_open expects 16 bytes of 0 at the beginning
        let mut encrypted_project_key = vec![0; 16];
        encrypted_project_key.extend(utils::base64::decode(&workspace.encrypted_key));
        let project_nonce = utils::base64::decode(&workspace.nonce);
        let public_key = utils::base64::decode(&workspace.sender.public_key); 
        let private_key = utils::base64::decode(&private_key);
        // TODO: This really needs better handling in the event that the given vecs are too small
        let project_nonce: [u8; 24] = project_nonce[..24].try_into().unwrap();
        let public_key: [u8; 32] = public_key[..32].try_into().unwrap();
        let private_key: [u8; 32] = private_key[..32].try_into().unwrap();
        let private_key = crypto::SecretKey(private_key);
        let public_key = crypto::PublicKey(public_key);
        let project_nonce = crypto::Nonce(project_nonce);

        let mut project_key: Vec<u8> = Vec::with_capacity(encrypted_project_key.len());
        for _ in 0..encrypted_project_key.len() {project_key.push(0)}
        crypto::box_open(&mut project_key, &encrypted_project_key, &project_nonce, &public_key, &private_key).unwrap();
        project_key.drain(..32);
        Ok(String::from_utf8(project_key).unwrap())
    }
    */
}

//allows a custom client to be provided if the default is not desired
pub struct InfisicalClientBuilder {
    api_base: String,
    reqwest_client_builder: Option<reqwest::ClientBuilder>
}

impl Default for InfisicalClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl InfisicalClientBuilder {
    pub fn new() -> InfisicalClientBuilder {
        InfisicalClientBuilder { api_base: String::from("https://app.infisical.com/api"), reqwest_client_builder: None }
    }

    pub fn build(mut self, api_key: String) -> Result<InfisicalClient> {
        
        // If a custom client was not provided then we create our own default client
        if self.reqwest_client_builder.is_none() {
            self.reqwest_client_builder = Some(reqwest::ClientBuilder::new());
        }

        // Add the API key as a default header since every endpoint expects it
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::try_from(api_key).map_err(crate::error::reqwest)?);

        match self.reqwest_client_builder {
            Some(mut reqwest_client_builder) => {
                reqwest_client_builder = reqwest_client_builder.default_headers(headers);
                Ok(InfisicalClient {
                    http_client: reqwest_client_builder.build().map_err(crate::error::builder)?,
                    api_base: self.api_base.clone(),
                })
            },
            None => unreachable!("There will always be a reqwest_client_builder at this point")
        }
    }

    pub fn api_base(mut self, value: String) -> InfisicalClientBuilder {
        self.api_base = value;
        self
    }
}
