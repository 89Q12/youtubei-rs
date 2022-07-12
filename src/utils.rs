use serde_json::Value;
use crate::types::client::ClientConfig;
use crate::types::misc::MetadataBadgeRenderer;
use crate::types::{client, self};
use crate::types::query_results::NextResult;

pub fn default_client_config() -> ClientConfig {
    ClientConfig::new(client::ClientTypes::Web,"US".to_string(),"US".to_string(), true)
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

/// Utility function to get likes for a video
pub fn get_likes(video: &NextResult) -> String {
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .get(0)
        .unwrap()
    {
        types::misc::NextContents::VideoPrimaryInfoRenderer(vpir) => match vpir
            .video_actions
            .menu_renderer
            .as_ref()
            .unwrap()
            .top_level_buttons
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
        {
            types::misc::TopLevelButtons::ButtonRenderer(_) => unreachable!(),
            types::misc::TopLevelButtons::ToggleButtonRenderer(btn) => btn
                .default_text
                .accessibility
                .as_ref()
                .unwrap()
                .accessibility_data
                .label
                .clone(),
        },
        _ => unreachable!(),
    }
}
/// Utility function to build the description for a video
pub fn get_description(video: &NextResult) -> String {
    let mut desc = String::from("");
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .get(1)
        .unwrap()
    {
        types::misc::NextContents::VideoSecondaryInfoRenderer(vsir) => {
            for text in vsir.description.runs.iter() {
                desc.push_str(text.text.clone().as_str());
            }
        }
        _ => unreachable!(),
    }
    desc
}
/// Utility function to check if the author is verified it takes the first BadgeRenderer
pub fn get_author_verified(video: &MetadataBadgeRenderer) -> bool {
    match video.icon.as_ref().unwrap().icon_type.as_str() {
        "OFFICIAL_ARTIST_BADGE" => true,
        "CHECK_CIRCLE_THICK" => true,
        _ => false,
    }
}
/// Utility function to get the subscriber count
pub fn get_subcribe_count(video: &NextResult) -> String {
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .get(1)
        .unwrap()
    {
        types::misc::NextContents::VideoSecondaryInfoRenderer(vsir) => {
            match &vsir.owner.video_owner_renderer.subscriber_count_text {
                Some(text) => text.simple_text.clone(),
                None => String::from(""),
            }
        }
        _ => unreachable!(),
    }
}
/// Utility function to get the thumbnail of the video owner
pub fn get_owner_thumbnail(video: &NextResult) -> String {
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .get(1)
        .unwrap()
    {
        types::misc::NextContents::VideoSecondaryInfoRenderer(vsir) => vsir
            .owner
            .video_owner_renderer
            .thumbnail
            .thumbnails
            .get(0)
            .unwrap()
            .url
            .clone(),
        _ => unreachable!(),
    }
}
/// Utility function to  the continuation of the video comments
pub fn get_continuation_comments(video: &NextResult) -> Option<String> {
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .last()
        .unwrap()
    {
        types::misc::NextContents::ContinuationItemRenderer(cir) => {
            match &cir.continuation_endpoint.continuation_command {
                Some(cmd) => Some(cmd.token.clone()),
                None => None,
            }
        }
        _ => None,
    }
}
/// Utility function to  the continuation of related videos 
pub fn get_continuation_related(video: &NextResult) -> Option<String> {
    match video
        .contents
        .as_ref()
        .unwrap()
        .two_column_watch_next_results
        .as_ref()
        .unwrap()
        .results
        .results
        .contents
        .last()
        .unwrap()
    {
        types::misc::NextContents::ContinuationItemRenderer(cir) => {
            match &cir.continuation_endpoint.continuation_command {
                Some(cmd) => Some(cmd.token.clone()),
                None => None,
            }
        }
        _ => None,
    }
}