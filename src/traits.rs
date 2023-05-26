use crate::{
    api::secrets::{RawSecret, SecretToUpdate},
    error::Result,
    utils::aes256gcm::encrypt,
};

pub trait Client {
    fn encrypt_secrets(
        &self,
        secrets: Vec<RawSecret>,
        project_key: &str,
    ) -> Result<Vec<SecretToUpdate>> {
        secrets
            .iter()
            .map(|secret| self.encrypt_secret(secret, project_key))
            .collect()
    }

    fn encrypt_secret(
        &self,
        secret: &RawSecret,
        project_key: &str
    ) -> Result<SecretToUpdate> {
        let comment = match &secret.comment {
            Some(comment) => comment,
            None => "",
        };

        Ok(SecretToUpdate {
            name: secret.name.clone(),
            type_name: secret.type_name.clone(),
            key: encrypt(&secret.key, &project_key)?.into(),
            value: encrypt(&secret.value, &project_key)?.into(),
            comment: encrypt(&comment, &project_key)?.into(),
        })
    }
}
