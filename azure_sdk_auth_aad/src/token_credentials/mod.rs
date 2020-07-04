mod cli_credentials;
mod default_credentials;
mod environment_credentials;
mod managed_identity_credentials;
pub use crate::token_credentials::cli_credentials::*;
pub use crate::token_credentials::default_credentials::*;
pub use crate::token_credentials::environment_credentials::*;
pub use crate::token_credentials::managed_identity_credentials::*;
use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;

#[derive(Debug, Clone)]
pub struct TokenResponse {
    pub token: AccessToken,
    pub expires_on: DateTime<Utc>,
}

impl TokenResponse {
    pub(crate) fn new(token: AccessToken, expires_on: DateTime<Utc>) -> Self {
        TokenResponse { token, expires_on }
    }
}

#[async_trait::async_trait]
pub trait TokenCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError>;
}
