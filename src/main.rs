mod audio_modifiers;
mod realtime_playback;

use crate::realtime_playback::playback;
use log::info;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

static AUDIO_DATA: LazyLock<Arc<Mutex<Vec<f32>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

fn main() {
    env_logger::init();

    let _ = thread::spawn(move || {
        let _ = playback(Arc::clone(&AUDIO_DATA));
    });

    loop {
        thread::sleep(Duration::from_millis(1));
        let audio = AUDIO_DATA.lock().unwrap().clone();
        match audio.get(600) {
            Some(value) => {
                info!("{}", value);
            }
            None => {}
        }
    }
}
