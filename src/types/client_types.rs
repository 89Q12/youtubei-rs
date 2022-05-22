use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ClientType{
   pub  name: String,
   pub  version: String,
   pub  api_key: String,
   pub  screen: String,
 }
 #[derive(Serialize, Deserialize)]
 pub  enum ClientTypes{
    Web,
    WebEmbeddedPlayer,
    WebMobile,
    WebScreenEmbed,
    Android,
    AndroidEmbeddedPlayer,
    AndroidScreenEmbed,
    TvHtml5ScreenEmbed,
}

impl ClientTypes {
 pub fn get_client_type(&self) -> ClientType{
    match self {
        ClientTypes::Web =>  ClientType{
              name:    "WEB".to_string(),
              version: "2.20210721.00.00".to_string(),
              api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
              screen:  "WATCH_FULL_SCREEN".to_string(),
            },
        ClientTypes::WebEmbeddedPlayer => ClientType{      
            name:    "WEB_EMBEDDED_PLAYER".to_string(),
            version: "1.20210721.1.0".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "EMBED".to_string(),
        },
        ClientTypes::WebMobile => ClientType{
            name:    "MWEB".to_string(),
            version: "2.20210726.08.00".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "".to_string(),
        },
        ClientTypes::WebScreenEmbed => ClientType{      
            name:    "WEB".to_string(),
            version: "2.20210721.00.00".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "EMBED".to_string(),
        },
        ClientTypes::Android => ClientType{
            name:    "ANDROID".to_string(),
            version: "16.20".to_string(),
            api_key: "AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w".to_string(),
            screen:  "".to_string(),
        },
        ClientTypes::AndroidEmbeddedPlayer => ClientType{
            name:    "ANDROID_EMBEDDED_PLAYER".to_string(),
            version: "16.20".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "".to_string(),
        },
        ClientTypes::AndroidScreenEmbed => ClientType{
            name:    "ANDROID".to_string(),
            version: "16.20".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "EMBED".to_string(),
        },
        ClientTypes::TvHtml5ScreenEmbed => ClientType{
            name:    "TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string(),
            version: "2.0".to_string(),
            api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
            screen:  "EMBED".to_string(),
        },
    }
 }
}
