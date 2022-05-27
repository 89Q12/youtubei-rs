use super::endpoints::EndpointWatch;

pub struct ChannelVideo{
    title: String, 
    id: String, 
    published_text: String, 
    author: String, 
    author_verified: bool,
    thumbnail:String,
    view_count_text:  String,
    length_text: String,
    channel_thumbnail: String, 
    endpoint: EndpointWatch
}