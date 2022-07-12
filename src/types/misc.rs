use serde::Deserialize;
use serde_json::Value;

use super::{video::{VideoRenderer, VideoPrimaryInfoRenderer, VideoSecondaryInfoRenderer, CompactVideoRenderer, GridVideoRenderer, CommentThreadRenderer, ReelItemRenderer}, playlist::{GridPlaylistRenderer, PlaylistRenderer, CompactRadioRenderer, NextPlaylistWrapper, RadioRenderer}, channel::{BackstagePostThreadRenderer, TabRenderer, ChannelRenderer, ChannelMetadataRenderer, ChannelVideoPlayerRenderer, GridChannelRenderer, ChannelAboutFullMetadataRenderer}};

#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnails{
    pub thumbnails: Vec<Thumbnail>
}
#[derive(Debug, Clone, Deserialize)]
pub struct  Thumbnail{
    pub url: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title{
    pub runs: Option<Vec<Run>>,
    pub accessibility:Option<Accessibility>,
    pub simple_text: Option<String>,    
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunsOption{
    pub runs: Option<Vec<Run>>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunsOptionAccessibilitySimpleText{
    pub runs: Option<Vec<Run>>,
    pub accessibility: Option<Accessibility>,
    pub simple_text: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Runs{
    pub runs: Vec<Run>,
    pub accessibility: Option<Accessibility>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run{
    pub text: String,
    pub navigation_endpoint: Option<NavigationEndpoint>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilitySimpleText {
    pub accessibility: Option<Accessibility>,
    pub simple_text: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub accessibility_data: AccessibilityData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessibilityData {
    pub label: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub browse_endpoint: Option<BrowseEndpoint>,
    pub watch_endpoint: Option<WatchEndpoint>,
    pub continuation_endpoint: Option<ContinuationEndpoint>,
    pub reel_watch_endpoint: Option<ReelWatchEndpoint>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelWatchEndpoint{
    pub video_id: String,
    pub params: String
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
    pub playlist_id: Option<String>,
    pub params: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
    pub canonical_base_url: Option<String>,
    pub params: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRendererVec{
    pub metadata_badge_renderer: MetadataBadgeRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer{
    pub icon: Option<IconType>,
    pub style: String,
    pub label: Option<String>,
    pub tooltip:  Option<String>,
    pub accessibility_data:  Option<AccessibilityData>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconType{
    pub icon_type: String
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRendererWrapper{
    pub menu_renderer: Option<MenuRenderer>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRenderer{
    pub accessibility: Accessibility,
    pub top_level_buttons: Option<Vec<TopLevelButtons>>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TopLevelButtons{
    ButtonRenderer(ButtonRenderer),
    ToggleButtonRenderer(ToggleButtonRenderer)
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer{
    pub navigation_endpoint: Option<NavigationEndpoint>,
    pub text: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleButtonRenderer{
    pub style: Value,
    pub is_toggled: bool,
    pub is_disabled: bool,
    pub default_icon: IconType,
    pub default_text: AccessibilitySimpleText,
    pub toggled_text: AccessibilitySimpleText,
    pub accessibility: AccessibilityData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoViewCountRenderer{
    pub view_count: Title,
    pub short_view_count: SimpleText,
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
pub struct ContinuationEndpoint{
    pub continuation_command: Option<ContinuationCommand>,
    pub get_transcript: Option<GetTranscriptEndpoint>
}
#[derive(Debug, Clone, Deserialize)]
pub struct ContinuationCommand{
    pub token: String
}
#[derive(Debug, Clone, Deserialize)]
pub struct GetTranscriptEndpoint{
    pub params: String
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemRendererWrapper{
    pub continuation_item_renderer:NavigationEndpoint
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemRenderer{
    pub continuation_endpoint:ContinuationEndpoint,
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
    pub description: SimpleText,
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
    pub short_description: String,
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
pub struct StorybordWrapper{
    pub player_storyboard_spec_renderer: PlayerStoryboardSpecRenderer,
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
pub struct ShelfContent{
    pub vertical_list_renderer: Option<VerticalListRenderer>,
    pub horizontal_list_renderer: Option<HorizontalListRenderer>,
    pub play_all_button: Option<PlayAllButton>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayAllButton{
    pub button_renderer: ButtonRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalListRenderer{
    pub items: Vec<ItemSectionRendererContents>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalListRenderer{
    pub items: Vec<ItemSectionRendererContents>,
    pub collapsed_item_count: usize,
    pub collapsed_state_button_text: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRendererWrapper{
    pub section_list_renderer: SectionListRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRenderer{
    pub contents: Vec<ItemSectionRendererContents>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum  ItemSectionRendererContents{
    ContinuationItemRenderer(ContinuationItemRenderer),
    GridRenderer(GridRenderer),
    GridVideoRenderer(GridVideoRenderer),
    BackstagePostThreadRenderer(BackstagePostThreadRenderer),
    ItemSectionRenderer(ItemSectionRenderer),
    PlaylistRenderer(PlaylistRenderer),
    VideoRenderer(VideoRenderer),
    ChannelRenderer(ChannelRenderer),
    GridChannelRenderer(GridChannelRenderer),
    GridPlaylistRenderer(GridPlaylistRenderer),
    ShelfRenderer(ShelfRenderer),
    ChannelVideoPlayerRenderer(ChannelVideoPlayerRenderer),
    ReelShelfRenderer(ReelShelfRenderer), // Shorts are stored in here
    SearchPyvRenderer(Value), // TODO FIND OUT WHAT IT IS
    CommentsEntryPointHeaderRenderer(Value),
    RadioRenderer(RadioRenderer), // Wrapper for CompactRadioRenderer
    ChannelAboutFullMetadataRenderer(ChannelAboutFullMetadataRenderer),
    RecognitionShelfRenderer(Value), // Channel members but needs more investigation
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HeaderContents{
    C4TabbedHeaderRenderer(C4TabbedHeaderRenderer),
    HashtagHeaderRenderer(HashtagHeaderRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NextContents{
    CompactPromotedVideoRenderer(Value), // Contains only ads which we dont need lol
    CompactRadioRenderer(CompactRadioRenderer), // Holds mixes? Aka autogenerated playlists
    CompactVideoRenderer(CompactVideoRenderer),
    CommentsEntryPointHeaderRenderer(Value),
    CommentRenderer(CommentRenderer),
    CommentThreadRenderer(CommentThreadRenderer),
    CommentsHeaderRenderer(Value),
    VideoPrimaryInfoRenderer(VideoPrimaryInfoRenderer),
    VideoSecondaryInfoRenderer(VideoSecondaryInfoRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer),
    ItemSectionRenderer(ItemSectionRenderer),
    PromotedSparklesWebRenderer(Value),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagHeaderRenderer{
    pub hashtag: SimpleText,
    pub hashtag_info_text: SimpleText,
    pub avatar_facepile: Vec<Thumbnails>,
    pub background_color: u64,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabRendererContent{
    RichGridRenderer(RichGridRenderer), // Hashtags are stored in there
    SectionListRenderer(SectionListRenderer),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RichGridRendererContent{
    RichItemRenderer(RichItemRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer), 
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
pub struct VideoRendererWrapper{
    pub video_renderer: VideoRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRendererWrapper{
    pub item_section_renderer:ItemSectionRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer{
    pub contents: Vec<ItemSectionRendererContents>,
    pub section_identifier: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRenderer{
    pub items: Vec<ItemSectionRendererContents>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstagePostRenderer{
    pub post_id: String,
    pub author_text: Runs,
    pub author_thumbnail: Thumbnails,
    pub author_endpoint: NavigationEndpoint,
    pub content_text: RunsOption,
    pub backstage_attachment: BackstageImageRenderer,
    pub published_time_text: Title,
    pub vote_count: AccessibilitySimpleText,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstageImageRenderer{
    pub backstage_image_renderer: Option<CommunityPostAttachmentImage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommunityPostAttachmentImage{
    pub image: Thumbnails,
    pub command: NavigationEndpoint,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnWatchNextResults{
    pub results: ResultsWrapper,
    pub secondary_results: SecondaryResultsWrapper,
    pub playlist: Option<NextPlaylistWrapper>
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultsWrapper{
    pub results: Results
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryResultsWrapper{
    pub secondary_results: SecondaryResults
}
#[derive(Debug, Clone, Deserialize)]
pub struct Results{
    pub contents: Vec<NextContents>
}
#[derive(Debug, Clone, Deserialize)]
pub struct SecondaryResults{
    pub results: Vec<NextContents>
}
#[derive(Debug, Clone, Deserialize)]
pub struct TwoColumnBrowseResultsRenderer{
    pub tabs: Vec<TabRendererWrapper>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabRendererWrapper{
    pub tab_renderer: Option<TabRenderer>,
    pub expandable_tab_renderer: Option<TabRenderer>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnSearchResultsRenderer{
    pub primary_contents: SectionListRendererWrapper,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnWrapper{
    pub two_column_browse_results_renderer: Option<TwoColumnBrowseResultsRenderer>,
    pub two_column_watch_next_results: Option<TwoColumnWatchNextResults>,
    pub two_column_search_results_renderer: Option<TwoColumnSearchResultsRenderer>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShelfRenderer{
    pub title: Title,
    pub content: ShelfContent,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelShelfRenderer{
    pub title: Runs,
    pub items: Vec<ReelShelfContent>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelShelfContent{
    pub reel_item_renderer:ReelItemRenderer
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderRendererWrapper{
    pub c4_tabbed_header_renderer: C4TabbedHeaderRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMetadataRendererWrapper{
    pub channel_metadata_renderer: ChannelMetadataRenderer,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderRenderer{
    pub channel_id: String,
    pub title: String,
    pub navigation_endpoint: NavigationEndpoint,
    pub avatar: Thumbnails,
    pub banner: Option<Thumbnails>,
    pub tv_banner: Option<Thumbnails>,
    pub mobile_banner: Option<Thumbnails>,
    pub badges: Option<Vec<BadgeRendererVec>>,
    pub subscriber_count_text: Option<AccessibilitySimpleText>,
   
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnResponseReceivedEndpoints{
    pub reload_continuation_items_command: Option<ReloadContinuationItemsCommand>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReloadContinuationItemsCommand{
    pub continuation_items: Vec<NextContents>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnResponseReceivedActions{
    pub append_continuation_items_action: AppendContinuationItemsAction
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendContinuationItemsAction{
    pub continuation_items: Vec<ItemSectionRendererContents>,
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
pub enum MetadataRowContents{
    MetadataRowRenderer(MetadataRowRenderer),
    MetadataRowHeaderRenderer(Value)
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowRenderer{
    pub title: SimpleText,
    pub contents: Vec<Title>,
    
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingEventData{
    pub start_time: String,
    pub upcoming_event_text:Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextContinuationDataWrapper{
    pub next_continuation_data: NextContinuationData,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NextContinuationData{
    pub continuation: String,
}