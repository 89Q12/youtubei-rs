use std::fmt::Error;
use crate::endpoints;
use crate::endpoints::*;
use crate::extractors::*;
use crate::types::channel::Channel;
use crate::types::client_config::ClientConfig;
use crate::types::query_results::SearchQuery;
use crate::types::query_results::{CommentsQuery, VideoQuery, ChannelQuery};
use crate::types::search_video::SearchVideo;
use crate::types::video::Video;

pub async fn search(query: String,client_config: &ClientConfig) -> Result<SearchQuery, Error>{
    let json = endpoints::search(&query, "", &client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    return Ok(extract_search_results(&json, false));
}

pub async fn load_search(continuation:String,client_config: &ClientConfig) ->Result<SearchQuery, Error>{
    let json = endpoints::search_continuation(&continuation, &client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    return Ok(extract_search_results(&json, true));
}
pub async fn load_related_videos(continuation:String,client_config: &ClientConfig) -> Result<Vec<SearchVideo>, Error>{
    let json = next(&continuation, client_config).await;
    if !json["error"].is_null(){
        panic!("Unexpected error: {}", json["error"].to_string());
    }
    Ok(load_related(&json))
}
pub async fn load_playlists(continuation:String,client_config: &ClientConfig) -> Result<ChannelQuery, Error>{
    todo!()
}
pub async fn load_channel_videos(continuation:String,client_config: &ClientConfig) -> Result<ChannelQuery, Error>{
    todo!()
}
pub async fn get_comments(continuation:String,client_config: &ClientConfig) ->Result<CommentsQuery,  Error>{
    todo!()
}

pub async fn get_video(video_id:String, params: String,client_config: &ClientConfig) ->Result<VideoQuery,  Error>{
    let player_json = player(&video_id, &params, &client_config).await;
    /*
    Error handling
    */
    if player_json["playabilityStatus"]["status"].as_str().unwrap() == "ERROR" || !player_json["error"].is_null() {
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

pub async fn get_channel(url:String, tab:String,client_config: &ClientConfig) -> Result<ChannelQuery,  Error>{
    let complete_url = url+"/"+&tab; 
    let resolved_url = resolve_url(&complete_url,&client_config ).await;
    if !resolved_url["error"].is_null(){
        panic!("{}",resolved_url["error"]["message"]);
    }
    let channel_json = browse_browseid(resolved_url["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), resolved_url["endpoint"]["browseEndpoint"]["params"].as_str().unwrap(), &client_config).await;
    let channel: Channel = extract_channel(&channel_json, &tab);
    Ok(ChannelQuery{
        channel,
    })
}
pub async fn get_playlist(playlist_id: String, client_config: &ClientConfig){
    todo!()
}