/// contains functions to generate the data to make an API call.
/// private because they return raw data and or errors
pub(crate) mod endpoints;
/// Contains all the structs used to represent the data returned by the API
pub mod types;
/// Contains utilities for various tasks
pub mod utils;
/// private because they are intended to be called by query functions only
pub(crate) mod extractors;
/// Contains the main functions of this library
/// Refer to the tests for usage information
pub mod query;
/// Contains test functions
mod tests{
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[cfg(test)]
    use crate::{query::{get_comments, load_related_videos, get_playlist,get_video},utils::default_client_config};
    use crate::{extractors, types::{channel::*, video::VideoPrimaryInfoRenderer}};


#[tokio::test]
async fn fetch_video_legacy() {
  let client_config = &default_client_config();
  // gC6dQrScmHE is the id of an LTT video, used here because LTT doesn't delete videos or make them private
  let video_query = get_video("gC6dQrScmHE".to_string(),"".to_string(),&client_config).await;
  // Checks that there is indeed a video
  assert_eq!(video_query.is_ok(), true);
  // unwrap video
  let video = &video_query.as_ref().unwrap().video;
  let comment_token = &video_query.as_ref().unwrap().continuation_comments;

  // Asser that the video has a title
  assert_eq!(video.title, String::from("Running a YouTube Business is EASY (just kidding)"));
  // Assert that are is a token to fetch_comments
  assert_eq!(comment_token.is_empty(), false);
}
#[tokio::test]
async fn fetch_comments_legacy() {
  let client_config = &default_client_config();
  // Comments for video gC6dQrScmHE
  let comment_query = get_comments( "Eg0SC2dDNmRRclNjbUhFGAYyVSIuIgtnQzZkUXJTY21IRTAAeAKqAhpVZ3d1eFlpV0dWYlV2SVRVdUZSNEFhQUJBZzABQiFlbmdhZ2VtZW50LXBhbmVsLWNvbW1lbnRzLXNlY3Rpb24%3D".to_string(),&client_config).await;
  // Checks that there are comments
  assert_eq!(comment_query.is_ok(), true);
  // unwrap into comments
  let comments = &comment_query.as_ref().unwrap().comments;
  let ctoken = &comment_query.as_ref().unwrap().continuation;
  // Assert that there are comments in the vector
  assert_ne!(comments.len(),0);
  // Assert that there is a ctoken
  assert_eq!(ctoken.is_empty(),false);
}

#[tokio::test]
async fn fetch_related_legacy() {
  let client_config = &default_client_config();
    // Related videos for video gC6dQrScmHE
  let related_query = load_related_videos("CBQSDRILZ0M2ZFFyU2NtSEUYACqIBjJzNkw2d3lfQkFxOEJBb0Q4ajRBQ2c3Q1Bnc0k1X2o1cGJENTNxZk9BUW9EOGo0QUNnM0NQZ29JMHZ1Y29wYlczT292Q2dQeVBnQUtEc0ktQ3dpdnRkTzJydV9uaGZ3QkNnUHlQZ0FLRGNJLUNnakR5TDZzZ19taXVWOEtBX0ktQUFvT3dqNExDTVBTeWEycm9McnltQUVLQV9JLUFBb053ajRLQ0w3RWpNM3NpcFdaWmdvRDhqNEFDZzNDUGdvSWtlZnoxOVR1eC1ZdUNnUHlQZ0FLRGNJLUNnam13XzJidFpHSnhrb0tBX0ktQUFvT3dqNExDTjZZa2EzVHZkNmQtd0VLQV9JLUFBb093ajRMQ0p6Rm1mM3VvdURwa1FFS0FfSS1BQW9Od2o0S0NMZXNvTGFieU9tU01Bb0Q4ajRBQ2czQ1Bnb0kySUxCMDZ6N3R0d05DZ1B5UGdBS0RjSS1DZ2lSdHBtN3U5RHduaGtLQV9JLUFBb093ajRMQ0lYem00eTZ5c2lFbndFS0FfSS1BQW9Pd2o0TENJMkN3LWFnc05yS3BRRUtBX0ktQUFvTndqNEtDUG41azRUUDJicTdiQW9EOGo0QUNnN0NQZ3NJdHEyR3A4R1E0dlBMQVFvRDhqNEFDZzNDUGdvSTg4eVRrLVR4N3MwMkNnUHlQZ0FLRGNJLUNnalIxTm14X01UWWpVb0tBX0ktQUFvTndqNEtDSmU1ME15OTRwV0NUaElVQUFJRUJnZ0tEQTRRRWhRV0dCb2NIaUFpSkNZYUJBZ0FFQUVhQkFnQ0VBTWFCQWdFRUFVYUJBZ0dFQWNhQkFnSUVBa2FCQWdLRUFzYUJBZ01FQTBhQkFnT0VBOGFCQWdRRUJFYUJBZ1NFQk1hQkFnVUVCVWFCQWdXRUJjYUJBZ1lFQmthQkFnYUVCc2FCQWdjRUIwYUJBZ2VFQjhhQkFnZ0VDRWFCQWdpRUNNYUJBZ2tFQ1VhQkFnbUVDY3FGQUFDQkFZSUNnd09FQklVRmhnYUhCNGdJaVFtag93YXRjaC1uZXh0LWZlZWR4AQ%3D%3D".to_string(),&client_config).await;
  // Checks that there are related videos
  assert_eq!(related_query.is_ok(), true);
  // unwrap into comments
  let related = related_query.unwrap();
  // Assert that there are 20 videos in the vector
  assert_eq!(related.len(),20);
}
#[tokio::test]
async fn fetch_playlist_legacy() {
  let client_config = &default_client_config();
  // PLe8jmEHFkvsbeJL2QNucGv00eO8PKbSUn is a log playlist from the channel Monstercat Uncaged
  let playlist_query = get_playlist("PLe8jmEHFkvsbeJL2QNucGv00eO8PKbSUn".to_string(),&client_config).await;
  // Checks that there are related videos
  assert_eq!(playlist_query.is_ok(), true);
  // unwrap into comments
  let playlist = playlist_query.unwrap();
  // Assert that there is the correct number of videos
  assert_eq!(playlist.video_count,"314 videos");
  // Assert that the videos vector has a len of 100
  assert_eq!(playlist.videos.len(), 100);
  
}

}