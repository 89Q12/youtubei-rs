use serde::Deserialize;

use super::{video::{ChildVideoRendererWrapper}, misc::*};
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
    pub title: RunsSimpleTextAccessibility,
    pub thumbnail: Thumbnails,
    pub video_count_text: Runs,
    pub video_count_short_text: SimpleText,
    pub view_playlist_text: Runs,
    pub published_time_text:  Option<SimpleText>, // looks like it misses sometimes
    pub owner_badges:Option<Vec<BadgeRendererVec>>,
    pub navigation_endpoint: NavigationEndpoint,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactRadioRenderer{
    pub navigation_endpoint: NavigationEndpoint,
    pub playlist_id: String,
    pub secondary_navigation_endpoint: NavigationEndpoint,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    pub video_count_short_text: Runs,
    pub video_count_text: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextPlaylistWrapper{
    pub playlist: NextPlaylist,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextPlaylist{
    pub playlist_id: String,
    pub title: String,
    pub current_index: u16,
    pub total_videos: u16,
    pub owner_name: SimpleText,
    pub contents: Vec<PlaylistPanelVideoRendererWrapper>,
    pub endpoint: NavigationEndpoint,
    pub continuations: Vec<NextContinuationDataWrapper>,
    pub short_byline_text: Runs,
    pub long_byline_text: Runs,
    pub title_text:Runs,

}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoRendererWrapper{
    pub playlist_panel_video_renderer: PlaylistPanelVideoRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoRenderer{
    pub video_id: String,
    pub title: AccessibilitySimpleText,
    pub long_byline_text: Runs,
    pub short_byline_text: Runs,
    pub length_text: AccessibilitySimpleText,
    pub thumbnail: Thumbnails,
    pub index_text: SimpleText,
    pub navigation_endpoint: NavigationEndpoint,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioRenderer{
    pub navigation_endpoint: NavigationEndpoint,
    pub playlist_id: String,
    pub title: SimpleText,
    pub video_count_text: Runs,
    pub video_count_short_text: Runs,
    pub videos: Vec<ChildVideoRendererWrapper>,
    pub thumbnail: Thumbnails,
}