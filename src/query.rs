use std::fs;

use serde_json::json;
use serde_json::Value;
use tracing::Level;

use crate::endpoints;
use crate::endpoints::*;
use crate::extractors::*;
use crate::types::channel::{ChannelTab, Tab};
use crate::types::client::ClientConfig;
use crate::types::error::Errors;
use crate::types::error::RequestError;
use crate::types::playlist::Playlist;
use crate::types::query_results::BrowseResult;
use crate::types::query_results::NextResult;
use crate::types::query_results::PlayerResult;
use crate::types::query_results::ResolveResult;
use crate::types::query_results::SearchResult;
use crate::types::query_results::{ChannelQuery, CommentsQuery, SearchQuery, VideoQuery};
use crate::types::video::{SearchVideo, Video};
use crate::utils::unwrap_to_string;

pub async fn search_legacy(
    query: String,
    client_config: &ClientConfig,
) -> Result<SearchQuery, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Searching with query {}",query);
    let json = endpoints::search(&query, "", &client_config).await;
    match json {
        Ok(result) => Ok(extract_search_results(&result, false)),
        Err(err) => Err(err),
    }
}
pub async fn load_search_legacy(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<SearchQuery, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Continuing search with continuation {}",continuation);
    let json = endpoints::search_continuation(&continuation, &client_config).await;
    match json {
        Ok(result) => Ok(extract_search_results(&result, true)),
        Err(err) => Err(err),
    }
}
pub async fn load_related_videos_legacy(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<Vec<SearchVideo>, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading related videos with continuation {}",continuation);
    let json = next(&continuation, &client_config).await;
    match json {
        Ok(result) => Ok(load_related(&result)),
        Err(err) => Err(err),
    }
}
pub async fn get_comments_legacy(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<CommentsQuery, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading comments with continuation {}",continuation);
    let comments_json = next(&continuation, client_config).await;
    match comments_json {
        Ok(result) => Ok(extract_comments(&result)),
        Err(err) => Err(err),
    }
}
pub async fn get_video_legacy(
    video_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<VideoQuery, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading video with id {}",video_id);
    let player_json = endpoints::player(&video_id, &params, &client_config).await;
    /*
    Error handling
    */
    let res = match player_json {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let res_next = next_with_data(
        serde_json::json!({
            "videoId": video_id,
            "params": params
        }),
        &client_config,
    )
    .await;
    let next_video_data = match res_next {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let video_player = extract_video_player_formats(&res["streamingData"]);
    let video: Video = video_from_next_and_player(
        &res,
        &next_video_data["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"],
        video_player,
    );
    Ok(extract_next_video_results(
        &next_video_data,
        VideoQuery {
            continuation_comments: "".to_string(),
            continuation_related: unwrap_to_string(
                next_video_data["contents"]["twoColumnWatchNextResults"]["secondaryResults"]
                    ["secondaryResults"]["results"][20]["continuationItemRenderer"]
                    ["continuationEndpoint"]["continuationCommand"]["token"]
                    .as_str(),
            ),
            video,
            related_videos: Vec::new(),
        },
    ))
}
pub async fn get_channel_info_legacy(
    channel_id: String,
    client_config: &ClientConfig,
) -> Result<ChannelQuery, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel info for channel: {}", channel_id);
    let complete_url = "https://www.youtube.com/channel/".to_owned() + &channel_id + "/about";
    let resolved_url = resolve_url(&complete_url, &client_config).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let channel_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"]
            .as_str()
            .unwrap(),
        res["endpoint"]["browseEndpoint"]["params"]
            .as_str()
            .unwrap(),
        &client_config,
    )
    .await;
    match channel_json {
        Ok(result) => Ok(ChannelQuery {
            channel: extract_channel_info(&result),
        }),
        Err(err) => return Err(err),
    }
}
pub async fn get_channel_tab_url_legacy(
    channel_id: String,
    tab: Tab,
    client_config: &ClientConfig,
) -> Result<ChannelTab, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel tab: {} for channel: {}", tab.get_name(),channel_id);
    let index = tab.get_index();
    let complete_url = "https://www.youtube.com/".to_owned() + &channel_id + "/about";
    let resolved_url = resolve_url(&complete_url, &client_config).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let channel_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"]
            .as_str()
            .unwrap(),
        res["endpoint"]["browseEndpoint"]["params"]
            .as_str()
            .unwrap(),
        &client_config,
    )
    .await;
    match channel_json {
        Ok(result) => Ok(extract_channel_tab(&result, index)),
        Err(err) => Err(err),
    }
}
pub async fn get_channel_tab_continuation_legacy(
    continuation: String,
    tab: Tab,
    client_config: &ClientConfig,
) -> Result<ChannelTab, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading channel tab: {}, with continuation: {}", tab.get_name(),continuation);
    let index = tab.get_index();
    let channel_json = endpoints::browse_continuation(&continuation, &client_config).await;
    match channel_json {
        Ok(result) => Ok(extract_channel_tab(&result, index)),
        Err(err) => Err(err),
    }
}
pub async fn get_playlist_legacy(
    playlist_id: String,
    client_config: &ClientConfig,
) -> Result<Playlist, RequestError> {
    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Loading playlist with id: {}",playlist_id);
    let complete_url = "https://www.youtube.com/playlist?list=".to_owned() + &playlist_id;
    let resolved_url = resolve_url(&complete_url, &client_config).await;
    let res = match resolved_url {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let playlist_json = browse_browseid(
        res["endpoint"]["browseEndpoint"]["browseId"]
            .as_str()
            .unwrap(),
        "",
        &client_config,
    )
    .await;
    match playlist_json {
        Ok(result) => Ok(extract_playlist(&result)),
        Err(err) => return Err(err),
    }
}

pub async fn next_video_id(
    video_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<NextResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing next request for video id: {} and params: {}",video_id,params);
    let json = next_with_data(
        json!({
            "videoId": video_id,
            "params": params,
        }),
        client_config,
    )
    .await;
    match json {
        Ok(json) => match extract_next_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing next result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, ID: {}, params: {}",err,video_id,params);
                    let mut log = String::from(&format!("---------------------------------------------------- \n Video ID: {} \n params: {} \n Error {} \n----------------------------------------------------", video_id,params, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_next"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn next_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<NextResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing next request for continuation: {}",continuation);
    let json = next(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_next_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing next result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Dumping json for request with error: {}, CToken: {}",err,continuation);
                    let mut log = String::from(&format!("---------------------------------------------------- \n CToken: {} \n Error {} \n----------------------------------------------------", continuation, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_next_ctoken"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}

pub async fn browse_id(
    browse_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing browse request for browse id: {} and params: {}",browse_id, params);
    let json = browse_browseid(&browse_id, &params, client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing browse result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::DEBUG,"Dumping json for request with error: {}, ID: {}, params: {}",err,browse_id,params);
                    let mut log = String::from(&format!("---------------------------------------------------- \n Browse ID: {} \n params: {} \n Error {} \n ----------------------------------------------------", browse_id,params, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_browse"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn browse_custom_data(
    data: Value,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing browse request for browse data: {}",data);
    let json = browse_with_data(data.clone(), client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing browse result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}",err);
                    let mut log = String::from(&format!("---------------------------------------------------- \n Custom Data:\n {} \n Error {} \n ----------------------------------------------------", data.to_string(), err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_browse_custom_data"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn browse_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing browse request for continuation: {}",continuation);
    let json = endpoints::browse_continuation(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing browse result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, CToken: {}",err, continuation);
                    let mut log = String::from(&format!("---------------------------------------------------- \n CToken: {} \n Error {} \n  ----------------------------------------------------", continuation, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_browse_ctoken"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn player(
    video_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<PlayerResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing player request for video id: {} and params: {}",video_id, params);
    let json = endpoints::player(&video_id, &params, client_config).await;
    match json {
        Ok(json) => match extract_player_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing player result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, ID: {}, params: {}",err,video_id,params);
                    let mut log = String::from(&format!("---------------------------------------------------- \n Video ID: {} \n params: {} \n Error {} \n----------------------------------------------------", video_id,params, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_player"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn resolve(url: String, client_config: &ClientConfig) -> Result<ResolveResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing resolve request for url: {}",url);
    let json = endpoints::resolve_url(&url, client_config).await;
    match json {
        Ok(json) => match extract_resolve_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing resolve result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, url: {}",err,url);
                    let mut log = String::from(&format!("---------------------------------------------------- \n URL: {} \n Error {} \n----------------------------------------------------", url, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_resolve"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn search(
    query: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<SearchResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing search request for query id: {} and params: {}",query, params);
    let json = endpoints::search(&query, &params, client_config).await;
    match json {
        Ok(json) => match extract_search_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing next result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, Search query: {}, params: {}",err,query,params);
                    let mut log = String::from(&format!("---------------------------------------------------- \n Search query: {} \n params: {} \n Error {} \n ----------------------------------------------------", query,params, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_search"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
pub async fn search_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<SearchResult, Errors> {
    tracing::event!(target: "youtubei_rs",Level::TRACE,"Preparing search request for continuation: {}",continuation);
    let json = endpoints::search_continuation(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_search_result(&json) {
            Ok(result) => return Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing next result: {}",err);
                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, CToken: {}",err, continuation);
                    let mut log = String::from(&format!("---------------------------------------------------- \n CToken: {} \n Error {} \n----------------------------------------------------", continuation, err));
                    log += &json.to_string();
                    fs::write(format!("json_dump_endpoint_search_ctoken"), log).unwrap();
                }
                return Err(Errors::ParseError(err));
            }
        },
        Err(err) => return Err(Errors::RequestError(err)),
    }
}
