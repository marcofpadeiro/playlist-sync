use anyhow::{Context, Result};
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, clients::OAuthClient, scopes};
use std::io::{self, Write};
use url::Url;

#[derive(Debug, Clone)]
pub struct SpotifyAuth {
    client_id: String,
    redirect_uri: String,
}

impl SpotifyAuth {
    pub fn new(client_id: impl Into<String>, redirect_uri: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
        }
    }

    pub fn build_client(&self) -> AuthCodePkceSpotify {
        let creds = Credentials::new(&self.client_id, "");
        let oauth = OAuth {
            redirect_uri: self.redirect_uri.clone(),
            scopes: scopes!("playlist-read-private", "playlist-read-collaborative"),
            ..Default::default()
        };
        AuthCodePkceSpotify::new(creds, oauth)
    }

    pub async fn authenticate(&self) -> Result<AuthCodePkceSpotify> {
        let mut client = self.build_client();

        let auth_url = client
            .get_authorize_url(None)
            .context("failed to build authorize url")?;
        eprintln!("\nOpen this URL in your browser:\n{auth_url}\n");
        eprint!("Paste the FULL redirect URL here: ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        io::stdin().read_line(&mut line).context("stdin failed")?;
        let url = Url::parse(line.trim()).context("invalid URL pasted")?;
        let code = url
            .query_pairs()
            .find_map(|(k, v)| (k == "code").then_some(v.into_owned()))
            .context("missing `code` param")?;

        client
            .request_token(&code)
            .await
            .context("token exchange failed")?;
        client.config.token_refreshing = true;

        Ok(client)
    }
}
