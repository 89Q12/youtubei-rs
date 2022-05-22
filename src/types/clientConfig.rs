use super::clientTypes::ClientTypes;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ClientConfig{
   pub client_type: ClientTypes,
   pub region: String,
   pub proxy_region:String,
}

impl ClientConfig {
    pub fn new(client_type: ClientTypes, region: String, proxy_region: String) -> Self { Self { client_type, region, proxy_region } }

    pub fn name(&self) ->  String {
       self.client_type.get_client_type().name
    }
    pub fn version(&self) ->  String {
      self.client_type.get_client_type().version
    }
    pub fn region(&self) -> &str{
        return &self.region
    }
}