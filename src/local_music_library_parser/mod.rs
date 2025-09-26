use std::{collections::HashMap, io::Error};

use crate::{provider::LocalSongsProvider, track::LocalTrack};

pub struct LocalMusicParser {}

impl LocalMusicParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalSongsProvider for LocalMusicParser {
    type Error = Error;

    fn get_tracks(&self) -> Result<HashMap<String, LocalTrack>, Self::Error> {
        unimplemented!("LocalMusicParser get_tracks is unimplemented");
    }
}
