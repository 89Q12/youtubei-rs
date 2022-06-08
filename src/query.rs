use serde_json::json;
use tracing::Level;

use crate::endpoints;
use crate::endpoints::*;
use crate::extractors::*;
use crate::types::channel::{ChannelTab,Tab};
use crate::types::client::ClientConfig;
use crate::types::error::Errors;
use crate::types::playlist::Playlist;
use crate::types::query_results::NextResult;
use crate::types::query_results::{CommentsQuery, VideoQuery, ChannelQuery,SearchQuery};
use crate::types::video::{SearchVideo,Video};
use crate::types::error::{RequestError, ParseError};
use crate::utils::unwrap_to_string;

pub async fn search_legacy(query: String,client_config: &ClientConfig) -> Result<SearchQuery, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Searching with query {}",query);
    let json = endpoints::search(&query, "", &client_config).await;
    match json {
        Ok(result) => Ok(extract_search_results(&result, false)),
        Err(err) => Err(err),
    }
}
pub async fn load_search_legacy(continuation:String,client_config: &ClientConfig) ->Result<SearchQuery, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Continuing search with continuation {}",continuation);
    let json = endpoints::search_continuation(&continuation, &client_config).await;
    match json {
        Ok(result) => Ok(extract_search_results(&result, true)),
        Err(err) => Err(err),
    }
}
pub async fn load_related_videos_legacy(continuation:String,client_config: &ClientConfig) -> Result<Vec<SearchVideo>, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading related videos with continuation {}",continuation);
    let json = next(&continuation, &client_config).await;
    match json {
        Ok(result) => Ok(load_related(&result)),
        Err(err) => Err(err),
    }
}
pub async fn get_comments_legacy(continuation:String,client_config: &ClientConfig) ->Result<CommentsQuery,  RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading comments with continuation {}",continuation);
    let comments_json = next(&continuation, client_config).await;
    match comments_json {
        Ok(result) => Ok(extract_comments(&result)),
        Err(err) => Err(err),
    }
}
pub async fn get_video_legacy(video_id:String, params: String,client_config: &ClientConfig) ->Result<VideoQuery,  RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading video with id {}",video_id);
    let player_json = player(&video_id, &params, &client_config).await;
    /*
    Error handling
    */
    let res = match player_json {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let res_next = next_with_data(serde_json::json!({
        "videoId": video_id,
        "params": params 
    }),&client_config).await;
    let next_video_data = match res_next {
        Ok(result) => result,
        Err(err) => return Err(err)
    };
    let video_player = extract_video_player_formats(&res["streamingData"]);
    let video: Video = video_from_next_and_player(&res, &next_video_data["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"], video_player);
    Ok(extract_next_video_results(&next_video_data, VideoQuery{
        continuation_comments: "".to_string(),
        continuation_related: unwrap_to_string(next_video_data["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"][20]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].as_str()),
        video,
        related_videos: Vec::new(),
    }))
}
pub async fn get_channel_info_legacy(channel_id:String,client_config: &ClientConfig) -> Result<ChannelQuery,  RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel info for channel: {}", channel_id);
    let complete_url = "https://www.youtube.com/".to_owned() + &channel_id +"/about"; 
    let resolved_url = resolve_url(&complete_url,&client_config ).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let channel_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        res["endpoint"]["browseEndpoint"]["params"].as_str().unwrap(), 
        &client_config
    ).await;
    match channel_json {
        Ok(result) => Ok(ChannelQuery{
            channel:extract_channel_info(&result),
        }),
        Err(err) => return Err(err),
    }
}
pub async fn get_channel_tab_url_legacy(channel_id:String,tab: Tab, client_config: &ClientConfig) -> Result<ChannelTab, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel tab: {} for channel: {}", tab.get_name(),channel_id);
    let index = tab.get_index();
    let complete_url = "https://www.youtube.com/".to_owned() + &channel_id +"/about"; 
    let resolved_url = resolve_url(&complete_url,&client_config).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let channel_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        res["endpoint"]["browseEndpoint"]["params"].as_str().unwrap(), 
        &client_config
    ).await;
    match channel_json {
        Ok(result) => Ok(extract_channel_tab(&result,index)),
        Err(err) => Err(err),
    }
}
pub async fn get_channel_tab_continuation_legacy(continuation:String,tab: Tab, client_config: &ClientConfig) -> Result<ChannelTab, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel tab: {}, with continuation: {}", tab.get_name(),continuation);
    let index = tab.get_index();
    let channel_json = browse_continuation(&continuation,&client_config).await;
    match channel_json {
        Ok(result) => Ok(extract_channel_tab(&result,index)),
        Err(err) => Err(err),
    }
}
pub async fn get_playlist_legacy(playlist_id: String,client_config: &ClientConfig)-> Result<Playlist, RequestError>{
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading playlist with id: {}",playlist_id);
    let complete_url = "https://www.youtube.com/playlist?list=".to_owned()+ &playlist_id;
    let resolved_url = resolve_url(&complete_url,&client_config).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let playlist_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"].as_str().unwrap(), 
        "", 
        &client_config
    ).await;
    match playlist_json {
        Ok(result) =>     Ok(extract_playlist(&result)),
        Err(err) => return Err(err),
    }
}

pub async fn next_video_id(video_id:String, params: String,client_config: &ClientConfig) -> Result<NextResult, Errors>{
    let json = next_with_data(json!({
        "videoId": video_id,
        "params": params,
    }), client_config).await;
    match json {
        Ok(json) => match extract_next_result(&json){
            Ok(result) => return Ok(result),
            Err(err) => return Err(Errors::ParseError(err))
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn next_continuation(continuation:String,client_config: &ClientConfig) -> Result<NextResult, Errors>{
    let json = next(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_next_result(&json){
            Ok(result) => return Ok(result),
            Err(err) => return Err(Errors::ParseError(err))
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
