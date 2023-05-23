#![warn(missing_docs)]

//! # infisical_api
//!
//! The `infisical-api` crate provides a [Client] wrapper around the [Infisical API](https://infisical.com/).
//! The client provides all the functionality of the Infisical API, including:
//!
//! - Updating Secrets
//! - Accessing Secrets
//! - Secret Rollback
//! - Project Management
//!
//! There are two client options provided currently:
//! - [`ApiTokenClient`] - No longer recommended by the Infisical team as the token that drives
//! this client has full
//! access to the same functionality that your user account does.
//! - [`ServiceTokenClient`] - Preferred over the [`ApiTokenClient`]. Permissions are scoped to a
//! specific project and environment within that project. Can also be set to read only.
//! 
//! The below examples will demonstrate the flow when using the [`ServiceTokenClient`] at it is
//! preferred, however examples for each token type may be found in their respective documentation.
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
//! let client = infisical_api::ServiceTokenClient::new("Your Service Token here").await?;
//! let secrets = client.get_encrypted_secrets();
//!
//! # Ok(())
//! # }
//! ```
//!
//! The [`ServiceTokenClient`][client] defaults to the Infisical Cloud api endpoint, however a [`ServiceTokenClientBuilder`] is provided for more flexibility. It allows a custom API base url to be set and a custom [`Reqwest::ClientBuilder`].
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let reqwest_client_builder = reqwest::Client::builder();
//! // ...
//! // Configure reqwest_client_builder as needed
//! // ...
//! let client = infisical_api::ServiceTokenClientBuilder::new()
//!     .api_base("Your custom API endpoint")
//!     .reqwest_client_builder(reqwest_client_builder)
//!     .build("Your Service Token");
//!
//! # Ok(())
//! # }
//! ```
//!
//! The crate also provides the option to decrypt secrets after retrieval if desired.
//! ```rust
//! # use infisical_api::Error;
//! # async fn run() -> Result<(), Error> {
//! let client = infisical_api::ServiceTokenClient::new("Your Service Token here").await?;
//! let secrets = client
//!     .get_decrypted_secrets().await?;
//!
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod api_token_client;
pub mod enums;
pub mod error;
pub mod service_token_client;
mod traits;
pub mod utils;

#[doc(inline)]
pub use self::api_token_client::{ApiTokenClient, ApiTokenClientBuilder};
#[doc(inline)]
pub use self::error::Error;
#[doc(inline)]
pub use self::service_token_client::{ServiceTokenClient, ServiceTokenClientBuilder};
#[doc(inline)]
pub use reqwest;
