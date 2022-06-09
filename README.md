# youtubei-rs
A asynchronous implementation of the invidious innertube aka youtubei API wrapper. <br>
Using tokio,reqwest, serde and serde_json.

# Breaking changes
- old queries are now prefixed with _legacy, if you rely on those just rename them in your program they still function the same
# Dependencies
- serde_json 
- serde: features = ["derive"]
- reqwest: features = ["json","gzip"]
- tokio: features = ["full"]

# Roadmap
- implementing proxy support
- removing panics
- adding more endpoints

# Implemented endpoints
- next
- browse
- search
- resolve_url
- player

# Supported queries
- next_video_id // Get next(aka related and comments) results for a given videoId 
- next_continuation // Fetch more next(aka related and comments) results for a given ctoken
- browse_id // Get browse(aka a channel or playlist) results for a given browseId
- browse_continuation // Fetch more browse(aka a channel or playlist) results for a given ctoken
- resolve, // Resolve a given url
- player // Get player data for a given videoId

# Example 
```rust
use youtubei_rs::{query::player, utils::default_client_config, types::query_results::PlayerResult};
#[tokio::main]
async fn main() {
    // create default client_config with WEB client
    let client_config = &default_client_config();
    // get player for video with id gC6dQrScmHE
    let player: PlayerResult = player(String::from("gC6dQrScmHE"),String::from(""),&client_config).await.unwrap();
    println!("{}",video_query.video_details.title); // video title
}

```
# With logging
For logging tracing is used so tracing_subscribe can be installed for easier use of tracing. The library has as target youtubei_rs with debug,trace and error levels.
```rust
use youtubei_rs::{query::player, utils::default_client_config};
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "youtubei_rs=debug");
    tracing_subscriber::fmt::init();
    // create default client_config with WEB client
    let client_config = &default_client_config();
    // get player for video with id gC6dQrScmHE
    let player: PlayerResult = player(String::from("gC6dQrScmHE"),String::from(""),&client_config).await.unwrap();
    println!("{}",player.video_details.title); // video title
}

```
# Supported queries (legacy)
- get_video: Fetches all information about the video except captions and storyboards
- get_channel_info: Fetches all channel information and about tab
- get_channel_tab: Fetches a specific tab like videos to get channel videos
- search: Search youtube
- load_search: Continue search with ctoken
- get_comments: Loads initial comments or more comments for video
- load_related_videos: Loads more related videos
- get_playlist: Loads a playlist

For more in depth info take a look at [query.rs](https://github.com/11Tuvork28/youtubei-rs/blob/main/src/query.rs) and [tests.rs](https://github.com/11Tuvork28/youtubei-rs/blob/master/src/tests.rc)