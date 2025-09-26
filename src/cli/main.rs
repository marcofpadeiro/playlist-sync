use anyhow::Result;
use playlist_sync::get_tracks_from_spotify_playlist;
use std::{env, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let id = env::args()
        .nth(1)
        .expect("expected playlist id as first argument");

    let playlist_items = get_tracks_from_spotify_playlist(&id).await;

    println!("{:?}", playlist_items);
    println!("Operation took: {:?}", start.elapsed());

    Ok(())
}
