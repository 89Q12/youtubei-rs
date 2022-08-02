use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub browse_endpoint: Option<BrowseEndpoint>,
    pub watch_endpoint: Option<WatchEndpoint>,
    pub continuation_endpoint: Option<ContinuationEndpoint>,
    pub reel_watch_endpoint: Option<ReelWatchEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelWatchEndpoint {
    pub video_id: String,
    pub params: String,
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
pub struct ContinuationEndpoint {
    pub continuation_command: Option<ContinuationCommand>,
    pub get_transcript: Option<GetTranscriptEndpoint>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ContinuationCommand {
    pub token: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GetTranscriptEndpoint {
    pub params: String,
}
