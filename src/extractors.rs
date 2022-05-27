use serde_json::Value;
use crate::{types::{endpoints::{EndpointBrowse, EndpointWatch}, query_results::VideoQuery, search_video::SearchVideo, video::Video, video_player::{VideoPlayer, Format}}, utils::{is_author_verified, unwrap_to_string, unwrap_to_i64}};

pub fn video_from_next_and_player(player_video_details: &Value, next_video_details: &Value, video_player: VideoPlayer) -> Video {
    Video { 
        title: unwrap_to_string(player_video_details["title"].as_str()), 
        id: unwrap_to_string(player_video_details["videoId"].as_str()), 
        author: unwrap_to_string(player_video_details["author"].as_str()), 
        ucid: unwrap_to_string(player_video_details["channelId"].as_str()), 
        published: unwrap_to_string(next_video_details[0]["videoPrimaryInfoRenderer"]["dateText"]["simpleText"].as_str()), 
        views:unwrap_to_string(next_video_details[0]["videoPrimaryInfoRenderer"]["viewCount"]["videoViewCountRenderer"]["viewCount"]["simpleText"].as_str()), 
        description_html: unwrap_to_string(player_video_details["shortDescription"].as_str()), 
        length_seconds: unwrap_to_i64( player_video_details["lengthSeconds"].as_i64()), 
        live_now: player_video_details["isLiveContent"].as_bool().unwrap(), 
        premiere_timestamp: "".to_string(), 
        author_verified: is_author_verified(&next_video_details[1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["badges"][0]), 
        video_player,
        thumbnail: unwrap_to_string(player_video_details["thumbnail"]["thumbnails"][0]["url"].as_str()),
        channel_thumbnail: unwrap_to_string(next_video_details[1]["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"]["thumbnail"]["thumbnails"][0]["url"].as_str())
    }
}
pub fn extract_video_player_formats(json: &Value) -> VideoPlayer {
    let mut video_player =VideoPlayer {
        formats: Vec::new(),
        apdaptiveformts: Vec::new(),
    };
    for i  in 0..json["formats"].as_array().unwrap().len() {
        video_player.formats.push(Format{
            itag: unwrap_to_i64(json["formats"][i]["itag"].as_i64()),
            url: unwrap_to_string(json["formats"][i]["url"].as_str()),
            mime_type: json["formats"][i]["mimeType"].to_string(),
            bitrate: unwrap_to_i64(json["formats"][i]["bitrate"].as_i64()),
            quality:unwrap_to_string( json["formats"][i]["quality"].as_str()),
            fps: unwrap_to_i64(json["formats"][i]["fps"].as_i64()),
            quality_label: unwrap_to_string( json["formats"][i]["qualityLabel"].as_str()),
            audio_quality :unwrap_to_string( json["formats"][i]["audioQuality"].as_str()),
        });
    }
    for i  in 0..json["adaptiveFormats"].as_array().unwrap().len() {
        video_player.apdaptiveformts.push(Format{
            itag: unwrap_to_i64(json["adaptiveFormats"][i]["itag"].as_i64()),
            url: unwrap_to_string(json["adaptiveFormats"][i]["url"].as_str()),
            mime_type: json["adaptiveFormats"][i]["mimeType"].to_string(),
            bitrate: unwrap_to_i64(json["adaptiveFormats"][i]["bitrate"].as_i64()),
            quality:unwrap_to_string( json["adaptiveFormats"][i]["quality"].as_str()),
            fps: unwrap_to_i64(json["adaptiveFormats"][i]["fps"].as_i64()),
            quality_label: unwrap_to_string( json["adaptiveFormats"][i]["qualityLabel"].as_str()),
            audio_quality :unwrap_to_string( json["adaptiveFormats"][i]["audioQuality"].as_str()),
        })
    }
    return video_player;

}
pub fn extract_next_video_results(json: &Value,mut  video_query: VideoQuery) -> VideoQuery{
    video_query.continuation_comments  = unwrap_to_string( json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][3]["itemSectionRenderer"]["contents"][0]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].as_str());
    for value in json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array().unwrap(){
        video_query.related_videos.push(compact_video_renderer(&value["compactVideoRenderer"]))
    }
    return video_query;
}

fn compact_video_renderer(video: &Value)-> SearchVideo{
    return SearchVideo{ 
        title: unwrap_to_string(video["title"]["simpleText"].as_str()), 
        id: unwrap_to_string(video["videoId"].as_str()), 
        channel_name: unwrap_to_string(video["longBylineText"]["runs"][0]["text"].as_str()), 
        author: unwrap_to_string(video["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str()), 
        author_verified: is_author_verified(&video["ownerBadges"][0]), 
        channel_thumbnail: unwrap_to_string(video["channelThumbnail"]["thumbnails"][0]["url"].as_str()), 
        view_count_text: unwrap_to_string(video["viewCountText"]["simpleText"].as_str()), 
        length_text: unwrap_to_string(video["lengthText"]["simpleText"].as_str()), 
        endpoint: EndpointWatch{
            url: unwrap_to_string(video["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()),
            video_id:unwrap_to_string(video["videoId"].as_str()),
            playlist_id: "".to_string(),
            params: "".to_string(),
        }, 
        browse_channel: EndpointBrowse { url: unwrap_to_string(video["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str()), browse_id: unwrap_to_string(video["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()), params: "".to_string() }, 
        thumbnail: unwrap_to_string(video["thumbnail"]["thumbnails"][0]["url"].as_str())
    }
}