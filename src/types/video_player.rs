pub struct VideoPlayer{
    pub formats: Vec<Format>,
    pub apdaptiveformts: Vec<Format>,
}

pub struct Format{
    pub itag: String,
    pub url: String,
    pub mime_type: String,
    pub bitrate: i64,
    pub quality: String,
    pub fps: i64,
    pub quality_label: String,
    pub audio_quality: String,
}