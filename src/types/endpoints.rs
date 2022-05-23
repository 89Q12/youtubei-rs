pub enum EndpointsUrl {
    Browse,
    Player,
}
pub struct EndpointWatch{
    pub url: EndpointsUrl,
    pub video_id: String,
    pub playlist_id: String,
    pub params: String,
}

pub struct EndpointBrowse{
    pub url: EndpointsUrl,
    pub browse_id: String,
}