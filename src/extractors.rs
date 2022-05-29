use serde_json::Value;
use crate::{types::{endpoints::{EndpointBrowse, EndpointWatch}, query_results::{VideoQuery, SearchQuery,SearchResult}, video::{Video,SearchVideo,ChannelVideo,VideoPlayer, Format}, channel::{Channel,ChannelTab, SearchChannel,CommunityPost,TabTypes::*,TabTypes}, playlist::SearchPlaylist}, utils::{is_author_verified, unwrap_to_string, unwrap_to_i64, is_auto_generated}};
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
    video_query.continuation_comments  = extract_continuation_token( &json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][3]["itemSectionRenderer"]["contents"][0]["continuationItemRenderer"]);
    for value in 0..json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"].as_array().unwrap().len()-1{
        video_query.related_videos.push(compact_video_renderer(&json["contents"]["twoColumnWatchNextResults"]["secondaryResults"]["secondaryResults"]["results"][value]["compactVideoRenderer"]))
    }
    return video_query;
}
pub fn load_related(json: &Value) -> Vec<SearchVideo> {
    let mut videos: Vec<SearchVideo> = Vec::new();
    let content =  &json["onResponseReceivedEndpoints"][0]["appendContinuationItemsAction"]["continuationItems"];
    for i in 0..content.as_array().unwrap().len()-1 {
        videos.push(compact_video_renderer(&content[i]["compactVideoRenderer"]))
    }
    return videos;
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
        endpoint: extract_watch_endpoint(&video["navigationEndpoint"]),
        browse_channel: extract_browse_endpoint(&video["longBylineText"]["runs"][0]["navigationEndpoint"]), 
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
pub fn extract_channel_info(json: &Value) -> Channel{
    let name = unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["title"].as_str());
    Channel{ 
        name: name.to_owned(),
        id: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["channelId"].as_str()),
        banner: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["banner"]["thumbnails"][0]["url"].as_str()),
        avatar: unwrap_to_string(json["header"]["c4TabbedHeaderRenderer"]["avatar"]["thumbnails"][0]["url"].as_str()),
        description: unwrap_to_string( json["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][5]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"][0]["channelAboutFullMetadataRenderer"]["description"]["simpleText"].as_str())
    }
}
pub fn get_channel_tab(json: &Value, index: usize) -> ChannelTab{
    // get the channel name from the metadata
    let channel_name = json["metadata"]["channelMetadataRenderer"]["title"].as_str().unwrap();
    // title is  always at this location
    let title = unwrap_to_string( json["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][index]["tabRenderer"]["title"].as_str());
    // placeholder values
    let mut continuation =String::from("");
    let mut content:Vec<TabTypes> = Vec::new();
    let items;
    if json["onResponseReceivedActions"].is_null() && !json["contents"].is_null(){
        items = &json["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][index]["tabRenderer"]["content"]["sectionListRenderer"]["contents"];
    }
    else{
        items = &json["onResponseReceivedActions"][0]["appendContinuationItemsAction"]["continuationItems"];
    }
    for i in 0..items.as_array().unwrap().len(){
        // itemSectionRenderer: {contents[{}]} and because we have only key we can just use the last method to get the key
        match items[i]["itemSectionRenderer"]["contents"][0].as_object().unwrap().keys().last().unwrap().as_str(){
            "shelfRenderer" => for renderer in items[i]["itemSectionRenderer"]["contents"][0]["shelfRenderer"]["content"]["horizontalListRenderer"]["items"].as_array().unwrap(){
                match renderer.as_object().unwrap().keys().last().unwrap().as_str() {
                    "gridPlaylistRenderer" => content.push(Playlists(grid_playlist_renderer(&renderer["gridPlaylistRenderer"],channel_name))),
                    "gridVideoRenderer"=> content.push(Videos(grid_video_renderer(&renderer["gridVideoRenderer"], channel_name))),
                    // I havent seen it here but theoretically it could be here so we should have it as arm
                    "continuationItemRenderer" => continuation= extract_continuation_token(&renderer["continuationItemRenderer"]),
                    _ => break
                }
            },
            "backstagePostThreadRenderer" => content.push(Community(backstage_post_thread_renderer(&items[i]["backstagePostThreadRenderer"],channel_name))),
            // Since its sometimes inside a gridRenderer object we need to iterate through the items and match against gridPlaylistRenderer and gridVideoRenderer
            "gridRenderer" => for renderer in items[i]["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"].as_array().unwrap(){
                match renderer.as_object().unwrap().keys().last().unwrap().as_str() {
                    "gridPlaylistRenderer" => content.push(Playlists(grid_playlist_renderer(&renderer["gridPlaylistRenderer"],channel_name))),
                    "gridVideoRenderer"=> content.push(Videos(grid_video_renderer(&renderer["gridVideoRenderer"], channel_name))),
                    "continuationItemRenderer" => continuation= extract_continuation_token(&renderer["continuationItemRenderer"]),
                    _ => break
                }
            },
            "gridPlaylistRenderer" => content.push(Playlists(grid_playlist_renderer(&items[i]["itemSectionRenderer"]["contents"][0]["gridPlaylistRenderer"],channel_name))),
            "gridVideoRenderer"=> content.push(Videos(grid_video_renderer(&items[i]["itemSectionRenderer"]["contents"][0]["gridVideoRenderer"], channel_name))),
            "continuationItemRenderer" => continuation= extract_continuation_token(&items[i]["itemSectionRenderer"]["contents"][0]["continuationItemRenderer"]),
            _ => break

        }
    }
    ChannelTab{
        title,
        selected: true,
        content,
        continuation,
    }
}
fn backstage_post_thread_renderer(item: &Value, name: &str) -> CommunityPost {
    CommunityPost{
            content_text: unwrap_to_string(item["contentText"].as_str()),
            content_attachment: unwrap_to_string(item["backstageAttachment"]["backstageImageRenderer"]["image"]["thumbnails"][0]["url"].as_str()) ,
            author_name: name.to_owned(),
            author_thumbnail:  unwrap_to_string(item["authorThumbnail"]["thumbnails"][0]["url"].as_str()),
            vote_count:  unwrap_to_i64(item["voteCount"]["simpleText"].as_i64()),
            published_time_text:  unwrap_to_string(item["publishedTimeText"]["runs"][0]["text"].as_str()),
            browse_endpoint: EndpointBrowse{
                url: unwrap_to_string(item["authorEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str()),
                browse_id:  unwrap_to_string(item["authorEndpoint"]["browseEndpoint"]["browseId"].as_str()),
                params: String::from(""),
            }
        }
}

fn grid_playlist_renderer(playlist: &Value, name: &str) -> SearchPlaylist{
    SearchPlaylist{
        title: unwrap_to_string(playlist["title"]["runs"][0]["text"].as_str()),
        id:  unwrap_to_string(playlist["playlistId"].as_str()),
        author: name.to_string(),
        ucid: String::from(""),
        video_count: unwrap_to_i64( playlist["videoCountShortText"]["simpleText"].as_i64()),
        thumbnail: unwrap_to_string(playlist["thumbnail"]["thumbnails"][0]["url"].as_str()),
        author_verified: is_author_verified(&playlist["ownerBadges"][0]),
        play_endpoint: extract_watch_endpoint(&playlist["navigationEndpoint"]),
        browse_endpoint:extract_browse_endpoint(&playlist["viewPlaylistText"]["runs"][0]["navigationEndpoint"])
    }

}
fn grid_video_renderer(video: &Value, channel_name:&str) -> ChannelVideo{
        ChannelVideo{
            title:  unwrap_to_string(video["title"]["runs"][0]["text"].as_str()), 
            id: unwrap_to_string(video["videoId"].as_str()), 
            published_text:  unwrap_to_string(video["publishedTimeText"]["simpleText"].as_str()), 
            author: channel_name.to_string(), 
            author_verified: is_author_verified(&video["ownerBadges"][0]), 
            thumbnail: unwrap_to_string(video["thumbnail"]["thumbnails"][0]["url"].as_str()),
            view_count_text:  unwrap_to_string(video["viewCountText"]["simpleText"].as_str()), 
            length_text:  unwrap_to_string(video["thumbnailOverlays"][0]["thumbnailOverlayTimeStatusRenderer"]["text"]["simpleText"].as_str()),
            channel_thumbnail: String::from(""), 
            endpoint: extract_watch_endpoint(&video["navigationEndpoint"]),
        }
}
/*
endregion channel_extraction
*/
/*
region search_extraction
*/
pub fn extract_search_results(json: &Value, continuation: bool)-> SearchQuery{
    let content;
    if continuation {
        content = &json["onResponseReceivedCommands"]["appendContinuationItemsAction"]["continuationItems"];
    }
    else {
        content = &json["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]["sectionListRenderer"]["contents"];
    }
    let mut search_query = SearchQuery{
        continuation: extract_continuation_token(&content[1]["continuationItemRenderer"]),
        results: Vec::new(),
    };
    for renderer in content[0]["itemSectionRenderer"]["contents"].as_array().unwrap().iter() {
        for (key, item) in renderer.as_object().unwrap() {
            match key.as_str(){
                "videoRenderer" => search_query.results.push(SearchResult::VideoRenderer(video_renderer(&item))),
                "channelRenderer" => search_query.results.push(SearchResult::SearchChannel(channel_renderer(&item))),
                "playlistRenderer" => search_query.results.push(SearchResult::PlaylistRenderer(playlist_renderer(&item))),
                "shelfRenderer" => for i  in 0..item["content"]["verticalListRenderer"]["items"].as_array().unwrap().len(){
                    search_query.results.push(SearchResult::VideoRenderer(video_renderer(&item["content"]["verticalListRenderer"]["items"][i]["videoRenderer"])));
                }
                _ => break
            }
        }
    }
    return search_query;
}

fn channel_renderer(channel_renderer:&Value) -> SearchChannel{
   return SearchChannel{
    author: unwrap_to_string(channel_renderer["title"]["simpleText"].as_str()),
    ucid: unwrap_to_string(channel_renderer["channelId"].as_str()),
    author_thumbnail: unwrap_to_string(channel_renderer["channelId"].as_str()),
    subscriber_count: unwrap_to_string(channel_renderer["subscriberCountText"]["simpleText"].as_str()),
    video_count:  channel_renderer["videoCountText"]["runs"][0]["text"].to_string()+ " videos",
    description_html: unwrap_to_string(channel_renderer["descriptionSnippet"]["runs"][0]["text"].as_str()),
    auto_generated: is_auto_generated(unwrap_to_string(channel_renderer["title"]["simpleText"].as_str())),
    author_verified: is_author_verified(&channel_renderer["ownerBadges"][0]),
    endpoint:extract_browse_endpoint(&channel_renderer["navigationEndpoint"]),
}
}

fn video_renderer(video_renderer:&Value) -> SearchVideo{
    return SearchVideo{ 
        title: unwrap_to_string(video_renderer["title"]["runs"][0]["text"].as_str()),  
        id: unwrap_to_string(video_renderer["videoId"].as_str()), 
        channel_name: unwrap_to_string(video_renderer["longBylineText"]["runs"][0]["text"].as_str()), 
        author: unwrap_to_string(video_renderer["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].as_str()), 
        author_verified: is_author_verified(&video_renderer["ownerBadges"][0]), 
        channel_thumbnail: unwrap_to_string(video_renderer["channelThumbnail"]["thumbnails"][0]["url"].as_str()), 
        view_count_text: unwrap_to_string(video_renderer["viewCountText"]["simpleText"].as_str()), 
        length_text: unwrap_to_string(video_renderer["lengthText"]["simpleText"].as_str()), 
        endpoint: extract_watch_endpoint(&video_renderer["navigationEndpoint"]),
        browse_channel: extract_browse_endpoint(&video_renderer["longBylineText"]["runs"][0]["navigationEndpoint"]), 
        thumbnail: unwrap_to_string(video_renderer["thumbnail"]["thumbnails"][0]["url"].as_str()),
        published_text: unwrap_to_string(video_renderer["publishedTimeText"]["simpleText"].as_str()),
    }
}
fn playlist_renderer(playlist_renderer:&Value) -> SearchPlaylist{
    return SearchPlaylist{
        title: unwrap_to_string(playlist_renderer["title"]["simpleText"].as_str()),
        id:  unwrap_to_string(playlist_renderer["playlistId"].as_str()),
        author: unwrap_to_string(playlist_renderer["shortBylineText"]["runs"][0]["text"].as_str()),
        ucid: unwrap_to_string(playlist_renderer["shortBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()),
        video_count: unwrap_to_i64(playlist_renderer["videoCountText"]["runs"][0]["text"].as_i64()),
        thumbnail: unwrap_to_string(playlist_renderer["playlistId"].as_str()),
        author_verified: is_author_verified(&playlist_renderer["ownerBadges"][0]),
        play_endpoint: extract_watch_endpoint(&playlist_renderer["navigationEndpoint"]),
        browse_endpoint: extract_browse_endpoint(&playlist_renderer["navigationEndpoint"]),
    }
}
/*
endregion search_extraction
*/
/*
region helper functions
*/
fn extract_continuation_token(continuation_item_render: &Value) -> String{
    return unwrap_to_string(continuation_item_render["continuationEndpoint"]["continuationCommand"]["token"].as_str());
}
fn extract_browse_endpoint(navigation_endpoint: &Value) -> EndpointBrowse{
    EndpointBrowse { 
        url: unwrap_to_string(navigation_endpoint["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()), 
        browse_id:unwrap_to_string(navigation_endpoint["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()), 
        params: String::from(""),
    }
}
fn extract_watch_endpoint(navigation_endpoint: &Value) -> EndpointWatch{
    EndpointWatch { 
        url: unwrap_to_string(navigation_endpoint["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]["url"].as_str()), 
        video_id: unwrap_to_string(navigation_endpoint["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str()),
        playlist_id: unwrap_to_string(navigation_endpoint["navigationEndpoint"]["watchEndpoint"]["playlistId"].as_str()),
        params: unwrap_to_string(navigation_endpoint["navigationEndpoint"]["watchEndpoint"]["params"].as_str()),
    }
}