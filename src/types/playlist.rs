use super::{endpoints::{EndpointBrowse, EndpointWatch}};

pub struct SearchPlaylist{
    pub title:String,
    pub id: String,
    pub author: String,
    pub ucid:String,
    pub video_count: i64,
    pub thumbnail: String,
    pub author_verified: bool,
    pub play_endpoint: EndpointWatch,
    pub browse_endpoint: EndpointBrowse,
}