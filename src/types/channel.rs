use serde::Deserialize;

use super::{video::ChannelVideo, playlist::{ChannelPlaylist}, endpoints::EndpointBrowse, misc::*};

/// Represents a channel info that is returned by the api when querying the channel.
pub struct Channel{
    pub name: String,
    pub id: String,
    pub banner: String,
    pub avatar: String,
    pub description: String,
}
/// Represents a post in the community section of a channel.
pub struct CommunityPost {
    pub content_text: String,
    pub content_attachment: String,
    pub author_name: String,
    pub author_thumbnail: String,
    pub vote_count: i64,
    pub published_time_text: String,
    pub browse_endpoint: EndpointBrowse,
}

/// Represents a channel that is found in search results.
pub struct SearchChannel{
    pub author: Author,
    pub ucid: String,
    pub author_thumbnail: String,
    pub subscriber_count: String,
    pub video_count:String,
    pub description_html: String,
    pub auto_generated: bool,
    pub endpoint: EndpointBrowse,
 }
 /// Represents a channel tab e.g. videos.
pub struct ChannelTab{
    pub title: String,
    pub selected: bool,
    pub content: Vec<TabTypes>,
    pub continuation: String,
}
pub struct Author{
    pub name: String,
    pub verified: bool,
    pub browse_endpoint: EndpointBrowse,
}
/// Represents the type a tab has.
pub enum TabTypes{
    Videos(ChannelVideo),
    Playlists(ChannelPlaylist),
    Community(CommunityPost)
}
// Basically the same but without types associated to the enum values.
pub enum Tab{
    Videos,
    Playlists,
    Community
}
impl Tab {
    /// Used to determine which tab is active by extractors
    /// ```
    ///  use youtubei_rs::types::channel::Tab;
    ///  let tab = Tab::Videos;
    ///  assert_eq!(tab.get_name(), "videos");
    ///  assert_eq!(tab.get_index(), 1);
    /// 
    /// ```
    pub fn get_name(&self) -> &str {
        match *self {
            Tab::Videos => "videos",
            Tab::Playlists => "playlists",
            Tab::Community => "community"
        }
    }
    pub fn get_index(&self) -> usize {
        match *self {
            Tab::Videos => 1,
            Tab::Playlists => 2,
            Tab::Community => 3,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRenderer{
    pub channel_id: String,
    pub title: SimpleText,
    pub navigation_endpoint: NavigationEndpoint,
    pub thumbnail: Thumbnails,
    pub description_snippet: Runs,
    pub short_byline_text: Runs,
    pub video_count_text: Runs,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
    pub long_byline_text: Runs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TabRenderer{
    pub endpoint: Option<NavigationEndpoint>,
    pub title: Option<String>,
    pub selected: bool,
    pub content: Option<TabRendererContent>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMetadataRenderer{
    pub title: String,
    pub description: String,
    pub rss_url: String,
    pub external_id: String,
    pub keywords: String,
    pub owner_urls: Vec<String>,
    pub avatar: Thumbnails,
    pub channel_url: String,
    pub is_family_safe: bool,
    pub available_country_codes: Vec<String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct BackstagePostThreadRenderer{
    pub post: BackstagePostRendererWrapper
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstagePostRendererWrapper{
    pub backstage_post_renderer: BackstagePostRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoPlayerRenderer{
    pub video_id: String,
    pub title: Runs,
    pub description: Runs,
    pub view_count_text: SimpleText,
    pub published_time_text:Runs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridChannelRenderer{
    pub channel_id: String,
    pub title: SimpleText,
    pub navigation_endpoint: NavigationEndpoint,
    pub thumbnail: Thumbnails,
    pub video_count_text: Runs,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
}