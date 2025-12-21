use std::collections::HashMap;
use std::sync::Arc;

use neuradock_domain::account::AccountRepository;
use neuradock_domain::check_in::ProviderRepository;
use neuradock_domain::proxy_config::ProxyConfigRepository;
use neuradock_domain::shared::{AccountId, DomainError, ProviderId};
use neuradock_infrastructure::http::{token::TokenClient, WafBypassService};
use neuradock_infrastructure::persistence::repositories::{
    SqliteProviderModelsRepository, SqliteWafCookiesRepository,
};

pub struct ProviderModelsQueryService {
    account_repo: Arc<dyn AccountRepository>,
    provider_repo: Arc<dyn ProviderRepository>,
    provider_models_repo: Arc<SqliteProviderModelsRepository>,
    waf_cookies_repo: Arc<SqliteWafCookiesRepository>,
    proxy_config_repo: Arc<dyn ProxyConfigRepository>,
}

impl ProviderModelsQueryService {
    pub fn new(
        account_repo: Arc<dyn AccountRepository>,
        provider_repo: Arc<dyn ProviderRepository>,
        provider_models_repo: Arc<SqliteProviderModelsRepository>,
        waf_cookies_repo: Arc<SqliteWafCookiesRepository>,
        proxy_config_repo: Arc<dyn ProxyConfigRepository>,
    ) -> Self {
        Self {
            account_repo,
            provider_repo,
            provider_models_repo,
            waf_cookies_repo,
            proxy_config_repo,
        }
    }

    pub async fn get_cached(&self, provider_id: &str) -> Result<Vec<String>, DomainError> {
        let cached = self
            .provider_models_repo
            .find_by_provider(provider_id)
            .await?;
        Ok(cached.map(|c| c.models).unwrap_or_default())
    }

    pub async fn fetch(
        &self,
        provider_id: String,
        account_id: String,
        force_refresh: bool,
    ) -> Result<Vec<String>, DomainError> {
        // Cache first (unless force refresh)
        if !force_refresh {
            let is_stale = self.provider_models_repo.is_stale(&provider_id, 24).await?;
            if !is_stale {
                if let Some(cached) = self
                    .provider_models_repo
                    .find_by_provider(&provider_id)
                    .await?
                {
                    return Ok(cached.models);
                }
            }
        }

        let account_id_obj = AccountId::from_string(&account_id);
        let account = self
            .account_repo
            .find_by_id(&account_id_obj)
            .await?
            .ok_or_else(|| DomainError::AccountNotFound(account_id.clone()))?;

        let provider_id_obj = ProviderId::from_string(&provider_id);
        let provider = self
            .provider_repo
            .find_by_id(&provider_id_obj)
            .await?
            .ok_or_else(|| DomainError::ProviderNotFound(provider_id.clone()))?;

        let models_path = provider.models_path().ok_or_else(|| {
            DomainError::Validation("Provider does not support models API".to_string())
        })?;
        let base_url = provider.domain().trim_end_matches('/').to_string();
        let api_user_header = provider.api_user_key();
        let api_user_header_opt = if api_user_header.is_empty() {
            None
        } else {
            Some(api_user_header)
        };

        let mut cookies: HashMap<String, String> = account.credentials().cookies().clone();

        // Merge cached WAF cookies if provider requires it.
        if provider.needs_waf_bypass() {
            if let Ok(Some(cached_waf)) = self.waf_cookies_repo.get_valid(&provider_id).await {
                for (k, v) in cached_waf.cookies {
                    cookies.insert(k, v);
                }
            }
        }

        let cookie_string = build_cookie_string(&cookies);
        let api_user = account.credentials().api_user();

        let proxy_url = self
            .proxy_config_repo
            .get()
            .await
            .ok()
            .and_then(|c| c.proxy_url());
        let client = TokenClient::with_proxy(proxy_url)
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let models_result = client
            .fetch_provider_models(
                &base_url,
                models_path,
                &cookie_string,
                api_user_header_opt,
                Some(api_user),
            )
            .await;

        let models = match models_result {
            Ok(models) => models,
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("WAF_CHALLENGE:") {
                    let _ = self.waf_cookies_repo.delete(&provider_id).await;
                    return Err(DomainError::Infrastructure(
                        "WAF challenge detected. Please use 'Refresh with WAF' to refresh cookies."
                            .to_string(),
                    ));
                }
                return Err(DomainError::Infrastructure(msg));
            }
        };

        self.provider_models_repo
            .save(&provider_id, &models)
            .await?;
        Ok(models)
    }

    pub async fn refresh_with_waf(
        &self,
        provider_id: String,
        account_id: String,
    ) -> Result<Vec<String>, DomainError> {
        let account_id_obj = AccountId::from_string(&account_id);
        let account = self
            .account_repo
            .find_by_id(&account_id_obj)
            .await?
            .ok_or_else(|| DomainError::AccountNotFound(account_id.clone()))?;

        let provider_id_obj = ProviderId::from_string(&provider_id);
        let provider = self
            .provider_repo
            .find_by_id(&provider_id_obj)
            .await?
            .ok_or_else(|| DomainError::ProviderNotFound(provider_id.clone()))?;

        let models_path = provider.models_path().ok_or_else(|| {
            DomainError::Validation("Provider does not support models API".to_string())
        })?;
        let base_url = provider.domain().trim_end_matches('/').to_string();
        let api_user_header = provider.api_user_key();
        let api_user_header_opt = if api_user_header.is_empty() {
            None
        } else {
            Some(api_user_header)
        };

        let proxy_url = self
            .proxy_config_repo
            .get()
            .await
            .ok()
            .and_then(|c| c.proxy_url());
        let waf_service = WafBypassService::with_proxy(true, proxy_url.clone());
        let client = TokenClient::with_proxy(proxy_url)
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let mut cookies: HashMap<String, String> = account.credentials().cookies().clone();

        if provider.needs_waf_bypass() {
            match self.waf_cookies_repo.get_valid(&provider_id).await {
                Ok(Some(cached_waf)) => {
                    for (k, v) in cached_waf.cookies {
                        cookies.insert(k, v);
                    }
                }
                _ => {
                    let new_cookies = waf_service
                        .get_waf_cookies(&provider.login_url(), &account_id)
                        .await
                        .map_err(|e| {
                            DomainError::Infrastructure(format!("WAF bypass failed: {e}"))
                        })?;

                    let _ = self.waf_cookies_repo.save(&provider_id, &new_cookies).await;
                    for (k, v) in new_cookies {
                        cookies.insert(k, v);
                    }
                }
            }
        }

        let api_user = account.credentials().api_user();
        let models = fetch_models_with_waf_retry(
            &client,
            &waf_service,
            &provider,
            &base_url,
            models_path,
            api_user_header_opt,
            api_user,
            &account_id,
            &provider_id,
            &self.waf_cookies_repo,
            cookies,
        )
        .await?;

        self.provider_models_repo
            .save(&provider_id, &models)
            .await?;
        Ok(models)
    }
}

fn build_cookie_string(cookies: &HashMap<String, String>) -> String {
    cookies
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("; ")
}

#[allow(clippy::too_many_arguments)]
async fn fetch_models_with_waf_retry(
    client: &TokenClient,
    waf_service: &WafBypassService,
    provider: &neuradock_domain::check_in::Provider,
    base_url: &str,
    models_path: &str,
    api_user_header_opt: Option<&str>,
    api_user: &str,
    account_id: &str,
    provider_id: &str,
    waf_cookies_repo: &SqliteWafCookiesRepository,
    mut cookies: HashMap<String, String>,
) -> Result<Vec<String>, DomainError> {
    let cookie_string = build_cookie_string(&cookies);
    let models_result = client
        .fetch_provider_models(
            base_url,
            models_path,
            &cookie_string,
            api_user_header_opt,
            Some(api_user),
        )
        .await;

    match models_result {
        Ok(models) => Ok(models),
        Err(e) => {
            let error_msg = e.to_string();
            if !error_msg.contains("WAF_CHALLENGE:") {
                return Err(DomainError::Infrastructure(error_msg));
            }

            let _ = waf_cookies_repo.delete(provider_id).await;
            let fresh_waf = waf_service
                .get_waf_cookies(&provider.login_url(), account_id)
                .await
                .map_err(|e| DomainError::Infrastructure(format!("WAF bypass failed: {e}")))?;

            let _ = waf_cookies_repo.save(provider_id, &fresh_waf).await;
            for (k, v) in fresh_waf {
                cookies.insert(k, v);
            }

            let fresh_cookie_string = build_cookie_string(&cookies);
            client
                .fetch_provider_models(
                    base_url,
                    models_path,
                    &fresh_cookie_string,
                    api_user_header_opt,
                    Some(api_user),
                )
                .await
                .map_err(|e| {
                    DomainError::Infrastructure(format!(
                        "Failed to fetch models even after WAF bypass: {e}"
                    ))
                })
        }
    }
}
