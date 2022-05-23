use super::video::Video;

pub struct Channel{
    pub name: String,
    pub id: String,
    pub banner: String,
    pub videos: Vec<Video>,
}