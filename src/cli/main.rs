use std::{env, time::Instant};

use anyhow::Result;
use playlist_sync::{
    get_tracks_from_playlist, local_music_library_parser::get_local_music_library_tracks,
};

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let id = env::args()
        .nth(1)
        .expect("expected playlist id as first argument");

    let playlist_items = get_tracks_from_playlist(&id).await;

    println!("{:?}", playlist_items);
    println!("Operation took: {:?}", start.elapsed());

    let start = Instant::now();
    let id = env::args().nth(2).expect("expected local library path");
    let local_library = get_local_music_library_tracks(&id)?;
    println!("{:?}", local_library);
    println!("Operation took: {:?}", start.elapsed());

    Ok(())
}
