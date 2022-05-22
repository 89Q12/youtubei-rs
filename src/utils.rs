use crate::types::clientConfig::ClientConfig;
use crate::types::clientTypes;

pub fn default_client_config() -> ClientConfig {
    ClientConfig::new(clientTypes::ClientTypes::Web,"US".to_string(),"US".to_string())
}