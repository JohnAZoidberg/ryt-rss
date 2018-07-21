# ryt-rss
Another implementation of [YoutubeRSS](https://github.com/JohnAZoidberg/youtuberss) after being frustrated by [Haskell](https://github.com/JohnAZoidberg/hyt-rss).

With this program you can convert any Youtube playlist or channel to a podcast RSS feed that you can subscribe to with your favourite RSS player.

## Status
Barely works. Code is not idiomatic rust by any stretch of the imagination but this project was written within the first 24 hours of writing my first Rust program.

## Building
1. Obtain rust - only tested with rust-nightly July 19 2018 (see Mozilla's [Rust Overlay](https://github.com/mozilla/nixpkgs-mozilla/blob/master/rust-overlay.nix))
2. Install dependencies: openssl
3. `cargo build`

## Running
`cargo run`

Oh you need a Youtube API v3 token from Google. Which is hardcoded at the moment because I have not figured out how to do config yet.

## Endpoints
- `/users/<user_id`
- `/channel/<channel_id`
- `/playlist/<playlist_id>`

## TODO
- Understand ownership better and remove .clone()
- fetch more than 50 videos of a channel/playlist
- fix some values of feed rss
- implement video_id => audio url (need to figure out how to simulate or call [youtube-dl](https://github.com/rg3/youtube-dl))
- Parse things like api_key from configuration file
- do proper error handling and not .expect()
  - Maybe create own Error type to unify Error type of the various libraries
