use serde::{Deserialize, Deserializer};

use crate::osascript::run_osascript_js;

// i could use include_str!() but im lazy
pub static GET_MUSIC_SCRIPT: &'static str = include_str!("../scripts/get-now-playing.js");

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MusicState {
    Playing,
    Paused,
    Stopped,
}

impl<'a> Deserialize<'a> for MusicState {
    fn deserialize<D>(deserializer: D) -> Result<MusicState, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "playing" => MusicState::Playing,
            "paused" => MusicState::Paused,
            "stopped" => MusicState::Stopped,
            _ => panic!("invalid music state"),
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ITunesSearchResult {
    #[serde(rename = "resultCount")]
    pub result_count: i32,
    pub results: Vec<ITunesSearchResultItem>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ITunesSearchResultItem {
    #[serde(rename = "artworkUrl100")]
    pub artwork_url_100: String,
    #[serde(rename = "collectionName")]
    pub collection_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GetNowPlayingResult {
    pub state: MusicState,
    pub album: String,
    pub song: String,
    pub duration: f64,
    pub position: f64,
    #[serde(rename = "rawArtwork")]
    pub raw_artwork: String,
}

pub fn get_now_playing() -> Result<GetNowPlayingResult, ()> {
    match run_osascript_js::<GetNowPlayingResult>(GET_MUSIC_SCRIPT) {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("error: {}", e);
            Err(())
        }
    }
}

pub fn get_artwork_url(song: &str) -> Option<ITunesSearchResultItem> {
    let res = ureq::get(&format!(
        "https://itunes.apple.com/search?term={}&entity=song",
        song
    ))
    .call()
    .map_err(|_| ())
    .unwrap();

    let json = serde_json::from_str::<ITunesSearchResult>(
        &res.into_string().map_err(|_| ()).unwrap().to_string(),
    )
    .map_err(|_| ())
    .unwrap();
    let mut result = None;

    if json.result_count == 1 {
        result = Some(json.results[0].clone());
    } else if json.result_count > 1 {
        // If there are multiple results, find the right album
        result = json
            .results
            .iter()
            .find(|r| r.collection_name == song)
            .cloned();
    }

    result
}
