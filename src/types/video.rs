use super::{misc::*, endpoints::NavigationEndpoint, enums::MetadataRowContents, channel::ChannelThumbnailSupportedRenderers, thumbnail::Thumbnails, accessibility::Accessibility};
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
    pub view_count_text: Option<RunsSimpleTextAccessibility>, // None if upcoming
    pub navigation_endpoint: NavigationEndpoint,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub owner_badges: Option<Vec<BadgeRendererVec>>,
    pub owner_text: Runs,
    pub short_byline_text: Runs,
    pub short_view_count_text: Option<RunsSimpleTextAccessibility>, // None if upcoming
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
    pub super_title_link: Option<Runs>
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer{
    pub player_microformat_renderer: PlayerMicroformat
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformat{
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    pub description: Option<SimpleText>,// Presumably none if the video is a short
    pub external_channel_id:String,
    pub is_family_safe: bool,
    pub view_count: String,
    pub category: String,
    pub publish_date: String,
    pub owner_channel_name: String,
    pub available_countries: Vec<String>,
    pub live_broadcast_details: Option<LiveBroadcastDetails>,
    pub upload_date: String,

}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastDetails{
    pub is_live_now: bool,
    pub start_timestamp: String
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails{
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
    pub keywords: Option<Vec<String>>,
    pub channel_id: String,
    pub short_description:  Option<String>,// Presumably none if the video is a short
    pub thumbnail: Thumbnails,
    pub view_count: String,
    pub author: String,
    pub is_private: bool,
    pub is_live_content: bool,
    pub is_upcoming: Option<bool>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStoryboardSpecRenderer{
    pub spec: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardWrapper{
    pub player_storyboard_spec_renderer: Option<PlayerStoryboardSpecRenderer>,
    pub player_live_storyboard_spec_renderer: Option<PlayerStoryboardSpecRenderer>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub expires_in_seconds: String,
    pub formats: Vec<Format>,
    pub adaptive_formats: Vec<Format>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub itag: i64,
    pub mime_type: String,
    pub bitrate: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub init_range: Option<Range>,
    pub index_range: Option<Range>,
    pub last_modified: String,
    pub content_length: Option<String>,
    pub quality: String,
    pub fps: Option<i64>,
    pub quality_label: Option<String>,
    pub projection_type: ProjectionType,
    pub approx_duration_ms: Option<String>,
    pub audio_quality: Option<String>,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum VideoQuality {
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    Low,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    Medium
}

#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProjectionType {
    Rectangular,
    Mesh
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRendererWrapper{
    pub comment_renderer: CommentRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRenderer{
    pub author_text: SimpleText,
    pub author_thumbnail: Thumbnails,
    pub author_endpoint: NavigationEndpoint,
    pub content_text: Runs,
    pub published_time_text: Runs,
    pub comment_id: String,
    pub vote_count: Option<AccessibilitySimpleText>, // None if there is no likes on the comment
    pub reply_count: Option<i16>,
    pub author_comment_badge: Option<AuthorCommentBadgeRendererWrapper>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorCommentBadgeRendererWrapper{
    pub author_comment_badge_renderer: AuthorCommentBadgeRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorCommentBadgeRenderer{
    pub icon: IconType,
    pub icon_tooltip: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRepliesRendererWrapper{
    pub comment_replies_renderer: CommentRepliesRenderer
}
#[derive(Debug, Clone, Deserialize)]
pub struct CommentRepliesRenderer{
    pub contents: Vec<ContinuationItemRendererWrapper>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoViewCountRenderer{
    pub view_count:  Option<RunsSimpleTextAccessibility>,
    pub short_view_count: Option<SimpleText>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoViewCountRendererWrapper{
    pub video_view_count_renderer: VideoViewCountRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOwnerRenderer{
    pub thumbnail: Thumbnails,
    pub title: Runs,
    pub navigation_endpoint: NavigationEndpoint,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
    pub badges: Option<Vec<BadgeRendererVec>>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner{
    pub video_owner_renderer: VideoOwnerRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowContainer{
    pub metadata_row_container_renderer: MetadataRowContainerRenderer
}
#[derive(Debug, Clone, Deserialize)]
pub struct MetadataRowContainerRenderer{
    pub rows: Option<Vec<MetadataRowContents>>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowRenderer{
    pub title: SimpleText,
    pub contents: Vec<RunsSimpleTextAccessibility>,
    
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingEventData{
    pub start_time: String,
    pub upcoming_event_text:Runs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRendererWrapper{
    pub video_renderer: VideoRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverlayRendererWrapper{
    pub player_overlay_renderer: PlayerOverlayRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverlayRenderer{
    pub decorated_player_bar_renderer:  Option<DecoratedPlayerBarRendererWrapper>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecoratedPlayerBarRendererWrapper{
    pub decorated_player_bar_renderer: DecoratedPlayerBarRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecoratedPlayerBarRenderer{
    pub player_bar: PlayerBar,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBar{
    pub multi_markers_player_bar_renderer: MultiMarkersPlayerBarRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiMarkersPlayerBarRenderer{
    pub markers_map: Vec<MarkersMap>
}
#[derive(Debug, Clone, Deserialize)]
pub struct MarkersMap{
    pub key: String,
    pub value: MarkersMapValues
}
#[derive(Debug, Clone, Deserialize)]
pub struct MarkersMapValues{
    pub chapters: Option<Vec<Chapter>>,
    pub heatmap: Option<HeatMap>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter{
    pub chapter_renderer: ChapterRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterRenderer{
    pub title: SimpleText,
    pub time_range_start_millis: u32,
    pub on_active_command: OnActiveCommand,
    pub thumbnail: Thumbnails
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnActiveCommand{
    pub set_active_panel_item_action: SetActivePanelItemAction
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActivePanelItemAction{
    pub item_index: u16
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMap{
    pub heatmap_renderer: HeatMapRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMapRenderer{
    pub max_height_dp: u16,
    pub min_height_dp: u16,
    pub show_hide_animation_duration_millis: u16,
    pub heat_markers_decorations: Vec<HeatMarkersDecorations>,
    pub heat_markers: Vec<HeatMarkers>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMarkersDecorations{
    pub timed_marker_decoration_renderer: TimedMarkerDecorationRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimedMarkerDecorationRenderer{
    pub visible_time_range_start_millis: u32,
    pub visible_time_range_end_millis: u32,
    pub decoration_time_millis: u32,
    pub label: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMarkers{
    pub heat_marker_renderer: HeatMarkerRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMarkerRenderer{
    pub time_range_start_millis: u32,
    pub marker_duration_millis: u32,
    pub heat_marker_intensity_score_normalized: f32
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayabilityStatus{
    pub status: String,
    pub reason: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptionsTracklistRenderer{
    pub player_captions_tracklist_renderer: PlayerCaptionsTracklist,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct  PlayerCaptionsTracklist{
    pub caption_tracks: Vec<CaptionTrack>,
    pub audio_tracks: Vec<AudioTrack>,
    pub translation_languages: Vec<TranslationLanguages>,
    pub default_audio_track_index: u16
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptionTrack{
    pub base_url: String,
    pub name: SimpleText,
    pub vss_id: String,
    pub language_code: String,
    pub kind: Option<String>,
    pub is_translatable: bool
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack{
    pub caption_track_indices: Vec<u16>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationLanguages{
    pub language_code: String,
    pub language_name: SimpleText,
}
