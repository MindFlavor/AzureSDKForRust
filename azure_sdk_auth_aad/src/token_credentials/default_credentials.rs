use crate::token_credentials::AzureCliCredential;
use crate::{token_credentials::TokenCredential, EnvironmentCredential, TokenResponse};
use azure_sdk_core::errors::AzureError;
use log::debug;

pub struct DefaultCredentialBuilder {
    include_environment_credential: bool,
    include_cli_credential: bool,
}

impl DefaultCredentialBuilder {
    pub fn new() -> Self {
        DefaultCredentialBuilder {
            include_cli_credential: true,
            include_environment_credential: true,
        }
    }

    pub fn exclude_environment_credential(&mut self) -> &mut Self {
        self.include_environment_credential = false;
        self
    }
    pub fn exclude_cli_credential(&mut self) -> &mut Self {
        self.include_cli_credential = false;
        self
    }
    pub fn build(&self) -> DefaultCredential {
        let source_count: usize =
            self.include_cli_credential as usize + self.include_cli_credential as usize;
        let mut sources =
            Vec::<Box<dyn TokenCredential + Send + Sync>>::with_capacity(source_count);
        if self.include_environment_credential {
            sources.push(Box::new(EnvironmentCredential {}));
        }
        if self.include_cli_credential {
            sources.push(Box::new(AzureCliCredential {}));
        }
        DefaultCredential::with_sources(sources)
    }
}

pub struct DefaultCredential {
    sources: Vec<Box<dyn TokenCredential + Send + Sync>>,
}

impl DefaultCredential {
    pub fn with_sources(sources: Vec<Box<dyn TokenCredential + Send + Sync>>) -> Self {
        DefaultCredential { sources }
    }
}

impl Default for DefaultCredential {
    fn default() -> Self {
        DefaultCredential {
            sources: vec![
                Box::new(EnvironmentCredential {}),
                Box::new(AzureCliCredential {}),
            ],
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for DefaultCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        for source in &self.sources {
            let token_res = source.get_token(resource).await;

            if let Ok(token) = token_res {
                return Ok(token);
            } else {
                debug!("Failed to get credentials: {:?}", token_res.err().unwrap());
            }
        }

        Err(AzureError::GenericErrorWithText(
            "End of default list".to_owned(),
        ))
    }
}
