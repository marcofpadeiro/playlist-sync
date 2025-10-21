use crate::spotify::provider::SpotifyProvider;
use anyhow::Result;
use std::collections::HashMap;

use crate::{
    provider::{ParsedPlaylist, PlaylistProvider, Provider},
    track::Track,
};
pub struct PlaylistProviderResolver {
    map: HashMap<Provider, Box<dyn PlaylistProvider>>,
}

impl PlaylistProviderResolver {
    pub async fn with_defaults() -> Result<Self> {
        let mut map: HashMap<Provider, Box<dyn PlaylistProvider>> = HashMap::new();

        let spotify = SpotifyProvider::new_from_env().await?;
        let sp: Box<dyn PlaylistProvider> = Box::new(spotify);

        map.insert(Provider::Spotify, sp);

        Ok(Self { map })
    }

    pub async fn get_tracks_from_playlist(&self, input: &str) -> Result<Vec<Track>> {
        let parsed = ParsedPlaylist::parse(input);
        let provider = self
            .map
            .get(&parsed.provider)
            .ok_or_else(|| anyhow::anyhow!("Unsupported provider: {:?}", parsed.provider))?;
        provider.get_tracks_from_playlist(&parsed.id).await
    }
}
