use super::{search_video::SearchVideo, search_playlist::SearchPlaylist, search_channel::SearchChannel, video::Video, channel::Channel};

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

}
pub struct VideoQuery{
    pub continuation: String,
    pub video:Video,
    pub related_videos: Vec<SearchVideo>,
}
pub struct ChannelQuery{
    pub continuation: String,
    pub channel: Channel,
    
}
