use super::endpoints::EndpointWatch;

pub struct ChannelVideo{
    pub title: String, 
    pub id: String, 
    pub published_text: String, 
    pub author: String, 
    pub author_verified: bool,
    pub thumbnail:String,
    pub view_count_text:  String,
    pub length_text: String,
    pub channel_thumbnail: String, 
    pub endpoint: EndpointWatch
}