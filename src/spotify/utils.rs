use std::error::Error;

use reqwest::{Client, IntoUrl, header::AUTHORIZATION};
use serde::de::DeserializeOwned;

use crate::spotify::auth::Auth;

pub async fn fetch_model<T, U>(href: U, auth: &Auth) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
    U: IntoUrl,
{
    let client = Client::new();

    let bearer = format!("Bearer {}", auth.access_token);
    let res = client
        .get(href)
        .header(AUTHORIZATION, bearer)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(format!("Request failed: {}", err_text).into());
    }

    let data: T = res.json().await?;
    Ok(data)
}
