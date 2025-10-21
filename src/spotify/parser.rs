use rspotify::model::{PlayableItem, PlaylistItem};

use crate::track::{Track, release_year};

impl Track {
    pub fn from_item(it: PlaylistItem) -> Option<Track> {
        match it.track {
            Some(PlayableItem::Track(t)) => {
                let album = t.album;
                let artist_name = t
                    .artists
                    .first()
                    .map(|a| a.name.clone())
                    .unwrap_or_default();

                let dur_ms = t.duration.num_milliseconds();

                let year = release_year(album.release_date);

                Some(Track::new(
                    t.name,
                    album.name,
                    artist_name,
                    Some(t.track_number as u32),
                    year,
                    Some(dur_ms as u128),
                ))
            }
            _ => None,
        }
    }
}
