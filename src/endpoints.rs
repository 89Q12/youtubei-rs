use serde_json::{json, Value};
use crate::types::client_config::ClientConfig;
use crate::utils::{merge, default_client_config};

pub async fn browse_continuation(continuation : &str, client_config : &ClientConfig) -> Value{
  
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
pub async fn browse_browseid(browse_id : &str, params:  &str, client_config : &ClientConfig) -> Value{
  
  let data = json!({
    "context"      : make_context(&client_config),
    "browseId"     : browse_id,
    "params"       : params,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
pub async fn next(continuation : &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
pub async fn next_with_data(mut data: serde_json::Value, client_config: &ClientConfig) -> Value{
  merge(&mut data, &json!({"context": make_context(client_config)}));
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
pub async fn player(video_id: &str, params:  &str,client_config: &ClientConfig) -> Value{
  let data = json!({
    "videoId" : video_id,
    "context" : make_context(client_config),
    "params"       : params,
  });
  return post_json("/youtubei/v1/player", data, &client_config).await;
}

pub async fn resolve_url(url: &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context" : make_context(&default_client_config()),
    "url"     : url,
  });
  return post_json("/youtubei/v1/navigation/resolve_url", data, &client_config).await;
}

pub async fn search(search_query: &str, params:  &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "query"   : search_query,
    "context" : make_context(&client_config),
    "params"  : params,
  });
  return post_json("/youtubei/v1/search", data, client_config).await;
}
pub async fn search_continuation(continuation : &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/search", data, &client_config).await;
}
fn make_context(client_config: &ClientConfig) -> serde_json::Value{
  let mut client_context = json!({
      "hl"           : "en",
      "gl"           : client_config.region(),
      "clientName"   : client_config.name(),
      "clientVersion": client_config.version(),
  });
if !client_config.client_type.get_client_type().screen.is_empty() {
  client_context["clientScreen"] = serde_json::Value::String(client_config.client_type.get_client_type().screen);
}
if client_config.client_type.get_client_type().screen == "EMBED"{
  client_context["third_party"] = json!({"embedUrl":  "https://www.youtube.com/embed/dQw4w9WgXcQ"})
}
return json!({"client": &client_context});
}

async fn post_json(endpoint: &str, data: Value, client_config : &ClientConfig) -> Value {
  let url = endpoint.to_owned()+"?key="+ &client_config.client_type.get_client_type().api_key.to_owned() +"&prettyPrint=false";
  let wrapped_response=  client_config.http_client
  .post("https://www.youtube.com".to_owned() +&url)
  .json(&data )
  .send().await;
  let response = {
    match wrapped_response{
      Ok(t) => t,
      Err(e) => return json!({"error": e.to_string()}),
    }
  };
  let json: Value={ 
    match response.json().await{
      Ok(json) => json,
      Err(e) => return json!({"error": e.to_string()}),
  }};
  return json;
}