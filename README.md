# youtubei-rs
A asynchronous implementation of the invidious innertube aka youtubei API wrapper. <br>
Using tokio,reqwest, serde and serde_json.

# Dependencies
- serde_json 
- serde: features = ["derive"]
- reqwest: features = ["json","gzip"]
- tokio: features = ["full"]

# Roadmap
- query: load more videos from a playlist
- propper error handling
- implementing proxy support
- removing panics
- possibly optimize the extractor
- adding more endpoints

# Implemented endpoints
- next
- browse
- search
- resolve_url
- player

# Supported queries
- get_video: Fetches all information about the video except captions and storyboards
- get_channel_info: Fetches all channel informantion and about tab
- get_channel_tab: Fetches a specific tab like videos to get channel videos
- search: Search youtube
- load_search: Continue search with ctoken
- get_comments: Loads initial comments or more comments for video
- load_related_videos: Loads more related videos
- get_playlist: Loads a playlist

For more in depth infos take a look at [query.rs](https://github.com/11Tuvork28/youtubei-rs/blob/main/src/query.rs) and [tests.rs](https://github.com/11Tuvork28/youtubei-rs/blob/master/src/tests.rc)

# General information
The most structs have common fields such as thumbnail and others.

- thumbnail represent the url of the thumbnail
- views/view_count/view_count are all taken from the innertube data and they are in readable format eg 103K views might be the value of this field where views is always there
