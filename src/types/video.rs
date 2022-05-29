use super::endpoints::{EndpointWatch, EndpointBrowse};
pub struct Video{
   pub title: String,
   pub id: String,
   pub author: String,
   pub ucid: String,
   pub published: String,
   pub views: String,
   pub description_html: String,
   pub length_seconds: i64,
   pub live_now: bool,
   pub premiere_timestamp: String,
   pub author_verified: bool,
   pub video_player: VideoPlayer,
   pub channel_thumbnail: String,
   pub thumbnail: String,
}


pub struct VideoPlayer{
   pub formats: Vec<Format>,
   pub apdaptiveformts: Vec<Format>,
}

pub struct Format{
   pub itag: i64,
   pub url: String,
   pub mime_type: String,
   pub bitrate: i64,
   pub quality: String,
   pub fps: i64,
   pub quality_label: String,
   pub audio_quality: String,
}


pub struct SearchVideo{
    pub title: String,
    pub id: String,
    pub channel_name: String,
    pub published_text: String,
    pub author: String,
    pub author_verified: bool,
    pub channel_thumbnail: String,
    pub view_count_text: String,
    pub length_text: String,
    pub thumbnail: String,
    pub endpoint: EndpointWatch,
    pub browse_channel: EndpointBrowse,
}

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
pub struct PlaylistVideo{
   pub title: String, 
   pub id: String, 
   pub author: String, 
   pub thumbnail:String,
   pub length_text: String,
   pub index: i64,
   pub endpoint: EndpointWatch
}