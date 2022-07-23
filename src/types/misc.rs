use serde::Deserialize;
use serde_json::Value;

use super::{video::ReelItemRenderer, playlist::NextPlaylistWrapper, endpoints::{NavigationEndpoint, ContinuationEndpoint}, enums::{NextContents, ItemSectionRendererContents, TopLevelButtons}, channel::TabRendererWrapper, accessibility::{Accessibility, AccessibilityData}, thumbnail::{ThumbnailUrlOnlyWrapper, Thumbnails}};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunsSimpleTextAccessibility{
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
pub struct HashtagHeaderRenderer{
    pub hashtag: SimpleText,
    pub hashtag_info_text: SimpleText,
    pub avatar_facepile: Vec<super::thumbnail::Thumbnails>,
    pub background_color: u64,
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
pub struct TwoColumnSearchResultsRenderer{
    pub primary_contents: SectionListRendererWrapper,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShelfRenderer{
    pub title: RunsSimpleTextAccessibility,
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
pub struct NextContinuationDataWrapper{
    pub next_continuation_data: NextContinuationData,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NextContinuationData{
    pub continuation: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertsRenderer{
    pub alert_renderer:AlertRenderer
}
#[derive(Debug, Clone, Deserialize)]
pub struct AlertRenderer{
    pub text: SimpleText,
    #[serde(rename ="type")]
    pub alert_type: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRenderer{
    pub text: RunsSimpleTextAccessibility,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleColumnBrowseResultsRenderer{
    pub tabs: Vec<TabRendererWrapper>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludingResultsForRenderer{
    pub including_results_for: Runs,
    pub corrected_query: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagTileRenderer{
    pub hashtag: SimpleText,
    pub hashtag_info_text: SimpleText,
    pub hashtag_thumbnail: ThumbnailUrlOnlyWrapper,
    pub hashtag_video_count: SimpleText,
    pub hashtag_channel_count: SimpleText
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowingResultsForRenderer{
    pub corrected_query: Runs,
    pub original_query: Runs,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroformatDataRenderer{
    pub url_canonical: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Thumbnails,
    pub unlisted: bool,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroformatDataRendererWrapper{
    pub microformat_data_renderer: MicroformatDataRenderer,
}