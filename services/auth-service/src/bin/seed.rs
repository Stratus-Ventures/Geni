use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let client = Client::new();

    println!("🌱 Seeding data to {}", base_url);

    // 1. Create User via Magic Link
    println!("  ➡️  Creating user 'test@example.com'...");
    let res = client.post(format!("{}/auth/magic-link/request", base_url))
        .json(&json!({ "email": "test@example.com" }))
        .send()
        .await?;

    if !res.status().is_success() {
        println!("  ❌ Failed to request link: {:?}", res.text().await?);
        return Ok(());
    }

    let token_str: String = res.json().await?;
    println!("     Token: {}", token_str);

    // 2. Verify Link (Logs in / Creates User)
    println!("  ➡️  Verifying link...");
    let res = client.post(format!("{}/auth/magic-link/verify", base_url))
        .json(&json!({ 
            "email": "test@example.com",
            "token": token_str
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        println!("  ❌ Failed to verify link: {:?}", res.text().await?);
        return Ok(());
    }

    let session: serde_json::Value = res.json().await?;
    println!("  ✅  User created! Session ID: {}", session["id"]);

    // 3. (Optional) Create another user
    println!("  ➡️  Creating user 'admin@example.com'...");
    let res = client.post(format!("{}/auth/magic-link/request", base_url))
        .json(&json!({ "email": "admin@example.com" }))
        .send()
        .await?;
    let token_str: String = res.json().await?;
    let res = client.post(format!("{}/auth/magic-link/verify", base_url))
        .json(&json!({ 
            "email": "admin@example.com",
            "token": token_str
        }))
        .send()
        .await?;
    let session: serde_json::Value = res.json().await?;
    println!("  ✅  User created! Session ID: {}", session["id"]);

    println!("✨ Seeding complete!");
    Ok(())
}
