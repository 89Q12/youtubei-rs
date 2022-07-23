use serde::Deserialize;

use super::{misc::*, endpoints::NavigationEndpoint, enums::{ItemSectionRendererContents, RichGridRendererContent, TabRendererContent}, video::VideoRendererWrapper, thumbnail::{Thumbnails, ThumbnailsAccessibility, ShowCustomThumbnailRenderer}, accessibility::Accessibility};

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
    pub selected: Option<bool>,
    pub content: Option<TabRendererContent>,
    pub expanded_text: Option<String>,
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
    pub post: CommunityPost
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
    pub video_count_text: Option<Runs>, // Seems to be None if there arent any videos
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAboutFullMetadataRenderer{
    pub description: Option<SimpleText>,
    pub view_count_text: SimpleText,
    pub joined_date_text: Runs,
    pub title: SimpleText,
    pub avatar: Thumbnails,
    pub country: Option<SimpleText>,
    pub channel_id: String
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommunityPost{
    SharedPostRenderer(SharedPostRenderer),
    BackstagePostRenderer(BackstagePostRenderer)
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridShowRenderer{
    pub navigation_endpoint: NavigationEndpoint,
    pub thumbnail_renderer: ShowCustomThumbnailRenderer,
    pub title: SimpleText,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRadioRenderer{
    pub navigation_endpoint: NavigationEndpoint,
    pub secondary_navigation_endpoint: NavigationEndpoint,
    pub playlist_id: String,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    pub video_count_text: Runs,
    pub video_count_short_text: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelFeaturedContentRenderer{
    pub title: Runs,
    pub items: ItemSectionRendererContents,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnailSupportedRenderers{
    pub channel_thumbnail_with_link_renderer: ChannelThumbnailWithLinkRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnailWithLinkRenderer{
    pub thumbnail: Thumbnails,
    pub accessibility: Accessibility,
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RichGridRenderer{
    pub contents: Vec<RichGridRendererContent>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RichItemRenderer{
    pub content: VideoRendererWrapper
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderRendererWrapper{
    pub c4_tabbed_header_renderer: C4TabbedHeaderRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderRenderer{
    pub channel_id: String,
    pub title: Option<String>, // None if the channels is terminated or the channel doesn't exist
    pub navigation_endpoint: Option<NavigationEndpoint>,// None if the channels is terminated or the channel doesn't exist
    pub avatar: Thumbnails,
    pub banner: Option<Thumbnails>,
    pub tv_banner: Option<Thumbnails>,
    pub mobile_banner: Option<Thumbnails>,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
   
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstagePostRenderer{
    pub post_id: String,
    pub author_text: Runs,
    pub author_thumbnail: Thumbnails,
    pub author_endpoint: NavigationEndpoint,
    pub content_text: RunsOption,
    pub backstage_attachment: Option<BackstageImageRenderer>,
    pub published_time_text: RunsSimpleTextAccessibility,
    pub vote_count: AccessibilitySimpleText,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPostRenderer{
    pub post_id: String,
    pub content: Runs,
    pub display_name: RunsSimpleTextAccessibility,
    pub endpoint: NavigationEndpoint,
    pub navigation_endpoint: NavigationEndpoint,
    pub original_post: BackstagePostRendererWrapper,
    pub thumbnail: ThumbnailsAccessibility
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstageImageRenderer{
    pub backstage_image_renderer: Option<CommunityPostAttachmentImage>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabRendererWrapper{
    pub tab_renderer: Option<TabRenderer>,
    pub expandable_tab_renderer: Option<TabRenderer>
}
#[derive(Debug, Clone, Deserialize)]
pub struct CommunityPostAttachmentImage{
    pub image: Thumbnails,
    pub command: NavigationEndpoint,
}