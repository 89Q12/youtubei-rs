# youtubei-rs
An asynchronous implementation of the Invidious InnerTube (aka youtubei) API wrapper.

Uses `tokio`, `reqwest`, `serde` and `serde_json`.

## Configuration
The `client_config` can be configured to dump the JSON response into a file if it's not parsable,
which is on by default when using the `default_client_config()` function.

When creating the client_config manually, you can parse true/false as the last parameter to disable this behavior.

## Roadmap
 - [ ] Implement proxy support
 - [ ] Remove panics
 - [ ] Add more endpoints

## Implemented endpoints
 - `next`
 - `browse`
 - `search`
 - `resolve_url`
 - `player`

## Supported queries
 - `next_video_id`: Get next (aka related and comments) results for a given videoId
 - `next_continuation`: Fetch more next (aka related and comments) results for a given ctoken
 - `browse_id`: Get browse (aka a channel or playlist) results for a given browseId
 - `browse_continuation`: Fetch more browse(aka a channel or playlist) results for a given ctoken
 - `resolve`: Resolve a given URL
 - `player`: Get player data for a given videoId
 
 For in depth info, take a look at [query.rs](https://github.com/11Tuvork28/youtubei-rs/blob/main/src/query.rs) and [tests.rs](https://github.com/11Tuvork28/youtubei-rs/blob/master/src/tests.rc).

## Example

```rust
use youtubei_rs::{query::player, utils::default_client_config, tokio};

#[tokio::main]
async fn main() {
    // Create the default client_config with web client
    let client_config = &default_client_config();

    // Get player for video with id `gC6dQrScmHE`
    let player = player(
        String::from("gC6dQrScmHE"), // Video ID
        String::new(), // Parameters
        &client_config // Client config
    ).await.except("Couldn't fetch the video info");

    println!("{}", player.video_details.title); // Print the video title
}
```

### With logging

For logging, tracing is used so `tracing_subscribe` can be installed for easier use of tracing. The library has as target youtubei_rs with debug, trace, and error levels.

```rust
use youtubei_rs::{query::player, utils::default_client_config, tokio};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "youtubei_rs=debug");
    tracing_subscriber::fmt::init();

    // Create the default client_config with web client
    let client_config = &default_client_config();

    // Get player for video with id `gC6dQrScmHE`
    let player = player(
        String::from("gC6dQrScmHE"), // Video ID
        String::new(), // Parameters
        &client_config // Client config
    ).await.except("Couldn't fetch the video info");

    println!("{}", player.video_details.title); // Print the video title
}
```
