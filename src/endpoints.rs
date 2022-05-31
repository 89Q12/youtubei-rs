use serde_json::{json, Value};
use crate::types::client::ClientConfig;
use crate::utils::merge;
/// Prepears the data and makes a post request to the browse endpoint
pub(crate) async fn browse_continuation(continuation : &str, client_config : &ClientConfig) -> Value{
  
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
/// Prepears the data and makes a post request to the browse endpoint
pub(crate)  async fn browse_browseid(browse_id : &str, params:  &str, client_config : &ClientConfig) -> Value{
  
  let data = json!({
    "context"      : make_context(&client_config),
    "browseId"     : browse_id,
    "params"       : params,
  });
  return post_json("/youtubei/v1/browse", data, client_config).await;
}
/// Prepears the data and makes a post request to the next endpoint
pub(crate)  async fn next(continuation : &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
/// Prepears the data and makes a post request to the next endpoint
pub(crate)  async fn next_with_data(mut data: serde_json::Value, client_config: &ClientConfig) -> Value{
  merge(&mut data, &json!({"context": make_context(client_config)}));
  return post_json("/youtubei/v1/next", data, &client_config).await;
}
/// Prepears the data and makes a post request to the player endpoint
pub(crate)  async fn player(video_id: &str, params:  &str,client_config: &ClientConfig) -> Value{
  let data = json!({
    "videoId" : video_id,
    "context" : make_context(client_config),
    "params"       : params,
  });
  return post_json("/youtubei/v1/player", data, &client_config).await;
}
/// Prepears the data and makes a post request to the resolve_url endpoint,
/// which returns another endpoint to query
pub(crate)  async fn resolve_url(url: &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context" : make_context(&client_config),
    "url"     : url,
  });
  return post_json("/youtubei/v1/navigation/resolve_url", data, &client_config).await;
}
/// Prepears the data and makes a post request to the search endpoint
pub(crate)  async fn search(search_query: &str, params:  &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "query"   : search_query,
    "context" : make_context(&client_config),
    "params"  : params,
  });
  return post_json("/youtubei/v1/search", data, client_config).await;
}
/// Prepears the data and makes a post request to the search endpoint
pub(crate)  async fn search_continuation(continuation : &str, client_config: &ClientConfig) -> Value{
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/search", data, &client_config).await;
}
/// takes the client_config and makes a json object that is used as context for the api call.
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
/// Makes a post request to the given endpoint with the given data and parameters
/// On failure, returns 
/// ```json
/// {"error": message}
/// ```
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