use super::{video::SearchVideo, playlist::SearchPlaylist, channel::SearchChannel, video::{Video, Comment}, channel::Channel};
/// Enum that represents search results.
pub enum SearchResult {
    VideoRenderer(SearchVideo),
    PlaylistRenderer(SearchPlaylist),
    SearchChannel(SearchChannel)
}
/// Represents a search query.
pub struct SearchQuery{
    pub continuation: String,
    pub results: Vec<SearchResult>,
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
