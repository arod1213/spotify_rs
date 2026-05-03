use std::env;

use spotify_rs::{Spotify, spotify::search::SearchInput};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let spotify = Spotify::new(&client_id, &client_secret).await.unwrap();
    let res = spotify
        .search(vec![
            SearchInput::Track("Drunk Tank".into()),
            SearchInput::Artist("Marc E Bassy".into()),
        ])
        .await
        .unwrap();
    dbg!(res);
}
