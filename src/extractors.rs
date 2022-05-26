use serde_json::Value;
use crate::{types::{endpoints::{EndpointBrowse, EndpointWatch}, query_results::VideoQuery, search_video::SearchVideo, video::Video, video_player::{VideoPlayer, Format}}, utils::is_author_verified};

pub fn video_from_next_and_player(player_video_details: &Value, next_video_details: &Value, video_player: VideoPlayer) -> Video {
    Video { 
        title: player_video_details["title"].to_string(), 
        id: player_video_details["videoId"].to_string(), 
        author: player_video_details["title"].to_string(), 
        ucid: player_video_details["chanelId"].to_string(), 
        published: next_video_details["videoPrimaryInfoRenderer"]["dateText"]["simpleText"].to_string(), 
        views:next_video_details["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]["viewCount"]["simpleText"].to_string(), 
        description_html: player_video_details["shortDescription"].to_string(), 
        length_seconds: player_video_details["lengthSeconds"].as_str().unwrap().parse::<i64>().unwrap(), 
        live_now: player_video_details["isLiveContent"].as_bool().unwrap(), 
        premiere_timestamp: "".to_string(), 
        author_verified: is_author_verified(&next_video_details["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["badges"]), 
        video_player
    }
}
pub fn extract_video_player_formats(json: &Value) -> VideoPlayer {
    let mut video_player =VideoPlayer {
        formats: Vec::new(),
        apdaptiveformts: Vec::new(),
    };
    for i  in 0..json["streamingData"]["formats"].as_array().unwrap().len() {
        video_player.formats.push(Format{
            itag: json["streamingData"]["formats"][i]["itag"].to_string(),
            url: json["streamingData"]["formats"][i]["url"].to_string(),
            mime_type: json["streamingData"]["formats"][i]["mimeType"].to_string(),
            bitrate: json["streamingData"]["formats"][i]["bitrate"].as_i64().unwrap(),
            quality: json["streamingData"]["formats"][i]["quality"].to_string(),
            fps: json["streamingData"]["formats"][i]["fps"].as_i64().unwrap(),
            quality_label: json["streamingData"]["formats"][i]["qualityLabel"].to_string(),
            audio_quality: json["streamingData"]["formats"][i]["audioQuality"].to_string(),
        })
    }
    for i  in 0..json["streamingData"]["adaptiveFormats"].as_array().unwrap().len() {
        video_player.apdaptiveformts.push(Format{
            itag: json["streamingData"]["adaptiveFormats"][i]["itag"].to_string(),
            url: json["streamingData"]["adaptiveFormats"][i]["url"].to_string(),
            mime_type: json["streamingData"]["adaptiveFormats"][i]["mimeType"].to_string(),
            bitrate: json["streamingData"]["adaptiveFormats"][i]["bitrate"].as_i64().unwrap(),
            quality: json["streamingData"]["adaptiveFormats"][i]["quality"].to_string(),
            fps: match json["streamingData"]["adaptiveFormats"][i]["fps"].as_i64(){
                Some(num) =>num,
                None => 0,
            },
            quality_label: json["streamingData"]["adaptiveFormats"][i]["qualityLabel"].to_string(),
            audio_quality: json["streamingData"]["adaptiveFormats"][i]["audioQuality"].to_string(),
        })
    }
    return video_player;

}
pub fn extract_next_video_results(json: &Value,mut  video_query: VideoQuery) -> VideoQuery{
    let continuation = json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][3]["itemSectionRenderer"]["contents"][0]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].as_str().unwrap();
    for value in json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array().unwrap(){
        video_query.related_videos.push(compact_video_renderer(&value["compactVideoRenderer"]))
    }
    video_query.continuation_comments = continuation.to_string();
    return video_query;
}

fn compact_video_renderer(video: &Value)-> SearchVideo{
    return SearchVideo{ 
        title: video["title"]["simpleText"].to_string(), 
        id: video["videoId"].to_string(), 
        channel_name: video["longBylineText"]["runs"]["text"].to_string(), 
        author: video["longBylineText"]["runs"]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].to_string(), 
        author_verified: is_author_verified(&video["ownerBadges"]), 
        channel_thumbnail: video["channelThumbnail"]["thumbnails"][0]["url"].to_string(), 
        view_count_text: video["viewCountText"]["simpleText"].to_string(), 
        length_text: video["lengthText"]["simpleText"].to_string(), 
        endpoint: EndpointWatch{
            url: video["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].to_string(),
            video_id:video["videoId"].to_string() ,
            playlist_id: "".to_string(),
            params: "".to_string(),
        }, 
        browse_channel: EndpointBrowse { url: video["longBylineText"]["runs"]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].to_string(), browse_id: video["longBylineText"]["runs"]["navigationEndpoint"]["browseEndpoint"]["browseId"].to_string(), params: "".to_string() }, 
        thumbnail: video["thumbnail"]["thumbnails"][0]["url"].to_string()
    }
}