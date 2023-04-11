use serde::{Deserialize, Serialize};

/// An enum representing the different types possible for a secret
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SecretType {
    Personal,
    Shared,
    // Represents an unknown secret type. If a new secret type is added to Infisical this should
    // keep deserialization from failing
    Other(String),
}
