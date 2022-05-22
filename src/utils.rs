use serde_json::Value;
use crate::types::clientConfig::ClientConfig;
use crate::types::clientTypes;

pub fn default_client_config() -> ClientConfig {
    ClientConfig::new(clientTypes::ClientTypes::Web,"US".to_string(),"US".to_string())
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