use anyhow::Result;
use lofty::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub track_number: Option<u32>,
    pub duration: Option<u64>, // in milliseconds
    pub file_path: String,
}

impl TrackMetadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let tagged_file = lofty::read_from_path(path)?;

        let title = tagged_file
            .primary_tag()
            .and_then(|tag| tag.title())
            .or_else(|| {
                tagged_file
                    .tag(lofty::id3::v2::Id3v2Tag::default().tag_type())
                    .and_then(|tag| tag.title())
            })
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string().into()
            })
            .to_string();

        let artist = tagged_file
            .primary_tag()
            .and_then(|tag| tag.artist())
            .or_else(|| {
                tagged_file
                    .tag(lofty::id3::v2::Id3v2Tag::default().tag_type())
                    .and_then(|tag| tag.artist())
            })
            .unwrap_or(std::borrow::Cow::Borrowed("Unknown Artist"))
            .to_string();

        let album = tagged_file
            .primary_tag()
            .and_then(|tag| tag.album())
            .or_else(|| {
                tagged_file
                    .tag(lofty::id3::v2::Id3v2Tag::default().tag_type())
                    .and_then(|tag| tag.album())
            })
            .unwrap_or(std::borrow::Cow::Borrowed("Unknown Album"))
            .to_string();

        let track_number = tagged_file
            .primary_tag()
            .and_then(|tag| tag.track())
            .or_else(|| {
                tagged_file
                    .tag(lofty::id3::v2::Id3v2Tag::default().tag_type())
                    .and_then(|tag| tag.track())
            });

        let duration = Some(tagged_file.properties().duration().as_millis() as u64);

        Ok(Self {
            title,
            artist,
            album,
            track_number,
            duration,
            file_path: path.to_string_lossy().to_string(),
        })
    }

    pub fn display_title(&self) -> String {
        if let Some(track_num) = self.track_number {
            format!("{}. {}", track_num, self.title)
        } else {
            self.title.clone()
        }
    }

    pub fn display_artist(&self) -> String {
        self.artist.clone()
    }

    pub fn display_album(&self) -> String {
        self.album.clone()
    }
}