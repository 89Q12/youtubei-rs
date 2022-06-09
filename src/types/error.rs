use std::{error::Error, fmt::Display};
use reqwest::StatusCode;
use serde_json::Value;

/// will be returned when an unrecoverable error is encountered while parsing
#[derive(Debug)]
pub struct  ParseError{
    pub message: String,
    pub to_parse_type: String,
}
impl  Error for ParseError{}
impl  Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse into {}, with message: {}", self.to_parse_type,self.message)
    }
}
/// Will be returned when an error while requesting data from the api
#[derive(Debug)]
pub struct RequestError{
    pub message: String,
    pub status: StatusCode,
    pub endpoint: String,
    pub request_data: Value,
}
impl  Error for RequestError{}
impl  Display for RequestError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on endpoint {} with status {} and request data {}", self.message, self.endpoint, self.status, self.request_data)
    }
}
#[derive(Debug)]
pub enum Errors {
    RequestError(RequestError),
    ParseError(ParseError)
}