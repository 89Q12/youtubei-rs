use serde_json::{json, Value};
use crate::types::clientConfig::ClientConfig;
use crate::utils::{merge, default_client_config};

pub async fn browse_continuation(continuation : &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
pub async fn browse_browseid(browse_id : &str, params:  &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "browseId"     : browse_id,
    "params"       : params,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
pub async fn next(continuation : &str, client_config: &ClientConfig){
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
pub async fn next_with_data(mut data: serde_json::Value, client_config: &ClientConfig){
  merge(&mut data, &json!({"context": make_context(client_config)}));
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
pub async fn player(video_id: &str, params:  &str,client_config: &ClientConfig){
  let data = json!({
    "videoId" : video_id,
    "context" : make_context(client_config),
    "params"       : params,
  });
  return post_json("/youtubei/v1/player", data, &client_config).await;
}

pub async fn resolve_url(url: &str, client_config: &ClientConfig){
  let data = json!({
    "context" : make_context(&default_client_config()),
    "url"     : url,
  });
  return post_json("/youtubei/v1/resolve_url", data, &client_config).await;
}

pub async fn search(search_query: &str, params:  &str, client_config: &ClientConfig){
  let data = json!({
    "query"   : search_query,
    "context" : make_context(client_config),
    "params"  : params,
  });
  return post_json("/youtubei/v1/search", data, client_config).await;
}

fn make_context(client_config: &ClientConfig) -> serde_json::Value{
  let mut client_context = json!({"client": {
    "hl"           : "en",
    "gl"           : client_config.region(),
    "clientName"   : client_config.name(),
    "clientVersion": client_config.version(),
  }
});
if !client_config.client_type.get_client_type().screen.is_empty() {
  client_context["client"]["clientScreen"] = serde_json::Value::String(client_config.client_type.get_client_type().screen);
}
if client_config.client_type.get_client_type().screen == "EMBED"{
  client_context["third_party"] = json!({"embedUrl":  "https://www.youtube.com/embed/dQw4w9WgXcQ"})
}
return json!({"client": &client_context});
}

async fn post_json(endpoint: &str, data: Value, client_config : &ClientConfig) {
 todo!();
}