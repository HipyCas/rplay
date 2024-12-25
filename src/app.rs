use std::{fs::read_dir, path::Path};

use console_engine::{events::Event, pixel::pxl, ConsoleEngine, KeyCode};

use crate::{args::Args, queue::Queue, song::Song};

const PLAYED_INDICATOR: char = '█';
const NOT_PLAYED_INDICATOR: char = '_';

enum Tabs {
    Next,
    // TODO History
    Log,
}

pub struct App {
    engine: ConsoleEngine,
    queue: Queue,
    current_song: Option<Song>,
    current_tab: Tabs,
    args: Args,
    error_log: Vec<(bool, String)>,
}

impl App {
    pub fn new(args: Args) -> Self {
        let engine = ConsoleEngine::init_fill(30).unwrap();

        let mut queue = Queue::default();
        let mut error_log = Vec::new();

        if args.path().is_dir() {
            for item in read_dir(&args.path()).expect("Cannot list item") {
                let item = item.expect("Could not access item");
                let path = item.path();
                if item.file_type().unwrap().is_file()
                    && matches!(path.extension().map(|it| it.to_str()), Some(Some("mp3")))
                {
                    if let Err(e) = queue.add_song(path.as_path()) {
                        error_log.push((true, e.to_string()));
                    }
                }
            }
        } else if args.path().is_file() {
            if let Err(e) = queue.add_song(args.path().as_path()) {
                error_log.push((true, e.to_string()));
            }
        }

        if *args.shuffle() {
            queue.shuffle();
        }

        error_log.push((false, format!("Loaded {} songs", queue.len())));

        Self {
            engine,
            queue,
            args,
            error_log,
            current_song: None,
            current_tab: Tabs::Next,
        }
    }

    pub fn run(&mut self) -> () {
        // TODO For crossfade, I just really need to play the song (queue.push) before the other one ends
        self.current_song = match self.queue.push() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error {:?}", e);
                None
            }
        };
        loop {
            match self.engine.poll() {
                Event::Frame => {
                    self.engine.clear_screen();
                    self.engine.fill_rect(
                        0,
                        1,
                        (self.engine.get_width()) as i32,
                        1,
                        pxl(NOT_PLAYED_INDICATOR),
                    );

                    if self.queue.all_played() {
                        self.current_song = match self.queue.push() {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("Error {:?}", e);
                                None
                            }
                        };
                    }
                    if self.queue.queue_empty() {
                        self.engine.print(0, 0, "No more songs in queue");
                    } else {
                        if let Some(song) = self.current_song.as_ref() {
                            let mut tit = song.title().unwrap_or("UNKNOWN").to_owned();
                            if let Some(true_art) = song.artist() {
                                tit.push(' ');
                                tit.push('-');
                                tit.push(' ');
                                tit.push_str(true_art);
                            }
                            self.engine.print(0, 0, &tit);
                            let curr_pos = self.queue.current_pos();
                            let duration_info = if let Some(total_dur) = song.total_duration() {
                                format!(
                                    "{}m {}s / {}m {}s",
                                    curr_pos.as_secs() / 60,
                                    curr_pos.as_secs() % 60,
                                    total_dur.as_secs() / 60,
                                    total_dur.as_secs() % 60
                                )
                            } else {
                                format!("{}m {}s", curr_pos.as_secs() / 60, curr_pos.as_secs() % 60)
                            };
                            self.engine.print(
                                (self.engine.get_width() - duration_info.len() as u32) as i32,
                                0,
                                &duration_info,
                            );

                            // Render played bar
                            if let Some(total_dur) = song.total_duration() {
                                // TODO? Increment the total seconds by one fraction so it doesn't fill up when there's still song left
                                self.engine.fill_rect(
                                    0,
                                    1,
                                    (self.engine.get_width() as f64 / total_dur.as_secs() as f64
                                        * self.queue.current_pos().as_secs() as f64)
                                        as i32,
                                    1,
                                    pxl(PLAYED_INDICATOR),
                                );
                                // self.engine.print(
                                //     0,
                                //     2,
                                //     &format!(
                                //         "{} -> {} ({}) = {} ({})",
                                //         queue.current_pos().as_secs(),
                                //         total_dur.as_secs(),
                                //         self.engine.get_width(),
                                //         self.engine.get_width() as f64 / total_dur.as_secs() as f64
                                //             * queue.current_pos().as_secs() as f64,
                                //         self.engine.get_width() as f64 / total_dur.as_secs() as f64
                                //     ),
                                // );
                            }
                        }

                        // Render next songs
                        if !self.queue.queue_empty() {
                            self.engine.print(
                                0,
                                3,
                                &format!(
                                    "QUEUE:\n{}",
                                    self.queue
                                        .next_songs(10)
                                        .iter()
                                        .map(|song| (
                                            song.title().unwrap_or("UNKNOWN"),
                                            song.artist()
                                        ))
                                        .fold(String::new(), |mut s, (it, art)| {
                                            s.push_str(&it);
                                            if let Some(true_art) = art {
                                                s.push(' ');
                                                s.push('-');
                                                s.push(' ');
                                                s.push_str(true_art);
                                            }
                                            s.push('\n');
                                            s
                                        })
                                ),
                            );
                        }

                        self.engine.print(
                            0,
                            3 + 1 + 11,
                            &format!("Playing from: {:?}", self.args.path()),
                        );
                    }
                    self.engine.draw();
                }
                Event::Key(keyevent) => {
                    if keyevent.code == KeyCode::Char('w') {
                        self.queue.play();
                    } else if keyevent.code == KeyCode::Char('e') {
                        self.queue.pause();
                    } else if keyevent.code == KeyCode::Char('n') {
                        match self.queue.next() {
                            Ok(s) => self.current_song = s,
                            Err(e) => self.error_log.push((true, e.to_string())),
                        }
                    } else if keyevent.code == KeyCode::Char('q') {
                        self.queue.stop();
                        break;
                    }
                }
                _ => {}
            }
        }
    }
}
