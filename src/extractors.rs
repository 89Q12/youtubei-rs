use serde_json::Value;
use crate::{types::{endpoints::{EndpointBrowse, EndpointWatch}, query_results::VideoQuery, search_video::SearchVideo, video::Video, video_player::{VideoPlayer, Format}, channel::Channel, tab::ChannelTab, channel_video::ChannelVideo, category::CategoryTypes::*, search_playlist::SearchPlaylist}, utils::{is_author_verified, unwrap_to_string, unwrap_to_i64}};
/*
region video_extraction
*/
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
    for value in 0..json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array().unwrap().len()-1{
        video_query.related_videos.push(compact_video_renderer(&json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"][value]["compactVideoRenderer"]))
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
        thumbnail: unwrap_to_string(video["thumbnail"]["thumbnails"][0]["url"].as_str()),
        published_text: unwrap_to_string(video["publishedTimeText"]["simpleText"].as_str()),
    }
}
/*
endregion video_extraction
*/
/*
region channel_extraction
*/
pub fn extract_channel(json: &Value, tab: &str) -> Channel{
    let name = unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["title"].as_str());
    Channel{ 
        name: name.to_owned(),
        id: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["channelId"].as_str()),
        banner: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["banner"]["thumbnails"][0]["url"].as_str()),
        avatar: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["avatar"]["thumbnails"][0]["url"].as_str()),
        tab: match tab {
            "videos"=> extract_videos_tab(&json["contents"]["twoColumnBrowseResultsRenderer"],&name),
            "playlists" => extract_playlists_tab(&json["contents"]["twoColumnBrowseResultsRenderer"], &name),
            _ => panic!("Unrecognized Tab: {}", tab)
        }
    }
}
/// Playlist are either in a gridRenderer or multiple shelfRenderer.
/// Which makes parsing a bit more complicated
fn extract_playlists_tab(json: &Value, name: &str) -> ChannelTab {
    let mut tab = ChannelTab{ 
        title: unwrap_to_string(json["tabs"][2]["tabRenderer"]["title"].as_str()), 
        selected: true, 
        content: Vec::with_capacity(29), // Youtube provides always 30 items, -1 due to the continuation token. 
        continuation: String::from(""),
    };
    // Should never be None
    let items = &json["tabs"][2]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"].as_array().unwrap();
    // go through the items and match against gridRenderer and shelfRenderer
    for item in items.iter() {
        for (section, value) in item.as_object().unwrap(){
            match  section.as_str(){
                "gridRenderer" => {
                    tab.continuation = unwrap_to_string(value["items"][30]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].as_str());
                    grid_playlist_renderer(&mut tab, &value, name)
                },
                "shelfRenderer" => {
                    // In the case we don't have a continuation token for whatever reason
                    tab.continuation = String::from("");
                    grid_playlist_renderer(&mut tab, &value["content"]["horizontalListRenderer"], name)
                },
                _ => println!("Lol")
            };
        }
    }
    return tab;
}
fn grid_playlist_renderer(tab: &mut ChannelTab, value: &Value, name: &str){
    for i in 0..value["items"].as_array().unwrap().len()-1{
        let playlist = &value["items"][i]["gridPlaylistRenderer"];
        tab.content.push(SearchPlaylists(
            SearchPlaylist{
                title: unwrap_to_string(playlist["title"]["runs"][0]["text"].as_str()),
                id:  unwrap_to_string(playlist["playlistId"].as_str()),
                author: name.to_string(),
                ucid: String::from(""),
                video_count: unwrap_to_i64(playlist["videoCountShortText"]["simpleText"].as_i64()),
                videos: Vec::with_capacity(0),
                thumbnail: unwrap_to_string(playlist["thumbnail"]["thumbnails"][0]["url"].as_str()),
                author_verified: is_author_verified(&playlist["ownerBadges"]),
                play_endpoint: EndpointWatch{
                    url: unwrap_to_string(playlist["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()),
                    video_id: unwrap_to_string(playlist["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str()),
                    playlist_id: unwrap_to_string(playlist["navigationEndpoint"]["watchEndpoint"]["playlistId"].as_str()),
                    params: unwrap_to_string(playlist["navigationEndpoint"]["watchEndpoint"]["params"].as_str()),
                },
                browse_endpoint: EndpointBrowse { 
                    url: unwrap_to_string(playlist["viewPlaylistText"]["runs"][0]["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()), 
                    browse_id:unwrap_to_string(playlist["viewPlaylistText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()), 
                    params: String::from(""),
                },
            }
        ))
    }
}
fn extract_videos_tab(json: &Value, channel_name:&str) -> ChannelTab{
    let mut tab = ChannelTab{ 
        title: unwrap_to_string(json["tabs"][1]["tabRenderer"]["title"].as_str()), 
        selected: true, 
        content: Vec::with_capacity(29), // Youtube provides always 30 items, -1 due to the continuation token. 
        continuation: unwrap_to_string(json["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"][30]["continuationItemRenderer"]["continuationEndpoint"]["continuationCommand"]["token"].as_str())
    };
    let items = &json["tabs"][1]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"];;
    for item in 0..items.as_array().unwrap().len(){
        tab.content.push(Video( ChannelVideo{
            title:  unwrap_to_string(items[item]["gridVideoRenderer"]["title"]["runs"][0]["text"].as_str()), 
            id: unwrap_to_string(items[item]["gridVideoRenderer"]["videoId"].as_str()), 
            published_text:  unwrap_to_string(items[item]["gridVideoRenderer"]["publishedTimeText"]["simpleText"].as_str()), 
            author: channel_name.to_string(), 
            author_verified: is_author_verified(&items[item]["gridVideoRenderer"]["ownerBadges"]), 
            thumbnail: unwrap_to_string(items[item]["gridVideoRenderer"]["thumbnail"]["thumbnails"][0]["url"].as_str()),
            view_count_text:  unwrap_to_string(items[item]["gridVideoRenderer"]["viewCountText"]["simpleText"].as_str()), 
            length_text:  unwrap_to_string(items[item]["gridVideoRenderer"]["thumbnailOverlays"][0]["thumbnailOverlayTimeStatusRenderer"]["text"]["simpleText"].as_str()),
            channel_thumbnail: String::from(""), 
            endpoint: EndpointWatch{
                url: unwrap_to_string(items[item]["gridVideoRenderer"]["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()),
                video_id: unwrap_to_string(items[item]["gridVideoRenderer"]["videoId"].as_str()),
                playlist_id: String::from(""),
                params: String::from(""),
            },
        }));
    }
    return tab;
}