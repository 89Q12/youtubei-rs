use super::endpoints::EndpointBrowse;

pub struct CommunityPost {
    pub content_text: String,
    pub content_attachment: String,
    pub author_name: String,
    pub author_thumbnail: String,
    pub vote_count: i64,
    pub published_time_text: String,
    pub browse_endpoint: EndpointBrowse,
}