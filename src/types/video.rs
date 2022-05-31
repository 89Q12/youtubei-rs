use serde_json::Value;
use super::{endpoints::{EndpointWatch, EndpointBrowse}, channel::Author};

/// Represents a video with all the information aviable except for captions und storyboards
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
   pub whitelisted_regions: Vec<Value>, // TODO: remove Value and move to correct type
   pub likes: String,
   pub gerne: String,
   pub is_upcoming: bool,
   pub is_family_safe: bool,
}

/// Represents all formats aviable for the current video
pub struct VideoPlayer{
   pub formats: Vec<Format>,
   pub apdaptiveformts: Vec<Format>,
}

// Represents a format
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

/// Represents a video found by search query
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
/// Represents a video shown in a channel
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
// Represents a video in a playlist
pub struct PlaylistVideo{
   pub title: String, 
   pub id: String, 
   pub author: Author, 
   pub thumbnail:String,
   pub length_text: String,
   pub index: i64,
   pub endpoint: EndpointWatch
}
// Represents a comment under a video
pub struct Comment{
   pub comment_id: String,
   pub text: String,
   pub author: Author,
   pub is_author_channel_owner: bool,
   pub author_thumbnail: String,
   pub replies: i64,
   pub reply_continuation: String,
   pub published_time_text: String,
   pub vote_count: String,
}