use std::{collections::HashMap, io::Error, path::PathBuf};

use crate::track::Track;

pub fn get_local_music_library_tracks(
    _path: &PathBuf,
) -> Result<HashMap<String, (Track, String)>, Error> {
    unimplemented!("LocalMusicParser get_tracks is unimplemented");
}
