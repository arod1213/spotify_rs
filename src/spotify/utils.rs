use std::error::Error;

use chrono::NaiveDate;
use reqwest::{Client, IntoUrl, header::AUTHORIZATION};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

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

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let fmts = ["%Y-%m-%d", "%Y-%m", "%Y"];
    let date = fmts
        .iter()
        .find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok());

    match date {
        Some(s) => Ok(s),
        None => {
            let err_msg = format!("{} is not a valid date", s);
            eprintln!("{}", err_msg);
            Err(serde::de::Error::custom(err_msg))
        }
    }
}
