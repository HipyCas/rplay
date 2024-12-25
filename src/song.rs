use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use std::{fs::File, path::PathBuf};

use anyhow::{Error, Result};
use audiotags::{AudioTag, Tag};
use delegate::delegate;
use getset::Getters;
use rodio::{Decoder, Source};

#[derive(Getters)]
pub struct Song {
    #[get = "pub"]
    path: PathBuf,
    name: Option<String>,
    #[get = "pub"]
    total_duration: Option<Duration>,
    tags: Box<dyn AudioTag + Send + Sync>,
}

impl Song {
    pub fn source(&self) -> Result<Decoder<BufReader<File>>> {
        let file = BufReader::new(File::open(&self.path).unwrap());
        Ok(Decoder::new(file)?)
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    // pub fn title(&self) -> Option<&str> {
    //     self.tags.title()
    // }

    delegate! {
        to self.tags {
            pub fn title(&self) -> Option<&str>;
            pub fn artist(&self) -> Option<&str>;
        }
    }
}

impl TryFrom<&Path> for Song {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self> {
        // Load file
        let file = BufReader::new(File::open(&value).unwrap());
        let source = Decoder::new(file)?;

        let total_duration = source.total_duration();

        Ok(Self {
            path: value.to_path_buf(),
            name: value
                .file_name()
                .map(|v| v.to_str().map(|s| s.to_owned()))
                .flatten(),
            total_duration,
            tags: Tag::new()
                .read_from_path(value)
                .expect("Could not read tags"),
        })
    }
}
