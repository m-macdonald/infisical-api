#![warn(missing_docs)]

//! # infisical-rs
//!
//! **Note: The author of this crate is very new to Rust and has likely made many mistakes. Please
//! feel free to open an issue or pull request for anything that can be improved.**
//!
//! The `infisical-rs` crate provides a [Client] wrapper around the [Infisical API](https://infisical.com/).
//! The client provides all the functionality of the Infisical API, including:
//!
//! - Updating Secrets
//! - Accessing Secrets
//! - Secret Rollback
//! - Project Management
//!
//! infisical_rs is built on top of reqwest and utilizes the async feature. An async runtime is
//! be required in order to function
//!
//! The crate also includes utility functions for easy encrypting and decrypting of secrets
//!
//! Simple secret retrieval can be done by creating a client and providing the workspace id of your
//! infisical project as well as the environment (dev, test, prod, etc.).
//!
//! ```rust
//! let client = infisical_rs::Client::new("Your API key here");
//! let secrets = client.get_encrypted_secrets("Your Infisical workspace ID", "Environment here");
//! ```
//!
//! The [`Client`][client] defaults to the Infisical Cloud api endpoint, however a [`ClientBuilder`][clientbuilder] is provided if self-hosting, or if a custom reqwest client is to be provided
//! ```rust
//! let mut client_builder = infisical_rs::ClientBuilder::new();
//! client_builder.api_base("Your custom API endpoint");
//! let reqwest_client_builder = reqwest::ClientBuilder::new();
//! // ...
//! // Configure reqwest_client_builder as needed
//! // ...
//! client_builder.reqwest_client_builder(reqwest_client_builder);
//! let client = client_builder.build("Your API key");
//! ```
//!
//! The crate also provides the option to decrypt secrets after retrieval if desired.
//! ```rust
//! let client = infisical_rs::Client::new("Your API key here");
//! let secrets = client
//!     .get_decrypted_secrets("Your Infisical workspace ID", "Environment here", "Your project key");
//! ```
//! It's recommended that you determine your project key ahead of time as it is required for
//! encryption and decryption functionality. There is a CLI built on top of this crate that can be
//! used to determine your project key, however the same can be accomplished by doing the following
//!
//! ```rust
//! let client = infisical_rs::Client::new("Your API key here");
//! let private_key = client
//!     .get_private_key("Your infisical password here")
//!     .await?;
//! let project_key = client
//!     .get_decrypted_project_key("Infisical workspace ID", &private_key)
//!     .await?;
//! ```

pub mod client;

pub mod api;
pub mod error;
pub mod utils;

pub use self::client::{Client, ClientBuilder};
pub use self::error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn temp() {}
}
