use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::track::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provider {
    Spotify,
    AppleMusic,
    Deezer,
    Tidal,
    YouTube,
    Unknown,
}

#[async_trait]
pub trait PlaylistProvider {
    fn provider(&self) -> Provider;
    async fn get_tracks_from_playlist(&self, playlist_url: &str) -> anyhow::Result<Vec<Track>>;
}

pub struct ParsedPlaylist {
    pub provider: Provider,
    pub id: String,
}

impl ParsedPlaylist {
    pub fn parse(input: &str) -> Self {
        let re_spotify_id = Regex::new(r"^[A-Za-z0-9]{22}$").unwrap();
        let re_apple_id = Regex::new(r"^pl\.[A-Za-z0-9\-_.]+$").unwrap();
        let re_deezer_id = Regex::new(r"^[0-9]{3,}$").unwrap();
        let re_tidal_id =
            Regex::new(r"^[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}$").unwrap();

        // Spotify URI's
        if input.starts_with("spotify:") {
            let parts: Vec<_> = input.split(':').collect();
            if parts.len() == 3 && parts[1] == "playlist" {
                return ParsedPlaylist {
                    provider: Provider::Spotify,
                    id: parts[2].to_string(),
                };
            }
        }

        if let Ok(url) = Url::parse(input) {
            let host = url.host_str().unwrap_or_default();

            // Spotify URL/URI-like
            if host.ends_with("open.spotify.com") {
                // path: /playlist/{id}
                let segments = url
                    .path_segments()
                    .map(|s| s.collect::<Vec<_>>())
                    .unwrap_or_default();
                if segments.get(0) == Some(&"playlist") {
                    if let Some(id) = segments.get(1) {
                        return ParsedPlaylist {
                            provider: Provider::Spotify,
                            id: (*id).to_string(),
                        };
                    }
                }
            }

            // Apple Music
            if host.ends_with("music.apple.com") {
                // path: /{country}/playlist/{slug}/pl.{id}
                let segments: Vec<_> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
                if segments.iter().any(|s| *s == "playlist") {
                    if let Some(last) = segments.last() {
                        if re_apple_id.is_match(last) {
                            return ParsedPlaylist {
                                provider: Provider::AppleMusic,
                                id: last.to_string(),
                            };
                        }
                    }
                }
            }

            // Deezer
            if host.ends_with("deezer.com") {
                // /playlist/{digits}
                let segs: Vec<_> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
                if segs.get(0) == Some(&"playlist") {
                    if let Some(id) = segs.get(1) {
                        if re_deezer_id.is_match(id) {
                            return ParsedPlaylist {
                                provider: Provider::Deezer,
                                id: id.to_string(),
                            };
                        }
                    }
                }
            }

            // TIDAL
            if host.ends_with("tidal.com") {
                // /browse/playlist/{uuid}
                let segs: Vec<_> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
                if segs.get(0) == Some(&"browse") && segs.get(1) == Some(&"playlist") {
                    if let Some(id) = segs.get(2) {
                        if re_tidal_id.is_match(id) {
                            return ParsedPlaylist {
                                provider: Provider::Tidal,
                                id: id.to_string(),
                            };
                        }
                    }
                }
            }

            // YouTube / YouTube Music (check 'list' query param)
            if host.ends_with("youtube.com")
                || host.ends_with("music.youtube.com")
                || host.ends_with("youtu.be")
            {
                if let Some(list_id) = url
                    .query_pairs()
                    .find_map(|(k, v)| (k == "list").then_some(v.to_string()))
                {
                    return ParsedPlaylist {
                        provider: Provider::YouTube,
                        id: list_id,
                    };
                }
            }
        }

        // raw IDs by pattern
        if re_spotify_id.is_match(input) {
            return ParsedPlaylist {
                provider: Provider::Spotify,
                id: input.to_string(),
            };
        }
        if re_apple_id.is_match(input) {
            return ParsedPlaylist {
                provider: Provider::AppleMusic,
                id: input.to_string(),
            };
        }
        if re_deezer_id.is_match(input) {
            return ParsedPlaylist {
                provider: Provider::Deezer,
                id: input.to_string(),
            };
        }
        if re_tidal_id.is_match(input) {
            return ParsedPlaylist {
                provider: Provider::Tidal,
                id: input.to_string(),
            };
        }

        ParsedPlaylist {
            provider: Provider::Unknown,
            id: input.to_string(),
        }
    }
}
