use std::time::Duration;

use discord_presence::Client;
use music::get_now_playing;

use crate::music::get_artwork_url;

mod music;
mod osascript;

macro_rules! info_log {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format_args!($($arg)*));
    }
}

fn main() {
    let mut global_song = get_now_playing().unwrap();
    let mut client = Client::new(952320054870020146);

    let _ = client.start();

    std::thread::sleep(Duration::from_secs(1));

    loop {
        let song = get_now_playing().unwrap();
        if song != global_song {
            global_song = song.clone();

            let artwork =
                get_artwork_url(format!("{} {} {}", song.song, song.artist, song.album).as_str())
                    .unwrap();

            client
                .set_activity(move |a| {
                    a.state(format!("on {}", song.album))
                        .details(&song.song)
                        .assets(|a| a.large_image(artwork).large_text(&song.song))
                })
                .unwrap();

            info_log!("updated presence!");
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
