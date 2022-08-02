use crate::types::{
    error::ParseError,
    query_results::{BrowseResult, NextResult, PlayerResult, ResolveResult, SearchResult},
};
use serde_json::Value;

pub fn extract_next_result(json: &Value) -> Result<NextResult, ParseError> {
    match serde_json::from_value::<NextResult>(json.to_owned()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ParseError {
            message: err.to_string(),
            to_parse_type: String::from("NextResult"),
        }),
    }
}
pub fn extract_search_result(json: &Value) -> Result<SearchResult, ParseError> {
    match serde_json::from_value::<SearchResult>(json.to_owned()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ParseError {
            message: err.to_string(),
            to_parse_type: String::from("SearchResult"),
        }),
    }
}
pub fn extract_player_result(json: &Value) -> Result<PlayerResult, ParseError> {
    match serde_json::from_value::<PlayerResult>(json.to_owned()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ParseError {
            message: err.to_string(),
            to_parse_type: String::from("PlayerResult"),
        }),
    }
}
pub fn extract_browse_result(json: &Value) -> Result<BrowseResult, ParseError> {
    match serde_json::from_value::<BrowseResult>(json.to_owned()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ParseError {
            message: err.to_string(),
            to_parse_type: String::from("BrowseResult"),
        }),
    }
}
pub fn extract_resolve_result(json: &Value) -> Result<ResolveResult, ParseError> {
    match serde_json::from_value::<ResolveResult>(json.to_owned()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ParseError {
            message: err.to_string(),
            to_parse_type: String::from("ResolveResult"),
        }),
    }
}
