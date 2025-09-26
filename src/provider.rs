use std::collections::HashMap;

use crate::track::{LocalTrack, Track};

pub trait PlaylistProvider {
    type Error;

    fn get_tracks(&self) -> Result<Vec<Track>, Self::Error>;
}

pub trait LocalSongsProvider {
    type Error;

    fn get_tracks(&self) -> Result<HashMap<String, LocalTrack>, Self::Error>;
}
