use super::{endpoints::{EndpointWatch, EndpointBrowse}, channel::Author};
pub struct Video{
   pub title: String,
   pub id: String,
   pub author: Author,
   pub ucid: String,
   pub published: String,
   pub views: String,
   pub description_html: String,
   pub length_seconds: i64,
   pub live_now: bool,
   pub premiere_timestamp: String,
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
    pub author: Author,
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
    pub author_name: String,
    pub thumbnail:String,
    pub view_count_text:  String,
    pub length_text: String,
    pub channel_thumbnail: String, 
    pub endpoint: EndpointWatch
}
pub struct PlaylistVideo{
   pub title: String, 
   pub id: String, 
   pub author: Author, 
   pub thumbnail:String,
   pub length_text: String,
   pub index: i64,
   pub endpoint: EndpointWatch
}
pub struct Comment{
   pub comment_id: String,
   pub text: String,
   pub author: Author,
   pub replies: i64,
   pub reply_continuation: String,
   pub published_time_text: String,
   pub vote_count: String,
}