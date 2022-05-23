use super::{video::{Video}, search_channel::{SearchChannel}, search_playlist::SearchPlaylist};

pub enum CategoryTypes{
    Video(Video),
    SearchChannel(SearchChannel),
    SearchPlaylists(SearchPlaylist)

}
pub struct Category{
   pub  title: String,
   pub  contents: Vec<CategoryTypes>,
   pub  description_html: String,
   pub  url: String,
   pub  badges: String,
}

impl Category {

}