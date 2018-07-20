extern crate tera;
use std::collections::HashMap;
use self::tera::{Context, Tera, Value, Error};

use super::*;

pub fn render_podcast(feed: &Podcast) -> Result<String, Error> {
    let mut tera: Tera = Tera::new("templates/**/*")?;
    let mut ctx = Context::new();
    ctx.add("podcast", &feed);
    tera.register_filter("cdata", cdata);
    tera.render("basefeed.rss.tera", &ctx)
}

pub fn cdata(val: Value, _hm: HashMap<String, Value>) -> tera::Result<Value> {
    Ok(tera::Value::String(format!("<![CDATA[{}]]>", val)))
}
