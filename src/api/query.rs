use std::fs;

use serde_json::{json, Value};
use tracing::Level;

use crate::api::endpoints;
use crate::api::endpoints::*;
use crate::api::extractors::*;

use crate::client::ClientConfig;
use crate::models::error::Errors;

use crate::models::query_results::{
    BrowseResult, NextResult, PlayerResult, ResolveResult, SearchResult,
};

pub async fn next_video_id(
    video_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<NextResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing next request for video id: {} and params: {}", video_id, params);

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
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing next result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::ERROR, "Dumping json for request with error: {}, ID: {}, Params: {}", err, video_id, params);
                    let mut log = String::from(&format!(
                        "------------------------\n Video ID: {}\n Params: {}\n Error: {} \n------------------------",
                        video_id, params, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_next", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn next_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<NextResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing next request for continuation: {}", continuation);

    let json = next(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_next_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing next result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::DEBUG, "Dumping json for request with error: {}, CToken: {}", err, continuation);

                    let mut log = String::from(&format!(
                        "------------------------\n CToken: {}\n Error {} \n------------------------",
                        continuation, err
                    ));
                    log += &json.to_string();
                    fs::write("json_dump_endpoint_next_ctoken", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn browse_id(
    browse_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing browse request for browse ID: {} and params: {}", browse_id, params);

    let json = browse_browseid(&browse_id, &params, client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing browse result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::DEBUG, "Dumping json for request with error: {}, ID: {}, params: {}", err, browse_id, params);

                    let mut log = String::from(&format!(
                        "------------------------\nBrowse ID: {}\n Params: {}\n Error {}\n ------------------------",
                        browse_id,params, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_browse", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn browse_custom_data(
    data: Value,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing browse request for browse data: {}", data);

    let json = browse_with_data(data.clone(), client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing browse result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::ERROR,"Dumping json for request with error: {}", err);

                    let mut log = String::from(&format!(
                        "------------------------\n Custom Data: {}\n Error {} \n------------------------",
                        data, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_browse_custom_data", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn browse_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<BrowseResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing browse request for continuation: {}", continuation);

    let json = endpoints::browse_continuation(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_browse_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing browse result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::ERROR, "Dumping json for request with error: {}, CToken: {}", err, continuation);

                    let mut log = String::from(&format!(
                        "------------------------\nCToken: {}\n Error {}\n------------------------",
                        continuation, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_browse_ctoken", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn player(
    video_id: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<PlayerResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing player request for video id: {} and params: {}", video_id, params);

    let json = endpoints::player(&video_id, &params, client_config).await;
    match json {
        Ok(json) => match extract_player_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing player result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, ID: {}, params: {}", err, video_id, params);

                    let mut log = String::from(&format!(
                        "------------------------\n Video ID: {}\n params: {}\n Error {}\n------------------------",
                        video_id, params, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_player", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn resolve(url: String, client_config: &ClientConfig) -> Result<ResolveResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing resolve request for url: {}", url);

    let json = endpoints::resolve_url(&url, client_config).await;
    match json {
        Ok(json) => match extract_resolve_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs",Level::ERROR,"Error parsing resolve result: {}",err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs",Level::ERROR,"Dumping json for request with error: {}, url: {}",err,url);
                    let mut log = String::from(&format!(
                        "------------------------ \n URL: {} \nError {} \n------------------------",
                        url, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_resolve", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn search(
    query: String,
    params: String,
    client_config: &ClientConfig,
) -> Result<SearchResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing search request for query id: {} and params: {}", query, params);

    let json = endpoints::search(&query, &params, client_config).await;
    match json {
        Ok(json) => match extract_search_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing next result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::ERROR, "Dumping json for request with error: {}, Search query: {}, params: {}", err, query, params);

                    let mut log = String::from(&format!(
                        "------------------------\n Search query: {} \n Params: {} \n Error {} \n------------------------",
                        query, params, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_search", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}

pub async fn search_continuation(
    continuation: String,
    client_config: &ClientConfig,
) -> Result<SearchResult, Errors> {
    tracing::event!(target: "youtubei_rs", Level::TRACE, "Preparing search request for continuation: {}", continuation);

    let json = endpoints::search_continuation(&continuation, client_config).await;
    match json {
        Ok(json) => match extract_search_result(&json) {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::event!(target: "youtubei_rs", Level::ERROR, "Error parsing next result: {}", err);

                if client_config.dump_on_error {
                    tracing::event!(target: "youtubei_rs", Level::ERROR,"Dumping json for request with error: {}, CToken: {}", err, continuation);

                    let mut log = String::from(&format!(
                        "------------------------\n CToken: {} \n Error {}\n------------------------",
                        continuation, err
                    ));

                    log += &json.to_string();
                    fs::write("json_dump_endpoint_search_ctoken", log).unwrap();
                }

                Err(Errors::ParseError(err))
            }
        },
        Err(err) => Err(Errors::RequestError(err)),
    }
}
