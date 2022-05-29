use super::{endpoints::{EndpointBrowse, EndpointWatch}, video::PlaylistVideo, channel::Author};

pub struct SearchPlaylist{
    pub title:String,
    pub id: String,
    pub author: Author,
    pub ucid:String,
    pub video_count: String,
    pub thumbnail: String,
    pub play_endpoint: EndpointWatch,
    pub browse_endpoint: EndpointBrowse,
}
pub struct Playlist{
    pub title:String,
    pub id: String,
    pub author: Author,
    pub video_count: String,
    pub updated_at: String,
    pub videos: Vec<PlaylistVideo>,
}

pub struct ChannelPlaylist{
    pub title:String,
    pub id: String,
    pub author_name: String,
    pub ucid:String,
    pub video_count: String,
    pub thumbnail: String,
    pub play_endpoint: EndpointWatch,
    pub browse_endpoint: EndpointBrowse,
}