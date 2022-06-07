use serde::Deserialize;

use super::{endpoints::{EndpointBrowse, EndpointWatch}, video::PlaylistVideo, channel::Author, misc::*};
///  Represents a playlist found in search results.
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
///  Represents a playlist that is returned by the api when queried by playlist ids.
pub struct Playlist{
    pub title:String,
    pub id: String,
    pub author: Author,
    pub video_count: String,
    pub updated_at: String,
    pub videos: Vec<PlaylistVideo>,
    pub continuation: String,
}
/// Represents a playlist found in the channel tab "playlists".
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
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistRenderer{
    pub playlist_id: String,
    pub title: SimpleText,
    pub thumbnails: Vec<Thumbnails>,
    pub video_count: String,
    pub view_playlist_text: Runs,
    pub short_byline_text: Runs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPlaylistRenderer{
    pub playlist_id: String,
    pub title: Title,
    pub thumbnail: Thumbnails,
    pub video_count_text: Runs,
    pub video_count_short_text: SimpleText,
    pub view_playlist_text: Runs,
    pub published_time_text: SimpleText,
    pub owner_badges:Option<Vec<BadgeRendererVec>>,
    pub navigation_endpoint: NavigationEndpoint,
}