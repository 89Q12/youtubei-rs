use reqwest::header;
use reqwest::Client;
use super::clientTypes::ClientTypes;
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
