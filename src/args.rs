use std::env::{args, current_dir};
use std::fs::canonicalize;
use std::path::PathBuf;

use getset::Getters;

#[derive(Debug, Getters)]
pub struct Args {
    #[get = "pub"]
    path: PathBuf,
    #[get = "pub"]
    shuffle: bool,
}

impl Args {
    pub fn load() -> Self {
        let mut args = args();
        // Get path
        let path = args
            .nth(1)
            .map(|s| canonicalize(s).expect("Invalid path"))
            .unwrap_or_else(|| current_dir().expect("Error getting current dir"));
        let shuffle = args.any(|a| a == "-s".to_owned());

        Self { path, shuffle }
    }
}
