use super::{video::SearchVideo, playlist::SearchPlaylist, channel::SearchChannel, video::{Video, Comment}, channel::Channel};

pub enum SearchResult {
    VideoRenderer(SearchVideo),
    PlaylistRenderer(SearchPlaylist),
    SearchChannel(SearchChannel)
}


pub struct SearchQuery{
    pub continuation: String,
    pub results: Vec<SearchResult>,
}
pub struct CommentsQuery{
    pub comments: Vec<Comment>,
    pub continuation: String,
}
pub struct VideoQuery{
    pub continuation_comments: String,
    pub continuation_related: String,
    pub video:Video,
    pub related_videos: Vec<SearchVideo>,
}
pub struct ChannelQuery{
    pub channel: Channel,
}
