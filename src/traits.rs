use crate::{
    api::models::{RawSecret, SecretToUpdate},
    error::Result,
    utils::aes256gcm::encrypt,
};

#[async_trait::async_trait]
pub trait Client {
    async fn encrypt_secrets(
        &self,
        secrets: Vec<RawSecret>,
        project_key: &str,
    ) -> Result<Vec<SecretToUpdate>> {
        secrets
            .iter()
            .map(|secret| -> Result<SecretToUpdate> {
                Ok(SecretToUpdate {
                    id: secret.id.clone(),
                    type_name: secret.type_name.clone(),
                    key: encrypt(&secret.key, &project_key)?.into(),
                    value: encrypt(&secret.value, &project_key)?.into(),
                    comment: encrypt(&secret.comment, &project_key)?.into(),
                })
            })
            .collect()
    }
}
