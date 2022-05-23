use super::endpoints::EndpointBrowse;

pub struct SearchChannel{
   pub author: String,
   pub ucid: String,
   pub author_thumbnail: String,
   pub subscriber_count: String,
   pub video_count: i32,
   pub description_html: String,
   pub auto_generated: bool,
   pub author_verified: bool,
   pub endpoint: EndpointBrowse,
}

impl SearchChannel {
    
}