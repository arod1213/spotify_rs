use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub access_token: String,
    pub token_type: String,
    pub error: Option<String>,
}

pub async fn get_token(client_id: &str, client_secret: &str) -> Result<Auth, Box<dyn Error>> {
    let client = Client::new();

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(format!("Request failed: {}", err_text).into());
    }

    let auth: Auth = res.json().await?;
    Ok(auth)
}
