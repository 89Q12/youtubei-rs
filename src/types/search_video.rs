use super::endpoints::{EndpointWatch, EndpointBrowse, EndpointNext};

pub struct SearchVideo{
    pub title: String,
    pub id: String,
    pub channel_name: String,
    pub description: String,
    pub author: String,
    pub author_verified: bool,
    pub channel_thumbnail: String,
    pub view_count_text: String,
    pub length_text: String,
    pub endpoint: EndpointWatch,
    pub browse_channel: EndpointBrowse,
    pub comments_continuation: EndpointNext
}