use serde_json::{json, Value};
use crate::types::clientConfig::ClientConfig;


fn browse_continuation(continuation : &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "continuation" : continuation,
  });
  return post_json("/youtubei/v1/browse", data, &client_config);
}
fn browse_browseid(browse_id : &str, params:  &str, client_config : &ClientConfig){
  
  let data = json!({
    "context"      : make_context(&client_config),
    "browseId"     : browse_id,
    "params"       : params,
  });
  return post_json("/youtubei/v1/browse", data, &client_config);
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