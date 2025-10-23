use anyhow::Result;
use crate::metadata::TrackMetadata;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub tracks: Vec<TrackMetadata>,
}

impl Album {
    pub fn new(name: String, artist: String) -> Self {
        Self {
            name,
            artist,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: TrackMetadata) {
        self.tracks.push(track);
        // Sort by track number
        self.tracks.sort_by_key(|t| t.track_number.unwrap_or(0));
    }

    pub fn display_name(&self) -> String {
        if self.artist != "Unknown Artist" {
            format!("{} - {}", self.artist, self.name)
        } else {
            self.name.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLibrary {
    pub albums: Vec<Album>,
    pub all_tracks: Vec<TrackMetadata>,
    pub current_album_index: usize,
    pub current_track_index: usize,
    pub music_directory: Option<PathBuf>,
}

impl MusicLibrary {
    pub fn new() -> Self {
        Self {
            albums: Vec::new(),
            all_tracks: Vec::new(),
            current_album_index: 0,
            current_track_index: 0,
            music_directory: None,
        }
    }

    pub fn scan_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self.music_directory = Some(path.to_path_buf());
        self.albums.clear();
        self.all_tracks.clear();

        let mut album_map: HashMap<String, Album> = HashMap::new();
        let supported_extensions = ["mp3", "flac", "wav", "ogg", "m4a", "aac"];

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension().and_then(|s| s.to_str()) {
                if supported_extensions.contains(&extension.to_lowercase().as_str()) {
                    if let Ok(metadata) = TrackMetadata::from_file(file_path) {
                        self.all_tracks.push(metadata.clone());

                        let album_key = format!("{} - {}", metadata.artist, metadata.album);
                        let album = album_map
                            .entry(album_key.clone())
                            .or_insert_with(|| Album::new(metadata.album.clone(), metadata.artist.clone()));

                        album.add_track(metadata);
                    }
                }
            }
        }

        // Convert to sorted vector
        self.albums = album_map.into_values().collect();
        self.albums.sort_by(|a, b| a.display_name().cmp(&b.display_name()));

        // Reset indices
        self.current_album_index = 0;
        self.current_track_index = 0;

        Ok(())
    }

    pub fn get_current_album(&self) -> Option<&Album> {
        self.albums.get(self.current_album_index)
    }

    pub fn get_current_track(&self) -> Option<&TrackMetadata> {
        self.get_current_album()
            .and_then(|album| album.tracks.get(self.current_track_index))
    }

    pub fn get_current_track_path(&self) -> Option<String> {
        self.get_current_track().map(|track| track.file_path.clone())
    }

    pub fn next_album(&mut self) {
        if !self.albums.is_empty() {
            self.current_album_index = (self.current_album_index + 1) % self.albums.len();
            self.current_track_index = 0;
        }
    }

    pub fn prev_album(&mut self) {
        if !self.albums.is_empty() {
            self.current_album_index = if self.current_album_index == 0 {
                self.albums.len() - 1
            } else {
                self.current_album_index - 1
            };
            self.current_track_index = 0;
        }
    }

    pub fn next_track(&mut self) {
        if let Some(album) = self.get_current_album() {
            if !album.tracks.is_empty() {
                self.current_track_index = (self.current_track_index + 1) % album.tracks.len();
            }
        }
    }

    pub fn prev_track(&mut self) {
        if let Some(album) = self.get_current_album() {
            if !album.tracks.is_empty() {
                self.current_track_index = if self.current_track_index == 0 {
                    album.tracks.len() - 1
                } else {
                    self.current_track_index - 1
                };
            }
        }
    }

    pub fn set_album(&mut self, index: usize) {
        if index < self.albums.len() {
            self.current_album_index = index;
            self.current_track_index = 0;
        }
    }

    pub fn set_track(&mut self, index: usize) {
        if let Some(album) = self.get_current_album() {
            if index < album.tracks.len() {
                self.current_track_index = index;
            }
        }
    }

    pub fn get_album_tracks(&self, album_index: usize) -> Option<&Vec<TrackMetadata>> {
        self.albums.get(album_index).map(|album| &album.tracks)
    }

    pub fn is_empty(&self) -> bool {
        self.albums.is_empty()
    }

    pub fn album_count(&self) -> usize {
        self.albums.len()
    }

    pub fn track_count(&self) -> usize {
        self.get_current_album()
            .map(|album| album.tracks.len())
            .unwrap_or(0)
    }
}