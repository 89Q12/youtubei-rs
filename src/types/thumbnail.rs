use serde::Deserialize;

use super::{accessibility::Accessibility, misc::RunsSimpleTextAccessibility};

#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailsAccessibility {
    pub thumbnails: Vec<Thumbnail>,
    pub accessibility: Accessibility,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailWrapper {
    pub thumbnail: Thumbnails,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowCustomThumbnailRenderer {
    pub show_custom_thumbnail_renderer: ThumbnailWrapper,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailUrlOnly {
    pub url: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailUrlOnlyWrapper {
    pub thumbnails: ThumbnailUrlOnly,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailOverlayTimeStatusRenderer {
    pub text: RunsSimpleTextAccessibility,
    pub style: String,
}
