use std::env;

use spotify_rs::Spotify;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let spotify = Spotify::new(&client_id, &client_secret).await.unwrap();
    let res = spotify.search("drunk tank", "marc e bassy").await.unwrap();
    dbg!(res);
}
