use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use delegate::delegate;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::song::Song;

pub struct Queue {
    items: Vec<Song>,
    // items: VecDeque<Song>,
    sink: Sink,
    // Not used but must live same as Sink
    _output: (OutputStream, OutputStreamHandle),
    current_song_idx: i32,
}

impl Queue {
    pub fn add_song<'a>(&mut self, path: &'a Path) -> Result<()> {
        self.items.push(Song::try_from(path)?);

        Ok(())
    }

    pub fn push(&mut self) -> Result<Option<Song>> {
        if self.items.is_empty() {
            Ok(None)
        } else {
            let song = self.items.remove(0);
            self.sink.append(song.source()?);
            self.current_song_idx += 1;
            Ok(Some(song))
        }
    }

    // pub fn push_all(&self) -> Result<(), DecoderError> {
    //     for song in &self.items {
    //         self.sink.append(song.source()?);
    //     }

    //     Ok(())
    // }

    pub fn next(&mut self) -> Result<Option<Song>> {
        self.sink.clear();
        let s = self.push();
        self.play();
        s
    }

    // TODO Create a QueueWithHistory type that supports this method maybe?
    // pub fn current_song(&self) -> Option<&Song> {
    //     if self.current_song_idx < 0 {
    //         None
    //     } else {
    //         self.items.get(self.current_song_idx as usize)
    //     }
    // }

    pub fn next_songs(&self, n: usize) -> &[Song] {
        &self.items[0..n]
        // self.items
        //     .iter()
        //     .skip((self.current_song_idx + 1) as usize)
        //     .take(n)
        //     .collect()
    }

    pub fn shuffle(&mut self) {
        self.items.shuffle(&mut thread_rng());
    }

    pub fn queue_empty(&self) -> bool {
        self.current_song_idx + 1 >= (self.items.len() as i32)
    }

    delegate! {
        to self.items {
            pub fn len(&self) -> usize;
        }
    }

    delegate! {
        to self.sink {
            pub fn play(&self);
            pub fn pause(&self);
            pub fn stop(&self);

            #[call(empty)]
            pub fn all_played(&self) -> bool;

            #[call(get_pos)]
            pub fn current_pos(&self) -> Duration;
        }
    }
}

impl Default for Queue {
    fn default() -> Self {
        let output = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output.1).unwrap();

        Self {
            items: Vec::new(),
            sink,
            _output: output,
            current_song_idx: -1,
        }
    }
}
