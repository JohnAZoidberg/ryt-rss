use super::*;

pub fn fetch_podcast(id: ChannelIdentifier) -> Result<Podcast, String> {
    let url: String = match id {
        ChannelIdentifier::Username(username) =>
            format!("blah{}", username),
        ChannelIdentifier::ChannelId(channel_id) =>
            format!("blah{}", channel_id),
    };
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
    Ok(feed)
}
