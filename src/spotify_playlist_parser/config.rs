use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use rspotify::Token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<i64>,
    scopes: Vec<String>,
}

pub fn save_token(path: &Path, t: &Token) -> Result<()> {
    let st = SerializableToken {
        access_token: t.access_token.clone(),
        refresh_token: t.refresh_token.clone(),
        expires_at: t.expires_at.map(|dt| dt.timestamp()),
        scopes: t.scopes.iter().cloned().collect(),
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let toml_str = toml::to_string(&st)?;
    fs::write(path, toml_str)?;
    Ok(())
}

pub fn load_token(path: &PathBuf) -> Result<Token> {
    let bytes = fs::read(path)?;
    let st: SerializableToken = toml::from_slice(&bytes)?;

    let expires_at = st
        .expires_at
        .and_then(|ts| chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0));

    let expires_in = expires_at.map(|at| {
        let now = chrono::Utc::now();
        let secs = (at - now).num_seconds();
        chrono::Duration::seconds(secs.max(0))
    });

    Ok(Token {
        access_token: st.access_token,
        refresh_token: st.refresh_token,
        expires_at,
        expires_in: expires_in.unwrap(),
        scopes: st.scopes.into_iter().collect(),
    })
}
