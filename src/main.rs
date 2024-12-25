use crate::app::App;
use crate::args::Args;

mod app;
mod args;
mod queue;
mod song;

// fn main() {
//     let mut queue = Queue::default();
//     let mut old_queue = Vec::new();

//     // Get stream and build sink
//     // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     // let sink = Sink::try_new(&stream_handle).unwrap();

//     let args = Args::load();

//     if args.path().is_dir() {
//         for item in read_dir(&args.path()).expect("Cannot list item") {
//             let item = item.expect("Could not access item");
//             let path = item.path();
//             if item.file_type().unwrap().is_file()
//                 && matches!(path.extension().map(|it| it.to_str()), Some(Some("mp3")))
//             {
//                 match queue.add_song(path.as_path()) {
//                     Ok(s) => old_queue.push(s),
//                     Err(e) => eprintln!("{:?}", e),
//                 }
//             }
//         }
//     } else if args.path().is_file() {
//         match queue.add_song(args.path().as_path()) {
//             Ok(s) => old_queue.push(s),
//             Err(e) => eprintln!("{:?}", e),
//         }
//     }

//     if *args.shuffle() {
//         queue.shuffle();
//     }
//     // queue.push();

//     // Get tags
//     // let tag = Tag::default().read_from_path(&path);
//     // if let Err(e) = tag {
//     //     println!("{}", e.to_string())
//     // }
//     // return;
//     let mut engine = ConsoleEngine::init_fill(30).unwrap();

//     // current_sink_len = sink.len();
//     // queue.push_one();
//     let mut current_song = match queue.push() {
//         Ok(s) => s,
//         Err(e) => {
//             eprintln!("Error {:?}", e);
//             None
//         }
//     };
//     loop {
//         match engine.poll() {
//             Event::Frame => {
//                 engine.clear_screen();
//                 engine.fill_rect(
//                     0,
//                     1,
//                     (engine.get_width()) as i32,
//                     1,
//                     pxl(NOT_PLAYED_INDICATOR),
//                 );

//                 if queue.all_played() {
//                     current_song = match queue.push() {
//                         Ok(s) => s,
//                         Err(e) => {
//                             eprintln!("Error {:?}", e);
//                             None
//                         }
//                     };
//                 }
//                 if queue.queue_empty() {
//                     engine.print(0, 0, "No more songs in queue");
//                 } else {
//                     if let Some(song) = current_song.as_ref() {
//                         let mut tit = song.title().unwrap_or("UNKNOWN").to_owned();
//                         if let Some(true_art) = song.artist() {
//                             tit.push(' ');
//                             tit.push('-');
//                             tit.push(' ');
//                             tit.push_str(true_art);
//                         }
//                         engine.print(0, 0, &tit);
//                         let curr_pos = queue.current_pos();
//                         let duration_info = if let Some(total_dur) = song.total_duration() {
//                             format!(
//                                 "{}m {}s / {}m {}s",
//                                 curr_pos.as_secs() / 60,
//                                 curr_pos.as_secs() % 60,
//                                 total_dur.as_secs() / 60,
//                                 total_dur.as_secs() % 60
//                             )
//                         } else {
//                             format!("{}m {}s", curr_pos.as_secs() / 60, curr_pos.as_secs() % 60)
//                         };
//                         engine.print(
//                             (engine.get_width() - duration_info.len() as u32) as i32,
//                             0,
//                             &duration_info,
//                         );
//                         if let Some(total_dur) = song.total_duration() {
//                             engine.fill_rect(
//                                 0,
//                                 1,
//                                 (engine.get_width() as f64 / total_dur.as_secs() as f64
//                                     * queue.current_pos().as_secs() as f64)
//                                     as i32,
//                                 1,
//                                 pxl(PLAYED_INDICATOR),
//                             );
//                             // engine.print(
//                             //     0,
//                             //     2,
//                             //     &format!(
//                             //         "{} -> {} ({}) = {} ({})",
//                             //         queue.current_pos().as_secs(),
//                             //         total_dur.as_secs(),
//                             //         engine.get_width(),
//                             //         engine.get_width() as f64 / total_dur.as_secs() as f64
//                             //             * queue.current_pos().as_secs() as f64,
//                             //         engine.get_width() as f64 / total_dur.as_secs() as f64
//                             //     ),
//                             // );
//                         }
//                     }

//                     if !queue.queue_empty() {
//                         engine.print(
//                             0,
//                             3,
//                             &format!(
//                                 "QUEUE:\n{}",
//                                 queue
//                                     .next_songs(10)
//                                     .iter()
//                                     .map(|song| (song.title().unwrap_or("UNKNOWN"), song.artist()))
//                                     .fold(String::new(), |mut s, (it, art)| {
//                                         s.push_str(&it);
//                                         if let Some(true_art) = art {
//                                             s.push(' ');
//                                             s.push('-');
//                                             s.push(' ');
//                                             s.push_str(true_art);
//                                         }
//                                         s.push('\n');
//                                         s
//                                     })
//                             ),
//                         );
//                     }
//                 }
//                 engine.draw();
//             }
//             Event::Key(keyevent) => {
//                 if keyevent.code == KeyCode::Char('w') {
//                     queue.play();
//                 } else if keyevent.code == KeyCode::Char('e') {
//                     queue.pause();
//                 } else if keyevent.code == KeyCode::Char('q') {
//                     queue.stop();
//                     break;
//                 }
//             }
//             _ => {}
//         }
//     }
// }

fn main() {
    let args = Args::load();
    let mut app = App::new(args);

    app.run();
}
