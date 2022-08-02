use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
/// Represents the client that is used in requests to the api.
/// For example the following represent the web client:
/// Note that gl and hl are added a later point as they depend on other information
/// ```not_rust
///
///  "client": {
///  "clientName": "WEB",
///  "clientScreen": "WATCH_FULL_SCREEN",
///  "clientVersion": "2.20210721.00.00",
///   "gl": "US",
///   "hl": "en"
/// },
/// ```
#[derive(Serialize, Deserialize)]
pub struct ClientType {
    pub name: String,
    pub version: String,
    pub api_key: String,
    pub screen: String,
}
#[derive(Serialize, Deserialize)]
pub enum ClientTypes {
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
    /// Returns the client type based on the enum value
    /// ```
    ///  use youtubei_rs::types::client::ClientTypes;
    ///  let client = ClientTypes::Web;
    ///  assert_eq!(client.get_client_type().name, "WEB");
    /// ```
    pub fn get_client_type(&self) -> ClientType {
        match self {
            ClientTypes::Web => ClientType {
                name: "WEB".to_string(),
                version: "2.20220714.01.00".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "WATCH_FULL_SCREEN".to_string(),
            },
            ClientTypes::WebEmbeddedPlayer => ClientType {
                name: "WEB_EMBEDDED_PLAYER".to_string(),
                version: "1.20210721.1.0".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "EMBED".to_string(),
            },
            ClientTypes::WebMobile => ClientType {
                name: "MWEB".to_string(),
                version: "2.20220714.01.00".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "".to_string(),
            },
            ClientTypes::WebScreenEmbed => ClientType {
                name: "WEB".to_string(),
                version: "2.20220714.01.00".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "EMBED".to_string(),
            },
            ClientTypes::Android => ClientType {
                name: "ANDROID".to_string(),
                version: "16.20".to_string(),
                api_key: "AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w".to_string(),
                screen: "".to_string(),
            },
            ClientTypes::AndroidEmbeddedPlayer => ClientType {
                name: "ANDROID_EMBEDDED_PLAYER".to_string(),
                version: "16.20".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "".to_string(),
            },
            ClientTypes::AndroidScreenEmbed => ClientType {
                name: "ANDROID".to_string(),
                version: "16.20".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "EMBED".to_string(),
            },
            ClientTypes::TvHtml5ScreenEmbed => ClientType {
                name: "TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string(),
                version: "2.0".to_string(),
                api_key: "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string(),
                screen: "EMBED".to_string(),
            },
        }
    }
}
/// Represents the client_config used by the endpoint functions
/// to determine gl and hl params in the request context.
/// Will later be used to choose a appropriate proxy.
pub struct ClientConfig {
    pub client_type: ClientTypes,
    pub region: String,
    pub proxy_region: String,
    pub http_client: Client,
    pub dump_on_error: bool,
}

impl ClientConfig {
    /// Constructs a new ClientConfig with the client and all required headers
    pub fn new(
        client_type: ClientTypes,
        region: String,
        proxy_region: String,
        dump_on_error: bool,
    ) -> Self {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json; charset=UTF-8"),
        );
        headers.insert(
            "Accept-Encoding",
            header::HeaderValue::from_static("gzip")
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        headers.insert(
            "accept-charset",
            header::HeaderValue::from_static("ISO-8859-1,utf-8;q=0.7,*;q=0.7"),
        );
        headers.insert(
            "accept-language",
            header::HeaderValue::from_static("en-us,en;q=0.5"),
        );
        headers.insert(
            "x-youtube-client-name",
            header::HeaderValue::from_static("1"),
        );
        headers.insert(
            "x-youtube-client-version",
            header::HeaderValue::from_static("2.20200609"),
        );
        headers.insert(
            "cookie", header::HeaderValue::from_static("CONSENT=YES+")
        );

        let _http_client = reqwest::ClientBuilder::new()
            .https_only(true)
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36")
            .default_headers(headers)
            .gzip(true)
            .build().unwrap();

        Self {
            client_type,
            region,
            proxy_region,
            http_client: _http_client,
            dump_on_error,
        }
    }

    pub fn name(&self) -> String {
        self.client_type.get_client_type().name
    }

    pub fn version(&self) -> String {
        self.client_type.get_client_type().version
    }

    pub fn region(&self) -> &str {
        &self.region
    }
}
