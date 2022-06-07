use serde_json::Value;
use crate::types::client::ClientConfig;
use crate::types::client;

pub fn default_client_config() -> ClientConfig {
    ClientConfig::new(client::ClientTypes::Web,"US".to_string(),"US".to_string())
}
/// Used to merge 2 values into one, probably could be optimized
pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (Value::Array(ref mut a), &Value::Object(ref b)) => {
            a.extend([Value::Object(b.clone())]);
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}
/// Utility function to check if a channel is verified
/// Takes &Value with index "ownerBadges`\[`0`\]`" or "badges"
/// returns true if channel is verified else false
pub fn is_author_verified(json: &Value) -> bool {
    let badge = &json["metadataBadgeRenderer"]["tooltip"];
    return badge.is_string() && badge.as_str().unwrap().to_lowercase() == "verified";
}

pub fn is_auto_generated(author: String) -> bool{
    return author.ends_with(" - Topic") || ["Popular on YouTube", "Music", "Sports", "Gaming"].contains(&author.as_str());
}

pub fn unwrap_to_string(input: Option<&str>) -> String{
    match input{
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

pub fn unwrap_to_i64(input:  Option<i64>) -> i64{
    match input{
        Some(n) => n,
        None => 0,
    }
}