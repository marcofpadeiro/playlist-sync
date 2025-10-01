mod metadata_parser;
use std::{collections::HashMap, fs::read_dir, io, path::Path};

use anyhow::anyhow;

use crate::track::Track;

pub fn get_local_music_library_tracks(dir: &str) -> anyhow::Result<HashMap<String, Vec<Track>>> {
    let path = Path::new(dir);

    if !path.exists() {
        return Err(anyhow!("Path {} does not exist", dir));
    }
    if !path.is_dir() {
        return Err(anyhow!("Path \"{}\" is not a directory not exist", dir));
    }

    let mut result: HashMap<String, Vec<Track>> = HashMap::new();
    get_files_in_dir(path)?.iter().for_each(|file| {
        if let Some(track) = Track::from_file(file) {
            result
                .entry(track.artist.clone())
                .or_insert(vec![])
                .push(track);
        }
    });
    Ok(result)
}

fn get_files_in_dir(path: &Path) -> io::Result<Vec<String>> {
    let mut result = Vec::new();

    // Read the entries in the directory
    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let files_in_subdir = get_files_in_dir(&entry_path)?;
            result.extend(files_in_subdir);
        } else {
            result.push(entry_path.to_string_lossy().to_string());
        }
    }

    Ok(result)
}
