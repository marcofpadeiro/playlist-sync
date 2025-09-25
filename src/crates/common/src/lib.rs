#[derive(Debug, Eq)]
pub struct Track {
    title: String,
    album: String,
    artist: String,
    track_num: Option<u32>,
    year: Option<u32>,
    duration: Option<u32>, // ms
}

impl Track {
    pub fn build(
        title: String,
        album: String,
        artist: String,
        track_num: Option<u32>,
        year: Option<u32>,
        duration: Option<u32>,
    ) -> Self {
        Self {
            title,
            album,
            artist,
            track_num,
            year,
            duration,
        }
    }
    fn normalize(&self) -> Track {
        Track::build(
            normalize_string(self.title.clone()),
            normalize_string(self.album.clone()),
            normalize_string(self.artist.clone()),
            self.track_num,
            self.year,
            self.duration,
        )
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
        let a = self.normalize();
        let b = other.normalize();

        let same_title = compare_strings(&a.title, &b.title);
        let same_album = compare_strings(&a.album, &b.album);
        let same_artist = compare_strings(&a.artist, &b.artist);

        let important_matches = same_title as u8 + same_album as u8 + same_artist as u8;

        match important_matches {
            3 => true,
            2 => a.secondaries_all_equal(&b),
            _ => false,
        }
    }
}

fn normalize_string(string: String) -> String {
    string.chars().filter(|c| c.is_ascii_alphabetic()).collect()
}
fn compare_strings(s1: &String, s2: &String) -> bool {
    s1 == s2 || s1.contains(s2) || s2.contains(s1)
}
