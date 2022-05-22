use serde_json::{json, Value};
use crate::types::clientConfig::ClientConfig;
use crate::utils::merge;

pub fn browse_continuation(continuation : &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/browse", data, client_config);
}
pub fn browse_browseid(browse_id : &str, params:  &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "browseId"     : browse_id,
    "params"       : params,
  });
  return post_json("/youtubei/v1/browse", data, client_config);
}
pub fn next(continuation : &str, client_config: &ClientConfig){
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/next", data, &client_config);
}
pub fn next_with_data(mut data: serde_json::Value, client_config: &ClientConfig){
  merge(&mut data, &json!({"context": make_context(client_config)}));
  return post_json("/youtubei/v1/next", data, &client_config);
}
pub fn player(video_id: &str, params:  &str,client_config: &ClientConfig){
  let data = json!({
    "videoId" : video_id,
    "context" : make_context(client_config),
    "params"       : params,
  });
  return post_json("/youtubei/v1/player", data, &client_config);
}



fn post_json(endpint: &str, data: Value, client_config : &ClientConfig) {
    todo!()
}

fn make_context(client_config: &ClientConfig) -> serde_json::Value{
  let  client_context = json!({"client": {
    "hl"           : "en",
    "gl"           : client_config.region(),
    "clientName"   : client_config.name(),
    "clientVersion": client_config.version(),}});
  return json!({"client": &client_context});
}