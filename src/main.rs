mod provider;
mod resolver;
mod spotify;
mod track;

use std::{env, time::Instant};

use anyhow::Result;

use crate::resolver::PlaylistProviderResolver;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let url = env::args()
        .nth(1)
        .expect("expected playlist id as first argument");

    let resolver = PlaylistProviderResolver::with_defaults().await?;
    let playlist_items = resolver.get_tracks_from_playlist(&url).await?;

    println!("{:?}", playlist_items);
    println!("Operation took: {:?}", start.elapsed());

    Ok(())
}
