pub struct EndpointWatch{
    pub url: String,
    pub video_id: String,
    pub playlist_id: String,
    pub params: String,
}

pub struct EndpointBrowse{
    pub url: String,
    pub browse_id: String,
    pub params: String,
}
pub struct EndpointNext{
    pub url:String,
    pub continuation: String,
}