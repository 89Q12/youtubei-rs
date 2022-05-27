use super::video_player::VideoPlayer;

pub struct Video{
   pub  title: String,
   pub  id: String,
   pub  author: String,
   pub  ucid: String,
   pub  published: String,
   pub  views: String,
   pub  description_html: String,
   pub  length_seconds: i64,
   pub  live_now: bool,
   pub  premiere_timestamp: String,
   pub  author_verified: bool,
   pub  video_player: VideoPlayer,
   pub channel_thumbnail: String,
   pub thumbnail: String,
}

impl Video {
    
}