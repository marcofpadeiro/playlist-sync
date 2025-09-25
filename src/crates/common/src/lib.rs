use std::path::PathBuf;

#[derive(Debug, Eq)]
pub struct Track {
    title: String,
    album: String,
    artist: String,
    track_num: Option<u32>,
    year: Option<u32>,
    duration: Option<u32>, // ms
    path: Option<PathBuf>,
}

pub trait TrackListProvider {
    type Error;

    fn get_tracks(&self) -> Result<Vec<Track>, Self::Error>;
}

impl Track {
    pub fn new(
        title: String,
        album: String,
        artist: String,
        track_num: Option<u32>,
        year: Option<u32>,
        duration: Option<u32>,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            title,
            album,
            artist,
            track_num,
            year,
            duration,
            path,
        }
    }
    fn eq_opt<T: PartialEq>(a: &Option<T>, b: &Option<T>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }

    fn secondaries_all_equal(&self, other: &Self) -> bool {
        Self::eq_opt(&self.track_num, &other.track_num)
            && Self::eq_opt(&self.year, &other.year)
            && Self::eq_opt(&self.duration, &other.duration)
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        let same_title = eq_normalized(&self.title, &other.title);
        let same_album = eq_normalized(&self.album, &other.album);
        let same_artist = eq_normalized(&self.artist, &other.artist);

        match same_title as u8 + same_album as u8 + same_artist as u8 {
            3 => true,
            2 => self.secondaries_all_equal(other),
            _ => false,
        }
    }
}

fn eq_normalized(a: &str, b: &str) -> bool {
    normalized_chars(a).eq(normalized_chars(b))
}

fn normalized_chars(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars()
        .filter(|c| c.is_alphabetic() && !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
}
