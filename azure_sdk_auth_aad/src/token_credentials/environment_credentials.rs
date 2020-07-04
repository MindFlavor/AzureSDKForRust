use crate::token_credentials::TokenCredential;

use azure_sdk_core::errors::AzureError;
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken,
    AsyncClientCredentialsTokenRequest, AuthType, AuthUrl, Scope, TokenResponse, TokenUrl,
};
use std::{str, time::Duration};
use url::Url;

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET_ENV_KEY: &str = "AZURE_CLIENT_SECRET";

pub struct EnvironmentCredential;

#[async_trait::async_trait]
impl TokenCredential for EnvironmentCredential {
    async fn get_token(&self, resource: &str) -> Result<crate::TokenResponse, AzureError> {
        let tenant_id = std::env::var(AZURE_TENANT_ID_ENV_KEY).map_err(|_| {
            AzureError::GenericErrorWithText(format!(
                "Missing tenant id set in {} environment variable",
                AZURE_TENANT_ID_ENV_KEY
            ))
        })?;
        let client_id = std::env::var(AZURE_CLIENT_ID_ENV_KEY)
            .map(|client_id| oauth2::ClientId::new(client_id))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Missing client id set in {} environment variable",
                    AZURE_CLIENT_ID_ENV_KEY
                ))
            })?;
        let client_secret = std::env::var(AZURE_CLIENT_SECRET_ENV_KEY)
            .map(|client_secret| Some(oauth2::ClientSecret::new(client_secret)))
            .unwrap_or_default();

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct authorize endpoint with tenant id {}",
                    tenant_id,
                ))
            })?,
        );
        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct token endpoint with tenant id {}",
                    tenant_id,
                ))
            })?,
        );

        let client = BasicClient::new(client_id, client_secret, auth_url, Some(token_url))
            .set_auth_type(AuthType::RequestBody);

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new(format!("{}.default", resource)))
            .request_async(async_http_client)
            .await
            .map(|r| {
                crate::TokenResponse::new(
                    AccessToken::new(r.access_token().secret().to_owned()),
                    Utc::now()
                        + chrono::Duration::from_std(
                            r.expires_in().unwrap_or(Duration::from_secs(0)),
                        )
                        .unwrap(),
                )
            })
            .map_err(|e| match e {
                oauth2::RequestTokenError::ServerResponse(s) => AzureError::GenericErrorWithText(
                    s.error_description()
                        .unwrap_or(&"Server error without description".to_string())
                        .to_owned(),
                ),
                _ => AzureError::GenericErrorWithText("OAuth2 error".to_string()),
            })?;

        Ok(token_result)
    }
}
