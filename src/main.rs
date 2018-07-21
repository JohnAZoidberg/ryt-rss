#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//extern crate rocket_contrib;
//use rocket_contrib::Template;

mod fetcher;
mod router;
mod renderer;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub enum ChannelIdentifier {
    Username(String),
    ChannelId(String)
}

pub enum PodcastIdentifier {
    Channel(ChannelIdentifier),
    Playlist(String)
}

#[derive(Serialize, Debug)]
pub struct Podcast {
    title: String,
    url: String,
    description: String,
    episodes: Vec<Episode>,
    thumbnail: String,
    build_date: Option<String>
}

#[derive(Serialize, Debug)]
pub struct Episode {
    title: String,
    url: String,
    description: String,
    file_url: String,
    published_date: String,
    duration: u32,
    length: u32
}

fn main() {
    rocket::ignite()
        .mount("/", routes![router::podcast_by_user_name,
                            router::podcast_by_channel_id,
                            router::podcast_by_playlist_id])
        // I can't figure out how to register_filter with the fairing :(
        //.attach(Template::fairing())
        .launch();
}
