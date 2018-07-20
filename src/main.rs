#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//extern crate rocket_contrib;
//use rocket_contrib::Template;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate tera;
use std::collections::HashMap;
use tera::{Context, Tera, Value, Error};

#[derive(Serialize, Debug)]
struct Podcast {
    title: &'static str,
    url: &'static str,
    description: &'static str,
    episodes: Vec<Episode>,
    thumbnail: &'static str,
    build_date: Option<&'static str>
}

#[derive(Serialize, Debug)]
struct Episode {
    title: &'static str,
    url: &'static str,
    description: &'static str,
    file_url: &'static str,
    published_date: &'static str,
    duration: &'static str,
    length: &'static str
}

fn render_podcast(feed: &Podcast) -> Result<String, Error> {
    let mut tera: Tera = Tera::new("../templates/**/*")?;
    let mut ctx = Context::new();
    ctx.add("podcast", &feed);
    tera.register_filter("cdata", cdata);
    tera.render("basefeed.rss.tera", &ctx)
}

#[get("/user/<user_id>")]
fn index(user_id: String) -> Result<String, Error> {
    let feed = Podcast {
        title: "title",
        url: "url",
        description: "description",
        build_date: None,
        thumbnail: "",
        episodes: vec![Episode {
            title: "Episode 1",
            url: "url ep 1",
            description: "dfoosfaisdfasfa",
            file_url: "foo",
            published_date: "today",
            duration: "1235",
            length: ""
        }]
    };
    render_podcast(&feed)
}

pub fn cdata(val: Value, _hm: HashMap<String, Value>) -> tera::Result<Value> {
    Ok(tera::Value::String(format!("<![CDATA[{}]]>", val)))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        // I can't figure out how to register_filter with the fairing :(
        //.attach(Template::fairing())
        .launch();
}
