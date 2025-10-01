use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::ItemKey,
};

use crate::track::{Track, release_year};

impl Track {
    pub fn from_file(path: &str) -> Option<Track> {
        match lofty::read_from_path(path) {
            Ok(tagged_file) => {
                let properties = tagged_file.properties();

                let tag = tagged_file
                    .primary_tag()
                    .or_else(|| tagged_file.first_tag());

                tag.and_then(|tag| {
                    let title = tag.get_string(&ItemKey::TrackTitle)?;
                    let album = tag.get_string(&ItemKey::AlbumTitle)?;
                    let artist = tag.get_string(&ItemKey::AlbumArtist)?;
                    let track_num = tag
                        .get_string(&ItemKey::TrackNumber)
                        .and_then(|t| t.parse::<u32>().ok());

                    let date: Option<String> = tag
                        .get_string(&ItemKey::RecordingDate)
                        .map(|s| s.to_string());
                    let year = release_year(date);

                    let duration = Some(properties.duration().as_millis());

                    Some(Track::new(
                        title.to_string(),
                        album.to_string(),
                        artist.split(";").next().unwrap().to_string(),
                        track_num,
                        year,
                        duration,
                    ))
                })
            }
            Err(_) => None,
        }
    }
}
