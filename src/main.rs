use std::env;

use spotify_rs::{Spotify, artist::IncludeGroup, search::SearchInput};

async fn playlist(client: &Spotify, id: &str) {
    let res = client.playlist_tracks(id, 20).await.unwrap();
    dbg!(res);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let spotify = Spotify::new(&client_id, &client_secret).await.unwrap();
    playlist(&spotify, "2cUf7XcKs82z2oVyKzPapN").await;

    /*
    let artist_id = "3Cons1O5zLCcnXuq7SJdY7"; // merck
    // let artist_id = "1uNFoZAHBGtllmzznpCI3s"; // justin bieber
    let res = spotify
        .artist_tracks(
            artist_id,
            &[
                IncludeGroup::Album,
                IncludeGroup::Single,
                IncludeGroup::AppearsOn,
            ],
        )
        .await
        .unwrap();
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
    */
}
