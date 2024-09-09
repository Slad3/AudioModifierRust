mod audio_modifiers;
mod realtime_playback;

use crate::realtime_playback::playback;
use std::io::{self};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

static AUDIO_DATA: LazyLock<Arc<Mutex<Vec<f32>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

fn main() -> Result<(), io::Error> {
    env_logger::init();
    let _ = thread::spawn(move || {
        let _ = playback(Arc::clone(&AUDIO_DATA));
    });

    loop {
        thread::sleep(Duration::from_millis(1));
        let audio = AUDIO_DATA.lock().unwrap().clone();
        println!("{}", audio.get(600).unwrap_or(&0.0f32));
    }
}
