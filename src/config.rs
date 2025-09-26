use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct SpotifyCreds {
    pub client_id: String,
    pub redirect_uri: String,
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut v = Vec::new();

    if let Some(base) = dirs::config_dir() {
        v.push(base.join("playlist-sync/config.toml"));
    }

    let project_root = env!("CARGO_MANIFEST_DIR");
    v.push(Path::new(project_root).join("config/spotify.toml"));

    v
}

pub fn load_spotify_creds() -> Result<SpotifyCreds> {
    let id_env = env::var("SPOTIFY_CLIENT_ID").ok();
    let uri_env = env::var("SPOTIFY_REDIRECT_URI").ok();

    if let (Some(client_id), Some(redirect_uri)) = (id_env, uri_env) {
        return Ok(SpotifyCreds {
            client_id,
            redirect_uri,
        });
    }

    for path in candidate_paths() {
        if path.exists() {
            let s = fs::read_to_string(&path)
                .with_context(|| format!("reading config file {}", path.display()))?;
            let cfg: SpotifyCreds = toml::from_str(&s)
                .with_context(|| format!("parsing TOML in {}", path.display()))?;
            return Ok(cfg);
        }
    }

    anyhow::bail!(
        "Missing Spotify credentials. Set SPOTIFY_CLIENT_ID and SPOTIFY_REDIRECT_URI \
         or create a config file at ~/.config/playlist-sync/config.toml \
         or ./config/spotify.toml"
    );
}

pub fn cache_file_path() -> PathBuf {
    let base = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    base.join("playlist-sync/token.toml")
}
