#![warn(missing_docs)]

//! # infisical_api
//!
//! **Note: The author of this crate is very new to Rust and has likely made many mistakes. Please
//! feel free to open an issue or pull request for anything that can be improved.**
//!
//! The `infisical-api` crate provides a [Client] wrapper around the [Infisical API](https://infisical.com/).
//! The client provides all the functionality of the Infisical API, including:
//!
//! - Updating Secrets
//! - Accessing Secrets
//! - Secret Rollback
//! - Project Management
//!
//! infisical_api is built on top of reqwest and utilizes the async feature. An async runtime is
//! required in order to function. A feature allowing the use of blocking calls may be provided in
//! the future.
//!
//! The crate also includes utility functions for easy encrypting and decrypting of secrets
//!
//! Simple secret retrieval can be done by creating a client and providing the workspace id of your
//! infisical project as well as the environment (dev, test, prod, etc.).
//!
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let client = infisical_api::Client::new("Your API key here")?;
//! let secrets = client.get_encrypted_project_secrets("Your Infisical workspace ID", "Environment here");
//!
//! # Ok(())
//! # }
//! ```
//!
//! The [`Client`][client] defaults to the Infisical Cloud api endpoint, however a [`ClientBuilder`] is provided for more flexibility. It allows a custom API base url to be set and a custom Reqwest ClientBuilder.
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let reqwest_client_builder = reqwest::Client::builder();
//! // ...
//! // Configure reqwest_client_builder as needed
//! // ...
//! let client = infisical_api::ClientBuilder::new()
//!     .api_base("Your custom API endpoint")
//!     .reqwest_client_builder(reqwest_client_builder)
//!     .build("Your API key");
//!
//! # Ok(())
//! # }
//! ```
//!
//! The crate also provides the option to decrypt secrets after retrieval if desired.
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let client = infisical_api::Client::new("Your API key here")?;
//! let secrets = client
//!     .get_decrypted_project_secrets("Your Infisical workspace ID", "Environment here", "Your project key").await?;
//!
//! # Ok(())
//! # }
//! ```
//! It's recommended that you determine your project key ahead of time as it is required for
//! encryption and decryption functionality.
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let client = infisical_api::Client::new("Your API key here")?;
//! let private_key = client
//!     .get_user_decrypted_private_key("Your infisical password here")
//!     .await?;
//! let project_key = client
//!     .get_decrypted_project_key("Infisical workspace ID", &private_key)
//!     .await?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod utils;

#[doc(inline)]
pub use self::client::{Client, ClientBuilder};
#[doc(inline)]
pub use self::error::Error;
pub use reqwest;

#[cfg(test)]
mod tests {
    #[test]
    fn temp() {}
}
