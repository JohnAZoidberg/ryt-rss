use std::vec::Vec;

extern crate serde_json;
use self::serde_json::{Value, Error};

extern crate reqwest;

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

fn parse_playlist(value: Value, playlist_id: String) -> Result<Podcast, Error> {
    let playlist     = &value["items"][0];
    let snippet      = &playlist["snippet"];
    //let playlist_id  = channel["contentDetails"]["relatedPlaylists"]["uploads"].as_str().expect("Not there").to_owned();

    let title        = snippet["title"].as_str().expect("Not there").to_owned();
    let description  = snippet["description"].as_str().expect("Not there").to_owned();
    let thumbnail    = snippet["thumbnails"]["high"]["url"].as_str().expect("Not there").to_owned();

    let episodes: Vec<Episode> = fetch_playlist_episodes(&playlist_id)?;

    Ok(Podcast {
        title: title,
        url: format!("https://www.youtube.com/playlist?list={}", playlist_id),
        //url: format!("https://www.youtube.com/playlist?list={}", playlist_id),
        description: description,
        build_date: None,
        thumbnail: thumbnail,
        episodes: episodes
    })
}

fn parse_channel(value: Value) -> Result<Podcast, Error> {
    let channel      = &value["items"][0];
    let snippet      = &channel["snippet"];
    let playlist_id  = channel["contentDetails"]["relatedPlaylists"]["uploads"].as_str().expect("Not there").to_owned();

    let title        = snippet["title"].as_str().expect("Not there").to_owned();
    let description  = snippet["description"].as_str().expect("Not there").to_owned();
    let thumbnail    = snippet["thumbnails"]["high"]["url"].as_str().expect("Not there").to_owned();

    let episodes: Vec<Episode> = fetch_playlist_episodes(&playlist_id)?;

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

fn fetch_playlist_episodes(playlist_id: &String) -> Result<Vec<Episode>, Error> {
    let url = build_url(Endpoint::PlaylistItems(playlist_id));

    // TODO lol handle potential errors properly
    // TODO get more than 50 videos
    let contents = reqwest::get(&url).expect("Foo").text().expect("Foo");

    let mut playlist: Value = serde_json::from_str(&contents)?;

    let items: &mut Vec<Value> = playlist["items"].as_array_mut().expect("Not there");
    items.iter().map(|ref item| parse_episode(item)).collect()
}

enum Endpoint <'a> {
    Channels(&'a ChannelIdentifier),
    Playlists(&'a String),
    PlaylistItems(&'a String)
}

fn build_url(endpoint: Endpoint) -> String {
    let endpoint_str = match endpoint {
        Endpoint::Channels(ChannelIdentifier::Username(username)) =>
            format!("channels?part=snippet%2CcontentDetails&forUsername={}", username),
        Endpoint::Channels(ChannelIdentifier::ChannelId(channel_id)) =>
            format!("channels?part=snippet%2CcontentDetails&id={}", channel_id),
        Endpoint::Playlists(playlist_id) =>
            format!("playlists?part=snippet&id={}", playlist_id),
        Endpoint::PlaylistItems(playlist_id) =>
            format!("playlistItems?part=snippet%2CcontentDetails&maxResults=50&playlistId={}", playlist_id),
    };
    let api_key = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    format!("https://www.googleapis.com/youtube/v3/{}&key={}",
            endpoint_str, api_key)
}

pub fn fetch_podcast(podcast_id: PodcastIdentifier) -> Result<Podcast, Error> {
    match podcast_id {
        PodcastIdentifier::Channel(channel_id) => {
            let url = build_url(Endpoint::Channels(&channel_id));
            // TODO lol handle potential errors properly
            let contents = reqwest::get(&url).expect("Foo").text().expect("Foo");
            let json: Value = serde_json::from_str(&contents)?;
            parse_channel(json)
        },
        PodcastIdentifier::Playlist(playlist_id) => {
            let url = build_url(Endpoint::Playlists(&playlist_id));
            // TODO lol handle potential errors properly
            let contents = reqwest::get(&url).expect("Foo").text().expect("Foo");
            let json: Value = serde_json::from_str(&contents)?;
            parse_playlist(json, playlist_id)
        }
    }
}
