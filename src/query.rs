use std::fmt::Error;
use crate::endpoints::*;
use crate::extractors::*;
use crate::types::query_results::{SearchResult, CommentsQuery, VideoQuery, ChannelQuery};
use crate::utils::default_client_config;
use crate::types::video::Video;
pub async fn search(query: String) -> SearchResult{
 todo!()
}

pub async fn get_comments(continuation:String) ->Result<CommentsQuery,  Error>{
    todo!()
}

pub async fn get_video(video_id:String, params: String) ->Result<VideoQuery,  Error>{
    let client_config =default_client_config();
    let player_json = player(&video_id, &params, &client_config).await;
    /*
    Error handling
    */
    if player_json["playabilityStatus"]["status"].as_str().unwrap() == "ERROR" || !player_json["err"].is_null() {
        panic!("{}", player_json["playabilityStatus"]["reason"].as_str().unwrap());
    }
    let next_video_data = next_with_data(serde_json::json!({
        "videoId": video_id,
        "params": params 
    }),&client_config).await;
    let video_player = extract_video_player_formats(&player_json["streamingData"]);
    let video: Video = video_from_next_and_player(&player_json["videoDetails"], &next_video_data["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"], video_player);
    Ok(extract_next_video_results(&next_video_data, VideoQuery{
        continuation_comments: "".to_string(),
        continuation_related: next_video_data["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"][20]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].to_string(),
        video,
        related_videos: Vec::new(),
    }))
}
pub fn get_channel(browse_id:String, url: String) ->Result<ChannelQuery,  Error>{
todo!()
}