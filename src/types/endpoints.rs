/// The params to use in the query with the player endpoint.
/// Where url represents the youtube url of the video
pub struct EndpointWatch{
    pub url: String,
    pub video_id: String,
    pub playlist_id: String,
    pub params: String,
}
/// The params to use in the query with the browse endpoint.
/// Where url represents the youtube url of the channel/playlist
pub struct EndpointBrowse{
    pub url: String,
    pub browse_id: String,
    pub params: String,
}
pub struct EndpointNext{
    pub url:String,
    pub continuation: String,
}