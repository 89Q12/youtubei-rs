use serde::Deserialize;

use super::{video::SearchVideo, playlist::SearchPlaylist, channel::{SearchChannel, ChannelMetadataRenderer}, video::{Video, Comment}, channel::Channel, misc::*};
/// Enum that represents search results.
pub enum SearchResultEnum {
    VideoRenderer(SearchVideo),
    PlaylistRenderer(SearchPlaylist),
    SearchChannel(SearchChannel)
}
/// Represents a search query.
pub struct SearchQuery{
    pub continuation: String,
    pub results: Vec<SearchResultEnum>,
}
/// Represents a comment query.
pub struct CommentsQuery{
    pub comments: Vec<Comment>,
    pub continuation: String,
}
/// Represents a query to get a specific video with all information.
pub struct VideoQuery{
    pub continuation_comments: String,
    pub continuation_related: String,
    pub video:Video,
    pub related_videos: Vec<SearchVideo>,
}
/// Needs some refactoring but it should represent a channel query with the queried tab.
// Currently its a useless abstraction
pub struct ChannelQuery{
    pub channel: Channel,
}


/// Represents a result from a arbitrary next query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextResult{
   pub contents: TwoColumnWrapper,
   pub player_overlays:  PlayerOverlayRendererWrapper,
   pub on_response_received_endpoints: Vec<OnResponseReceivedEndpoints>
}

/// Represents a result from a arbitrary player query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResult{
   pub streaming_data: StreamingData,
   pub video_details: VideoDetails,
   pub storyboards: StorybordWrapper,
   pub microformat: PlayerMicroformatRenderer,
   pub playability_status:PlayabilityStatus,
   pub captions: PlayerCaptionsTracklistRenderer
}


/// Represents a result from a arbitrary browse query
#[derive(Debug, Clone, Deserialize)]
pub struct BrowseResult{
    pub contents: TwoColumnWrapper,
    pub header: C4TabbedHeaderRenderer,
    pub metadata: ChannelMetadataRenderer
}
/// Represents a result from a arbitrary resolve_url query
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveResult{
    pub endpoint: BrowseEndpoint
}

/// Represents a result from a arbitrary search query
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult{
    pub refinements: Vec<String>,
    pub contents: TwoColumnWrapper,
}
