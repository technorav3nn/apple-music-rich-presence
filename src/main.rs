use std::time::Duration;

use discord_presence::Client;
use music::get_now_playing;

use crate::music::{get_artwork_url, ITunesSearchResultItem};

mod music;
mod osascript;

macro_rules! info_log {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format_args!($($arg)*));
    }
}

fn format_secs(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}", minutes, seconds)
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

            let result = get_artwork_url(format!("{} {}", song.song, song.album).as_str())
                .unwrap_or(ITunesSearchResultItem {
                    artwork_url_100: String::from("https://i.imgur.com/removed.png"),
                    collection_name: String::from("no"),
                });

            let formatted_duration = format_secs(song.duration as u64);
            let formatted_position = format_secs(song.position as u64);

            client
                .set_activity(move |a| {
                    a.state(format!("on {}", song.album))
                        .details(format!(
                            "{} | {} - {}",
                            &song.song, formatted_position, formatted_duration
                        ))
                        .assets(|a| a.large_image(result.artwork_url_100).large_text(&song.song))
                })
                .unwrap();

            info_log!("updated presence!");
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
