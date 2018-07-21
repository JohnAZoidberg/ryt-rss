use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

extern crate serde_json;
use self::serde_json::{Value, Error};

use super::*;

fn parse_episode(value: &Value) -> Result<Episode, Error> {
    let snippet      = &value["snippet"];
    let playlist_id  = value["id"].as_str().expect("Not there").to_owned();

    let title        = snippet["title"].as_str().expect("Not there").to_owned();
    let description  = snippet["description"].as_str().expect("Not there").to_owned();
    let published_at = snippet["publishedAt"].as_str().expect("Not there").to_owned();

    Ok(Episode {
        title: title,
        url: format!("https://www.youtube.com/watch?v={}", playlist_id),
        description: description,
        file_url: String::from("TODO convert"),
        published_date: published_at,
        duration: 1235,
        length: 1024
    })
}

fn parse_podcast(value: Value) -> Result<Podcast, Error> {
    let channel      = &value["items"][0];
    let snippet      = &channel["snippet"];
    let playlist_id  = channel["contentDetails"]["relatedPlaylists"]["uploads"].as_str().expect("Not there").to_owned();

    let title        = snippet["title"].as_str().expect("Not there").to_owned();
    let description  = snippet["description"].as_str().expect("Not there").to_owned();
    let thumbnail    = snippet["thumbnails"]["high"]["url"].as_str().expect("Not there").to_owned();

    let episodes: Vec<Episode> = get_playlist_episodes(&playlist_id)?;

    Ok(Podcast {
        title: title,
        url: format!("https://www.youtube.com/channel/{}", snippet["id"]),
        //url: format!("https://www.youtube.com/playlist?list={}", playlist_id),
        description: description,
        build_date: None,
        thumbnail: thumbnail,
        episodes: episodes
    })
}

fn get_playlist_episodes(playlist_id: &String) -> Result<Vec<Episode>, Error> {
    let mut f = File::open("playlist.json").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file to string");
    let mut playlist: Value = serde_json::from_str(&contents)?;

    let items: &mut Vec<Value> = playlist["items"].as_array_mut().expect("Not there");
    items.iter().map(|&ref item| parse_episode(&item)).collect()
}

pub fn fetch_podcast(id: ChannelIdentifier) -> Result<Podcast, Error> {
    let url: String = match id {
        ChannelIdentifier::Username(username) =>
            format!("blah{}", username),
        ChannelIdentifier::ChannelId(channel_id) =>
            format!("blah{}", channel_id),
    };

    let mut f = File::open("channel.json").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file to string");
    let channel: Value = serde_json::from_str(&contents)?;

    parse_podcast(channel)
}
