use std::env;

use spotify_rs::{Spotify, search::SearchInput};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let spotify = Spotify::new(&client_id, &client_secret).await.unwrap();
    let artist_id = "3Cons1O5zLCcnXuq7SJdY7";
    let res = spotify.artist_tracks(artist_id).await.unwrap();
    let val = serde_json::to_value(&res).unwrap();
    println!("{}", serde_json::to_string(&val).unwrap());
    // let res = spotify
    //     .search(
    //         vec![
    //             SearchInput::Track("Drunk Tank".into()),
    //             SearchInput::Artist("Marc E Bassy".into()),
    //         ],
    //         10,
    //     )
    //     .await
    //     .unwrap();
    // dbg!(res);
}
