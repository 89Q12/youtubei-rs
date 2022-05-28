use super::{video::ChannelVideo, playlist::SearchPlaylist, endpoints::EndpointBrowse};

pub struct Channel{
    pub name: String,
    pub id: String,
    pub banner: String,
    pub avatar: String,
    pub description: String,
}
pub struct CommunityPost {
    pub content_text: String,
    pub content_attachment: String,
    pub author_name: String,
    pub author_thumbnail: String,
    pub vote_count: i64,
    pub published_time_text: String,
    pub browse_endpoint: EndpointBrowse,
}
pub struct SearchChannel{
    pub author: String,
    pub ucid: String,
    pub author_thumbnail: String,
    pub subscriber_count: String,
    pub video_count:String,
    pub description_html: String,
    pub auto_generated: bool,
    pub author_verified: bool,
    pub endpoint: EndpointBrowse,
 }
 
pub struct ChannelTab{
    pub title: String,
    pub selected: bool,
    pub content: Vec<TabTypes>,
    pub continuation: String,
}
pub enum TabTypes{
    Videos(ChannelVideo),
    Playlists(SearchPlaylist),
    Community(CommunityPost)
}