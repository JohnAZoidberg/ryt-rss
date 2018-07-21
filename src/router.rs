extern crate tera;
// TODO use other type
// TODO why self::tera ?
use self::tera::{Error};

use super::*;

#[get("/user/<user_id>")]
pub fn podcast_by_user_name(user_id: String) -> Result<String, Error> {
    let feed = fetcher::fetch_podcast(
        PodcastIdentifier::Channel(ChannelIdentifier::Username(user_id)))?;
    renderer::render_podcast(&feed)
}

#[get("/channel/<channel_id>")]
pub fn podcast_by_channel_id(channel_id: String) -> Result<String, Error> {
    let feed = fetcher::fetch_podcast(
        PodcastIdentifier::Channel(ChannelIdentifier::ChannelId(channel_id)))?;
    renderer::render_podcast(&feed)
}

#[get("/playlist/<playlist_id>")]
pub fn podcast_by_playlist_id(playlist_id: String) -> Result<String, Error> {
    let feed = fetcher::fetch_podcast(
        PodcastIdentifier::Playlist(playlist_id))?;
    renderer::render_podcast(&feed)
}
