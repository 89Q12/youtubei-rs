use super::misc::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRenderer {
    pub channel_thumbnail_supported_renderers: ChannelThumbnailSupportedRenderers,
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: Runs,
    pub long_byline_text: Runs,
    pub published_time_text: Option<SimpleText>, // None if upcoming
    pub length_text: Option<AccessibilitySimpleText>,
    pub view_count_text: Option<RunsOptionAccessibilitySimpleText>, // None if upcoming
    pub navigation_endpoint: NavigationEndpoint,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub owner_text: Runs,
    pub short_byline_text: Runs,
    pub short_view_count_text: Option<RunsOptionAccessibilitySimpleText>, // None if upcoming
    pub upcoming_event_data: Option<UpcomingEventData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactVideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: RunsSimpleTextAccessibility,
    pub long_byline_text: Runs,
    pub published_time_text: Option<SimpleText>, // ONLY None if youtube returns a recommendation and the view count will be "Recommended to you"
    pub length_text: Option<AccessibilitySimpleText>, // None if its a live video
    pub view_count_text: RunsSimpleTextAccessibility, // contains runs if video is live_now instead of simple_text
    pub channel_thumbnail: Thumbnails,
    pub navigation_endpoint: NavigationEndpoint,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub short_byline_text: Runs,
    pub short_view_count_text: RunsSimpleTextAccessibility, // contains runs if video is live_now instead of simple_text
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridVideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: RunsSimpleTextAccessibility,
    pub published_time_text: Option<SimpleText>, // None when its a live video or if it's premiere
    pub view_count_text: Option<RunsSimpleTextAccessibility>, // None when its a live video
    pub navigation_endpoint: NavigationEndpoint,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub short_view_count_text: RunsSimpleTextAccessibility,
    pub upcoming_event_data: Option<UpcomingEventData>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPrimaryInfoRenderer {
    pub title: Runs,
    pub view_count: VideoViewCountRendererWrapper,
    pub date_text: SimpleText,
    pub video_actions: MenuRendererWrapper,
    pub super_title_link: Runs
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSecondaryInfoRenderer {
    pub owner: Owner,
    pub description: Option<Runs>,
    pub metadata_row_container: MetadataRowContainer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentThreadRenderer {
    pub comment: CommentRendererWrapper,
    pub replies: Option<CommentRepliesRendererWrapper>,
}
/// Represents a youtube short
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelItemRenderer {
    pub video_id: String,
    pub headline: SimpleText,
    pub thumbnail: Thumbnails,
    pub view_count_text: AccessibilitySimpleText,
    pub accessibility: Accessibility,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildVideoRenderer {
    pub length_text: AccessibilitySimpleText,
    pub navigation_endpoint: NavigationEndpoint,
    pub title: SimpleText,
    pub video_id: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildVideoRendererWrapper {
    pub child_video_renderer: ChildVideoRenderer,
}