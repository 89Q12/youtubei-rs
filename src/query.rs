use std::fmt::Error;

use crate::types::query_results::{SearchResult, CommentsQuery, VideoQuery, ChannelQuery};
pub fn search(query: String) -> SearchResult{
 todo!()
}

pub fn get_comments(continuation:String) ->Result<CommentsQuery,  Error>{
    todo!()
}

pub fn get_video(video_id:String, params: String) ->Result<VideoQuery,  Error>{
 todo!()
}
pub fn get_channel(browse_id:String, url: String) ->Result<ChannelQuery,  Error>{
todo!()
}