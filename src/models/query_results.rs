use serde::Deserialize;

use super::{
    endpoints::NavigationEndpoint,
    enums::{HeaderContents, MetadataRenderers, TwoColumnTypes},
    misc::*,
    video::{
        PlayabilityStatus, PlayerCaptionsTracklistRenderer, PlayerMicroformatRenderer,
        PlayerOverlayRendererWrapper, StoryboardWrapper, StreamingData, VideoDetails,
    },
};

/// Represents a result from a arbitrary next query
/// Everything can be None but never are all at the same time None
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextResult {
    pub contents: Option<TwoColumnTypes>,
    pub player_overlays: Option<PlayerOverlayRendererWrapper>,
    pub on_response_received_endpoints: Option<Vec<OnResponseReceivedEndpoints>>,
}

/// Represents a result from a arbitrary player query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResult {
    pub streaming_data: Option<StreamingData>, // Only None if there is an error or the video is to premiere in the future
    pub video_details: VideoDetails,
    pub storyboards: Option<StoryboardWrapper>,
    pub microformat: PlayerMicroformatRenderer,
    pub playability_status: PlayabilityStatus,
    pub captions: Option<PlayerCaptionsTracklistRenderer>,
}

/// Represents a result from a arbitrary browse query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResult {
    pub contents: Option<TwoColumnTypes>,
    pub header: Option<HeaderContents>,
    pub metadata: Option<MetadataRenderers>,
    pub on_response_received_actions: Option<Vec<OnResponseReceivedActions>>,
    pub alerts: Option<Vec<AlertsRenderer>>,
    pub microformat: Option<MicroformatDataRendererWrapper>,
}
/// Represents a result from a arbitrary resolve_url query
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveResult {
    pub endpoint: NavigationEndpoint,
}

/// Represents a result from a arbitrary search query
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub refinements: Option<Vec<String>>,
    pub contents: TwoColumnTypes,
}
