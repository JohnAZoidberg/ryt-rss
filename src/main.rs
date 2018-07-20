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

#[derive(Serialize, Debug)]
pub struct Podcast {
    title: &'static str,
    url: &'static str,
    description: &'static str,
    episodes: Vec<Episode>,
    thumbnail: &'static str,
    build_date: Option<&'static str>
}

#[derive(Serialize, Debug)]
pub struct Episode {
    title: &'static str,
    url: &'static str,
    description: &'static str,
    file_url: &'static str,
    published_date: &'static str,
    duration: &'static str,
    length: &'static str
}

fn main() {
    rocket::ignite()
        .mount("/", routes![router::podcast_by_user_name,
                            router::podcast_by_channel_id])
        // I can't figure out how to register_filter with the fairing :(
        //.attach(Template::fairing())
        .launch();
}
