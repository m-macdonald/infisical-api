use serde::{Serialize, Deserialize};
use time::{
    serde::iso8601, 
    OffsetDateTime
};

use crate::error::{self, api, Result};
use crate::api::Audit;
use crate::utils::aes256gcm::{
    Encryption,
    decrypt,
    encrypt
};
use crate::enums::SecretType;

#[deprecated]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecretsRequest {
    pub base_url: String,
    pub workspace_id: String,
    pub environment: String,
    pub secrets: Vec<SecretToUpdate>,
} 

#[deprecated]
#[derive(Deserialize, Debug)]
pub struct CreateSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecretRequest {
    #[serde(skip)]
    pub base_url: String,
    pub workspace_id: String,
    pub environment: String,
    #[serde(flatten)]
    pub secret: SecretToUpdate,
}

#[derive(Deserialize, Debug)]
pub struct CreateSecretResponse {
    pub secret: EncryptedSecret
}

#[derive(Serialize, Debug)]
pub struct SecretToUpdate {
    #[serde(skip)]
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: SecretType,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(flatten)]
    pub comment: EncryptedComment,
}

#[derive(Debug)]
pub struct RawSecret {
    pub type_name: SecretType,
    pub name: String,
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
}

#[deprecated]
#[derive(Serialize, Debug)]
pub struct UpdateSecretsRequest {
    #[serde(skip_serializing)]
    pub base_url: String,
    pub secrets: Vec<SecretToUpdate>,
}

#[derive(Serialize, Debug)]
pub struct UpdateSecretRequest {
    #[serde(skip)]
    pub base_url: String,
    #[serde(flatten)]
    pub secret: SecretToUpdate,
}

// #[derive(Serialize)]
// pub struct SecretToUpdate {
//     pub id: String,
//     #[serde(flatten)]
//     pub key: EncryptedKey,
//     #[serde(flatten)]
//     pub value: EncryptedValue,
// }

//#[deprecated]
#[derive(Deserialize, Debug)]
pub struct UpdateSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSecretResponse {
    pub secret: EncryptedSecret,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretsRequest {
    #[serde(skip)]
    pub base_url: String,
    pub workspace_id: String,
    pub environment: String,
}

#[derive(Deserialize)]
pub struct GetSecretsResponse {
    pub secrets: Vec<EncryptedSecret>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretRequest {
    #[serde(skip)]
    pub base_url: String,
    #[serde(skip)]
    pub secret_name: String,
    pub workspace_id: String,
    pub environment: String,
    #[serde(rename = "type")]
    pub secret_type: SecretType
}

#[derive(Deserialize)]
pub struct GetSecretResponse {
    pub secret: EncryptedSecret
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecretRequest {
    #[serde(skip)]
    pub base_url: String,
    #[serde(skip)]
    pub secret_name: String,
    pub environment: String,
    #[serde(rename = "type")]
    pub type_name: SecretType,
    pub workspace_id: String
}

#[derive(Deserialize)]
pub struct DeleteSecretResponse {
    pub secret: EncryptedSecret
}

pub struct GetSecretVersionsRequest {
    pub base_url: String,
    pub secret_id: String,
    pub offset: String,
    pub limit: String,
}

#[derive(Deserialize)]
pub struct GetSecretVersionsResponse {
    #[serde(alias = "secretVersions")]
    pub secret_versions: Vec<SecretVersion>,
}

#[derive(Deserialize)]
pub struct SecretVersion {
    pub tags: Vec<String>,
    #[serde(alias = "_id")]
    pub id: String,
    pub secret: String,
    pub version: u8,
    pub workspace: String,
    #[serde(alias = "type")]
    pub type_name: SecretType,
    pub environment: String,
    #[serde(alias = "isDeleted")]
    pub is_deleted: bool,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(alias = "__v")]
    pub v: u8,
    #[serde(flatten)]
    pub audit: Audit,
}

#[derive(Serialize)]
pub struct RollbackSecretToVersionRequest {
    pub base_url: String,
    pub secret_id: String,
    pub version: u8,
}

#[derive(Deserialize)]
pub struct RollbackSecretToVersionResponse {
    pub secret: EncryptedSecret,
}

#[derive(Deserialize, Debug)]
//#[serde(rename_all = "camelCase")]
pub struct EncryptedSecret {
    #[serde(alias = "_id")]
    pub id: String,
    pub version: u8,
    pub workspace: String,
    #[serde(alias = "type")]
    pub type_name: SecretType,
    #[serde(flatten)]
    pub key: EncryptedKey,
    #[serde(flatten)]
    pub value: EncryptedValue,
    #[serde(flatten)]
    pub comment: EncryptedComment,
    pub tags: Vec<String>,
    pub path: String,
    #[serde(flatten)]
    pub audit: Audit,
    #[serde(alias = "__v")]
    pub v: u8
}

#[derive(Deserialize)]
pub struct SecretTag {
    tag: String,
    slug: String,
}

#[derive(Debug)]
pub struct DecryptedSecret {
    pub id: String,
    pub version: u8,
    pub workspace: String,
    pub type_name: SecretType,
    pub key: String,
    pub value: String,
//    pub comment: Option<String>,
    pub comment: String,
    pub audit: Audit,
}

impl EncryptedSecret {
    pub fn decrypt(secret: &EncryptedSecret, private_key: &str) -> Result<DecryptedSecret> {
        let mut comment = None::<String>;
        let key = decrypt(
            &secret.key.ciphertext,
            &secret.key.iv,
            &secret.key.tag,
            private_key,
        )?;
        let value = decrypt(
            &secret.value.ciphertext,
            &secret.value.iv,
            &secret.value.tag,
            private_key,
        )?;
        let comment = decrypt(
            &secret.comment.ciphertext,
            &secret.comment.iv,
            &secret.comment.tag,
            private_key,
        )?;
        /*
        if let Some(encrypted_comment) = &secret.comment {
            comment = Some(decrypt(
                &encrypted_comment.ciphertext,
                &encrypted_comment.iv,
                &encrypted_comment.tag,
                private_key,
            )?);
        }
*/
        Ok(DecryptedSecret {
            id: secret.id.clone(),
            version: secret.version,
            workspace: secret.workspace.clone(),
            type_name: secret.type_name.clone(),
            key,
            value,
            comment,
            audit: secret.audit.clone(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedKey {
    #[serde(rename = "secretKeyCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretKeyIV")]
    pub iv: String,
    #[serde(rename = "secretKeyTag")]
    pub tag: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedValue {
    #[serde(rename = "secretValueCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretValueIV")]
    pub iv: String,
    #[serde(rename = "secretValueTag")]
    pub tag: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedComment {
    #[serde(rename = "secretCommentCiphertext")]
    pub ciphertext: String,
    #[serde(rename = "secretCommentIV")]
    pub iv: String,
    #[serde(rename = "secretCommentTag")]
    pub tag: String,
}

impl From<Encryption> for EncryptedComment {
    fn from(encryption: Encryption) -> EncryptedComment {
        EncryptedComment {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
}

impl From<Encryption> for EncryptedValue {
    fn from(encryption: Encryption) -> EncryptedValue {
        EncryptedValue {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
}

impl From<Encryption> for EncryptedKey {
    fn from(encryption: Encryption) -> EncryptedKey {
        EncryptedKey {
            ciphertext: encryption.text,
            tag: encryption.tag,
            iv: encryption.nonce,
        }
    }
}
