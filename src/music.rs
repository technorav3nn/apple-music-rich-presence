use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

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
pub struct GetNowPlayingResult {
    pub state: MusicState,
    pub artist: String,
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

pub fn get_artwork_url(song: &str) -> Result<String, ()> {
    let res = ureq::get(&format!(
        "https://itunes.apple.com/search?term={}&entity=song",
        song
    ))
    .call()
    .map_err(|_| ())
    .unwrap();

    let json =
        json_to_hashmap(&res.into_string().unwrap(), vec!["resultCount", "results"]).unwrap();

    let results = json.get("results").unwrap().as_array().unwrap();
    let artwork = results
        .first()
        .unwrap()
        .get("artworkUrl100")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    Ok(artwork)
}

fn json_to_hashmap(json: &String, keys: Vec<&str>) -> Result<HashMap<String, Value>, ()> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
    let mut map = HashMap::new();
    for key in keys {
        let (k, v) = lookup.remove_entry(key).unwrap();
        map.insert(k, v);
    }
    Ok(map)
}
