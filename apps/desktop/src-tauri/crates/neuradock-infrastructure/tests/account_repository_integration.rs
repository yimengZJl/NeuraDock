use std::collections::HashMap;
use std::sync::Arc;

use neuradock_domain::account::{Account, AccountRepository, Credentials};
use neuradock_domain::shared::ProviderId;
use neuradock_infrastructure::persistence::repositories::SqliteAccountRepository;

mod test_helpers;

#[tokio::test]
async fn account_repo_save_and_find_integration() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;

    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Build domain account
    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "abc123".to_string());
    let credentials = Credentials::new(cookies, "api_user_1".to_string());

    let account = Account::new(
        "Integration Account".to_string(),
        ProviderId::from_string("test-provider"),
        credentials,
    )
    .expect("Create account aggregate");

    // Save
    repo.save(&account).await.expect("Save account");

    // Find
    let found = repo
        .find_by_id(account.id())
        .await
        .expect("Find account")
        .expect("Account should be found");

    assert_eq!(found.name(), account.name());
    assert_eq!(found.provider_id().as_str(), account.provider_id().as_str());
}

#[tokio::test]
async fn account_repo_update_account() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create and save account
    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "original123".to_string());
    let credentials = Credentials::new(cookies, "api_user_1".to_string());

    let mut account = Account::new(
        "Original Name".to_string(),
        ProviderId::from_string("test-provider"),
        credentials,
    )
    .expect("Create account");

    repo.save(&account).await.expect("Save account");

    // Update account
    account.update_name("Updated Name".to_string()).expect("Update name");
    repo.save(&account).await.expect("Update account");

    // Verify update
    let found = repo
        .find_by_id(account.id())
        .await
        .expect("Find account")
        .expect("Account should exist");

    assert_eq!(found.name(), "Updated Name");
}

#[tokio::test]
async fn account_repo_delete_account() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create and save account
    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "test123".to_string());
    let credentials = Credentials::new(cookies, "api_user".to_string());

    let account = Account::new(
        "To Be Deleted".to_string(),
        ProviderId::from_string("test-provider"),
        credentials,
    )
    .expect("Create account");

    repo.save(&account).await.expect("Save account");

    // Verify exists
    let found = repo
        .find_by_id(account.id())
        .await
        .expect("Find account");
    assert!(found.is_some());

    // Delete
    repo.delete(account.id()).await.expect("Delete account");

    // Verify deleted
    let found = repo
        .find_by_id(account.id())
        .await
        .expect("Find account");
    assert!(found.is_none());
}

#[tokio::test]
async fn account_repo_find_by_ids_batch() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create multiple accounts
    let mut account_ids = Vec::new();
    for i in 0..5 {
        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), format!("session_{}", i));
        let credentials = Credentials::new(cookies, format!("api_user_{}", i));

        let account = Account::new(
            format!("Account {}", i),
            ProviderId::from_string("test-provider"),
            credentials,
        )
        .expect("Create account");

        account_ids.push(account.id().clone());
        repo.save(&account).await.expect("Save account");
    }

    // Batch find
    let found = repo
        .find_by_ids(&account_ids)
        .await
        .expect("Find by IDs");

    assert_eq!(found.len(), 5);
}

#[tokio::test]
async fn account_repo_find_all() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create multiple accounts
    for i in 0..3 {
        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), format!("session_{}", i));
        let credentials = Credentials::new(cookies, format!("api_user_{}", i));

        let account = Account::new(
            format!("Account {}", i),
            ProviderId::from_string("test-provider"),
            credentials,
        )
        .expect("Create account");

        repo.save(&account).await.expect("Save account");
    }

    // Find all
    let accounts = repo.find_all().await.expect("Find all");
    assert_eq!(accounts.len(), 3);
}

#[tokio::test]
async fn account_repo_find_enabled_only() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create enabled and disabled accounts
    for i in 0..4 {
        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), format!("session_{}", i));
        let credentials = Credentials::new(cookies, format!("api_user_{}", i));

        let mut account = Account::new(
            format!("Account {}", i),
            ProviderId::from_string("test-provider"),
            credentials,
        )
        .expect("Create account");

        // Disable half of them
        if i % 2 == 0 {
            account.toggle(false);
        }

        repo.save(&account).await.expect("Save account");
    }

    // Find enabled only
    let enabled = repo.find_enabled().await.expect("Find enabled");
    assert_eq!(enabled.len(), 2); // Only half are enabled
}

#[tokio::test]
async fn account_repo_credentials_encryption() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Create account with sensitive cookies
    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "super_secret_session_token".to_string());
    cookies.insert("auth".to_string(), "super_secret_auth_token".to_string());
    let credentials = Credentials::new(cookies.clone(), "sensitive_api_user".to_string());

    let account = Account::new(
        "Encrypted Account".to_string(),
        ProviderId::from_string("test-provider"),
        credentials,
    )
    .expect("Create account");

    repo.save(&account).await.expect("Save account");

    // Verify credentials are encrypted in database by querying raw data
    let raw_cookies: String = sqlx::query_scalar(
        "SELECT cookies FROM accounts WHERE id = ?",
    )
    .bind(account.id().as_str())
    .fetch_one(&pool)
    .await
    .expect("Fetch raw cookies");

    // Encrypted data should not contain plaintext
    assert!(!raw_cookies.contains("super_secret_session_token"));

    // But decrypted account should have correct credentials
    let found = repo
        .find_by_id(account.id())
        .await
        .expect("Find account")
        .expect("Account exists");

    assert_eq!(
        found.credentials().cookies().get("session").unwrap(),
        "super_secret_session_token"
    );
    assert_eq!(found.credentials().api_user(), "sensitive_api_user");
}

#[tokio::test]
async fn account_repo_concurrent_saves() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = Arc::new(SqliteAccountRepository::new(Arc::new(pool.clone()), encryption));

    // Create multiple accounts concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let repo_clone = Arc::clone(&repo);
        let handle = tokio::spawn(async move {
            let mut cookies = HashMap::new();
            cookies.insert("session".to_string(), format!("session_{}", i));
            let credentials = Credentials::new(cookies, format!("api_user_{}", i));

            let account = Account::new(
                format!("Concurrent Account {}", i),
                ProviderId::from_string("test-provider"),
                credentials,
            )
            .expect("Create account");

            repo_clone.save(&account).await.expect("Save account");
        });
        handles.push(handle);
    }

    // Wait for all saves to complete
    for handle in handles {
        handle.await.expect("Task completed");
    }

    // Verify all accounts were saved
    let accounts = repo.find_all().await.expect("Find all");
    assert_eq!(accounts.len(), 10);
}

#[tokio::test]
async fn account_repo_empty_batch_find() {
    let (pool, encryption) = test_helpers::setup_in_memory_db().await;
    let repo = SqliteAccountRepository::new(Arc::new(pool.clone()), encryption);

    // Try to find with empty IDs list
    let found = repo.find_by_ids(&[]).await.expect("Find by empty IDs");
    assert_eq!(found.len(), 0);
}
