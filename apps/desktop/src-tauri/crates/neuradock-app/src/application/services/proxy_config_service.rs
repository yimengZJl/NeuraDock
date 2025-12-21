use std::sync::Arc;

use neuradock_domain::proxy_config::ProxyConfigRepository;
use neuradock_domain::shared::DomainError;

use crate::application::dtos::{ProxyConfigDto, UpdateProxyConfigInput};

pub struct ProxyConfigService {
    repo: Arc<dyn ProxyConfigRepository>,
}

impl ProxyConfigService {
    pub fn new(repo: Arc<dyn ProxyConfigRepository>) -> Self {
        Self { repo }
    }

    pub async fn get(&self) -> Result<ProxyConfigDto, DomainError> {
        let config = self.repo.get().await?;
        Ok(ProxyConfigDto::from(&config))
    }

    pub async fn update(
        &self,
        input: UpdateProxyConfigInput,
    ) -> Result<ProxyConfigDto, DomainError> {
        let mut config = self.repo.get().await?;

        let proxy_type = input.to_proxy_type().map_err(DomainError::Validation)?;

        config.update(input.enabled, proxy_type, input.host, input.port)?;
        self.repo.save(&config).await?;

        Ok(ProxyConfigDto::from(&config))
    }
}
