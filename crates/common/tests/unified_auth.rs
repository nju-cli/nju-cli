use anyhow::{Context, Result};
use common::unified_auth;

#[tokio::test]
async fn logs_in_with_secret_credentials() -> Result<()> {
    dotenv::from_path("tests/auth.secret").context("failed to load tests/auth.secret")?;
    let username = dotenv::var("NJU_USERNAME").context("NJU_USERNAME is missing")?;
    let password = dotenv::var("NJU_PASSWORD").context("NJU_PASSWORD is missing")?;

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("failed to build reqwest client")?;
    let castgc = unified_auth::login(&client, username, password).await?;

    assert!(!castgc.trim().is_empty());

    Ok(())
}
