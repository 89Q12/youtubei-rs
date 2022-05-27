use serde_json::Value;
use crate::types::client_config::ClientConfig;
use crate::types::client_types;

pub fn default_client_config() -> ClientConfig {
    ClientConfig::new(client_types::ClientTypes::Web,"US".to_string(),"US".to_string())
}
// Used to merge 2 values into one, probably could be optimized
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
pub fn is_author_verified(json: &Value) -> bool {
    let badge = &json["metadataBadgeRenderer"]["tooltip"];
    if badge.is_string(){
        if badge.as_str().unwrap() =="Verified"{
           return true;
        }
    }
    return false;
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