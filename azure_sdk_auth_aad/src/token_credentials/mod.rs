mod cli_credentials;
mod default_credentials;
mod environment_credentials;
mod managed_identity_credentials;
pub use crate::token_credentials::cli_credentials::*;
pub use crate::token_credentials::default_credentials::*;
pub use crate::token_credentials::environment_credentials::*;
pub use crate::token_credentials::managed_identity_credentials::*;
use azure_sdk_core::errors::AzureError;
use oauth2::AccessToken;

#[async_trait::async_trait]
pub trait TokenCredential {
    async fn get_token(&self, resource: &str) -> Result<Box<AccessToken>, AzureError>;
}
