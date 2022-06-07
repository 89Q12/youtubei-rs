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
#[cfg(test)]
    use serde_json::json;

    use crate::{query::{get_comments, load_related_videos, get_playlist,get_video},utils::default_client_config, types::{video::VideoRenderer, query_results::{NextResult, BrowseResult, PlayerResult, SearchResult}}, endpoints};
    use crate::{extractors, types::{channel::*, video::{VideoPrimaryInfoRenderer, VideoSecondaryInfoRenderer, CompactVideoRenderer}}};


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
#[tokio::test]
async fn test_channel_renderer(){
  let j = json!({
  "channelId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
  "title": {
    "simpleText": "Linus Tech Tips"
  },
  "navigationEndpoint": {
    "clickTrackingParams": "CJoFENowGAAiEwjF08Tq1Jb4AhWOglUKHQsmCTA=",
    "commandMetadata": {
      "webCommandMetadata": {
        "url": "/c/LinusTechTips",
        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
        "rootVe": 3611,
        "apiUrl": "/youtubei/v1/browse"
      }
    },
    "browseEndpoint": {
      "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
      "canonicalBaseUrl": "/c/LinusTechTips"
    }
  },
  "thumbnail": {
    "thumbnails": [
      {
        "url": "//yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s88-c-k-c0x00ffffff-no-rj-mo",
        "width": 88,
        "height": 88
      },
      {
        "url": "//yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s176-c-k-c0x00ffffff-no-rj-mo",
        "width": 176,
        "height": 176
      }
    ]
  },
  "descriptionSnippet": {
    "runs": [
      {
        "text": "Looking for a Tech YouTuber? Linus Tech Tips is a passionate team of \"professionally curious\" experts in consumer technology ..."
      }
    ]
  },
  "shortBylineText": {
    "runs": [
      {
        "text": "Linus Tech Tips",
        "navigationEndpoint": {
          "clickTrackingParams": "CJoFENowGAAiEwjF08Tq1Jb4AhWOglUKHQsmCTA=",
          "commandMetadata": {
            "webCommandMetadata": {
              "url": "/c/LinusTechTips",
              "webPageType": "WEB_PAGE_TYPE_CHANNEL",
              "rootVe": 3611,
              "apiUrl": "/youtubei/v1/browse"
            }
          },
          "browseEndpoint": {
            "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "canonicalBaseUrl": "/c/LinusTechTips"
          }
        }
      }
    ]
  },
  "videoCountText": {
    "runs": [
      {
        "text": "5,776"
      },
      {
        "text": " videos"
      }
    ]
  },
  "subscriptionButton": {
    "subscribed": false
  },
  "ownerBadges": [
    {
      "metadataBadgeRenderer": {
        "icon": {
          "iconType": "CHECK_CIRCLE_THICK"
        },
        "style": "BADGE_STYLE_TYPE_VERIFIED",
        "tooltip": "Verified",
        "trackingParams": "CJoFENowGAAiEwjF08Tq1Jb4AhWOglUKHQsmCTA=",
        "accessibilityData": {
          "label": "Verified"
        }
      }
    }
  ],
  "subscriberCountText": {
    "accessibility": {
      "accessibilityData": {
        "label": "14.6 million subscribers"
      }
    },
    "simpleText": "14.6M subscribers"
  },
  "subscribeButton": {
    "buttonRenderer": {
      "style": "STYLE_DESTRUCTIVE",
      "size": "SIZE_DEFAULT",
      "isDisabled": false,
      "text": {
        "runs": [
          {
            "text": "Subscribe"
          }
        ]
      },
      "navigationEndpoint": {
        "clickTrackingParams": "CJsFEPBbIhMIxdPE6tSW-AIVjoJVCh0LJgkw",
        "commandMetadata": {
          "webCommandMetadata": {
            "url": "https://accounts.google.com/ServiceLogin?service=youtube&uilel=3&passive=true&continue=https%3A%2F%2Fwww.youtube.com%2Fsignin%3Faction_handle_signin%3Dtrue%26app%3Ddesktop%26hl%3Den%26next%3D%252Fresults%253Fsearch_query%253Dltt%252Bchannel%26continue_action%3DQUFFLUhqa2gxeFVlVU52ajhOYWZnbG1FTWFBRlliT3NXd3xBQ3Jtc0tuWTc2NUp0ZWUyWlV0d2ZwbkNyTVpyaktZbThnOVhDMTNNVElxSWs1d0NkM2tDQU5DNV9pQTZ1S3Vxclc3TlpkUFdwYjJYZVJUazNTb2ZMSV8tTUVqSzc5cXhIT3ZiMEhTMlRMdDJNNTQ5a3JSUHV2eWFYcGFWRHlkbWYyMGNCV19GOXY2UTNJMWZIRWVQYUtsclFQaHJKTDd3SHlOTExCVm9VN0pFWWdyeVM3SHkzQUR1N0FxakFpbl8xYlI5ZG01UmRHczRHWF9Ub19KZzlVcG1aVFUyaGhKcENB&hl=en",
            "webPageType": "WEB_PAGE_TYPE_UNKNOWN",
            "rootVe": 83769
          }
        },
        "signInEndpoint": {
          "nextEndpoint": {
            "clickTrackingParams": "CJsFEPBbIhMIxdPE6tSW-AIVjoJVCh0LJgkw",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/results?search_query=ltt+channel",
                "webPageType": "WEB_PAGE_TYPE_SEARCH",
                "rootVe": 4724
              }
            },
            "searchEndpoint": {
              "query": "ltt channel"
            }
          },
          "continueAction": "QUFFLUhqa2gxeFVlVU52ajhOYWZnbG1FTWFBRlliT3NXd3xBQ3Jtc0tuWTc2NUp0ZWUyWlV0d2ZwbkNyTVpyaktZbThnOVhDMTNNVElxSWs1d0NkM2tDQU5DNV9pQTZ1S3Vxclc3TlpkUFdwYjJYZVJUazNTb2ZMSV8tTUVqSzc5cXhIT3ZiMEhTMlRMdDJNNTQ5a3JSUHV2eWFYcGFWRHlkbWYyMGNCV19GOXY2UTNJMWZIRWVQYUtsclFQaHJKTDd3SHlOTExCVm9VN0pFWWdyeVM3SHkzQUR1N0FxakFpbl8xYlI5ZG01UmRHczRHWF9Ub19KZzlVcG1aVFUyaGhKcENB"
        }
      },
      "trackingParams": "CJsFEPBbIhMIxdPE6tSW-AIVjoJVCh0LJgkw"
    }
  },
  "trackingParams": "CJoFENowGAAiEwjF08Tq1Jb4AhWOglUKHQsmCTA=",
  "longBylineText": {
    "runs": [
      {
        "text": "Linus Tech Tips",
        "navigationEndpoint": {
          "clickTrackingParams": "CJoFENowGAAiEwjF08Tq1Jb4AhWOglUKHQsmCTA=",
          "commandMetadata": {
            "webCommandMetadata": {
              "url": "/c/LinusTechTips",
              "webPageType": "WEB_PAGE_TYPE_CHANNEL",
              "rootVe": 3611,
              "apiUrl": "/youtubei/v1/browse"
            }
          },
          "browseEndpoint": {
            "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "canonicalBaseUrl": "/c/LinusTechTips"
          }
        }
      }
    ]
  }
  });
  let u: ChannelRenderer = serde_json::from_value(j).unwrap();
  assert_eq!(u.title.simple_text, "Linus Tech Tips");
  assert_eq!(u.channel_id, "UCXuqSBlHAE6Xw-yeJA0Tunw")
}

#[tokio::test]
async fn test_video_primary_info_renderer(){
  let j: serde_json::Value = json!({
      "title": {
        "runs": [
          {
            "text": "The Lab is a Disaster - WAN Show June 3, 2022"
          }
        ]
      },
      "viewCount": {
        "videoViewCountRenderer": {
          "viewCount": {
            "simpleText": "647,476 views"
          },
          "shortViewCount": {
            "simpleText": "647K views"
          }
        }
      },
      "videoActions": {

      },
      "trackingParams": "CI8DEMyrARgAIhMIjcHEuoyZ-AIVS9QRCB1pbQIX",
      "dateText": {
        "simpleText": "Streamed live on Jun 3, 2022"
      }
   });
  let u: VideoPrimaryInfoRenderer = serde_json::from_value(j).unwrap();
  assert_eq!(u.title.runs.get(0).unwrap().text,"The Lab is a Disaster - WAN Show June 3, 2022");
}

#[tokio::test]
async fn test_video_secondary_info_renderer(){
  // The json is shortened due to macro limits, 
  let j: serde_json::Value = json!({
    
      "owner": {
        "videoOwnerRenderer": {
          "thumbnail": {
            "thumbnails": [
              {
                "url": "https://yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s48-c-k-c0x00ffffff-no-rj",
                "width": 48,
                "height": 48
              },
              {
                "url": "https://yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s88-c-k-c0x00ffffff-no-rj",
                "width": 88,
                "height": 88
              },
              {
                "url": "https://yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s176-c-k-c0x00ffffff-no-rj",
                "width": 176,
                "height": 176
              }
            ]
          },
          "title": {
            "runs": [
              {
                "text": "Linus Tech Tips",
                "navigationEndpoint": {
                  "clickTrackingParams": "CI4DEOE5IhMInqXs06eb-AIV5swRCB3YEwCp",
                  "commandMetadata": {
                    "webCommandMetadata": {
                      "url": "/c/LinusTechTips",
                      "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                      "rootVe": 3611,
                      "apiUrl": "/youtubei/v1/browse"
                    }
                  },
                  "browseEndpoint": {
                    "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
                    "canonicalBaseUrl": "/c/LinusTechTips"
                  }
                }
              }
            ]
          },
          "subscriptionButton": {
            "type": "FREE"
          },
          "navigationEndpoint": {
            "clickTrackingParams": "CI4DEOE5IhMInqXs06eb-AIV5swRCB3YEwCp",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/c/LinusTechTips",
                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                "rootVe": 3611,
                "apiUrl": "/youtubei/v1/browse"
              }
            },
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/c/LinusTechTips"
            }
          },
          "subscriberCountText": {
            "accessibility": {
              "accessibilityData": {
                "label": "14.6 million subscribers"
              }
            },
            "simpleText": "14.6M subscribers"
          },
          "trackingParams": "CI4DEOE5IhMInqXs06eb-AIV5swRCB3YEwCp",
          "badges": [
            {
              "metadataBadgeRenderer": {
                "icon": {
                  "iconType": "CHECK_CIRCLE_THICK"
                },
                "style": "BADGE_STYLE_TYPE_VERIFIED",
                "tooltip": "Verified",
                "trackingParams": "CI4DEOE5IhMInqXs06eb-AIV5swRCB3YEwCp",
                "accessibilityData": {
                  "label": "Verified"
                }
              }
            }
          ],
          "membershipButton": {
            "buttonRenderer": {
              "style": "STYLE_SUGGESTIVE",
              "size": "SIZE_DEFAULT",
              "isDisabled": false,
              "text": {
                "runs": [
                  {
                    "text": "Join"
                  }
                ]
              },
              "navigationEndpoint": {
                "clickTrackingParams": "CI8DEKhgIhMInqXs06eb-AIV5swRCB3YEwCp",
                "commandMetadata": {
                  "webCommandMetadata": {
                    "ignoreNavigation": true
                  }
                },
                "modalEndpoint": {
                  "modal": {
                    "modalWithTitleAndButtonRenderer": {
                      "title": {
                        "runs": [
                          {
                            "text": "Want to join this channel?"
                          }
                        ]
                      },
                      "content": {
                        "runs": [
                          {
                            "text": "Sign in to become a member."
                          }
                        ]
                      },
                      "button": {
                        "buttonRenderer": {
                          "style": "STYLE_BRAND",
                          "size": "SIZE_DEFAULT",
                          "isDisabled": false,
                          "text": {
                            "simpleText": "Sign in"
                          },
                          "navigationEndpoint": {
                            "clickTrackingParams": "CJADEPBbIhMInqXs06eb-AIV5swRCB3YEwCp",
                            "commandMetadata": {
                              "webCommandMetadata": {
                                "url": "https://accounts.google.com/ServiceLogin?service=youtube&uilel=3&passive=true&continue=https%3A%2F%2Fwww.youtube.com%2Fsignin%3Faction_handle_signin%3Dtrue%26app%3Ddesktop%26hl%3Den%26next%3Dhttps%253A%252F%252Fwww.youtube.com%252Fyoutubei%252Fv1%252Fnext%253Fkey%253DAIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8&hl=en",
                                "webPageType": "WEB_PAGE_TYPE_UNKNOWN",
                                "rootVe": 83769
                              }
                            },
                            "signInEndpoint": {
                              "hack": true
                            }
                          },
                          "trackingParams": "CJADEPBbIhMInqXs06eb-AIV5swRCB3YEwCp"
                        }
                      }
                    }
                  }
                }
              },
              "trackingParams": "CI8DEKhgIhMInqXs06eb-AIV5swRCB3YEwCp",
              "accessibilityData": {
                "accessibilityData": {
                  "label": "Join this channel"
                }
              },
              "targetId": "sponsorships-button"
            }
          }
        }
      },
      "description": {
        "runs": [
          {
            "text": "Visit "
          },
          {
            "text": "https://www.squarespace.com/WAN",
            "navigationEndpoint": {
              "clickTrackingParams": "CIsDEM2rARgBIhMInqXs06eb-AIV5swRCB3YEwCpSILE2te456zvtQE=",
              "commandMetadata": {
                "webCommandMetadata": {
                  "url": "https://www.youtube.com/redirect?event=video_description&redir_token=QUFFLUhqbnJYOTNiMEZPaGppaVZKNVl2RHhvajhSRjYtZ3xBQ3Jtc0trWTEzT1pfMUw4OU9Fc2Rkamc3b3ZKcnczM1QxdkhrbFI1NHFVN0hmZV9LaERDc0RobmtYTzl6RkFNTmJLTkFla1BQZ2tSaXBjRTFPNFozdFdaWC12cnl0X2dUMmNsMGVQSTN4S29mb1FNM29wdzRudw&q=https%3A%2F%2Fwww.squarespace.com%2FWAN&v=td6zO4r2ogI",
                  "webPageType": "WEB_PAGE_TYPE_UNKNOWN",
                  "rootVe": 83769
                }
              },
              "urlEndpoint": {
                "url": "https://www.youtube.com/redirect?event=video_description&redir_token=QUFFLUhqbnJYOTNiMEZPaGppaVZKNVl2RHhvajhSRjYtZ3xBQ3Jtc0trWTEzT1pfMUw4OU9Fc2Rkamc3b3ZKcnczM1QxdkhrbFI1NHFVN0hmZV9LaERDc0RobmtYTzl6RkFNTmJLTkFla1BQZ2tSaXBjRTFPNFozdFdaWC12cnl0X2dUMmNsMGVQSTN4S29mb1FNM29wdzRudw&q=https%3A%2F%2Fwww.squarespace.com%2FWAN&v=td6zO4r2ogI",
                "target": "TARGET_NEW_WINDOW",
                "nofollow": true
              }
            }
          },
          {
            "text": " and use offer code WAN for 10% off\nGet 50% off (up to $200) your annual Zoho Desk subscription at: "
          },
        ]
      },
      "metadataRowContainer": {
        "metadataRowContainerRenderer": {
          "collapsedItemCount": 0,
          "trackingParams": "CIsDEM2rARgBIhMInqXs06eb-AIV5swRCB3YEwCp"
        }
      },
      "showMoreText": {
        "simpleText": "Show more"
      },
      "showLessText": {
        "simpleText": "Show less"
      },
    }
   );
  let u: VideoSecondaryInfoRenderer = serde_json::from_value(j).unwrap();
  // Assert that the title text is Linus Tech Tips
  assert_eq!(u.owner.video_owner_renderer.title.runs.get(0).unwrap().text, "Linus Tech Tips");
  assert_eq!(u.description.runs.len(), 3);
}
#[tokio::test]
async fn test_compact_video_renderer(){
  let j: serde_json::Value = json!({
    "videoId": "qjw8ohwZ4nY",
    "thumbnail": {
      "thumbnails": [
        {
          "url": "https://i.ytimg.com/vi/qjw8ohwZ4nY/hqdefault.jpg?sqp=-oaymwEiCKgBEF5IWvKriqkDFQgBFQAAAAAYASUAAMhCPQCAokN4AQ==&rs=AOn4CLBqQOG4TTkeEcvziR8iHav1gHLCMA",
          "width": 168,
          "height": 94
        },
        {
          "url": "https://i.ytimg.com/vi/qjw8ohwZ4nY/hqdefault.jpg?sqp=-oaymwEjCNACELwBSFryq4qpAxUIARUAAAAAGAElAADIQj0AgKJDeAE=&rs=AOn4CLAfZFFZiZ2jETmbggU3II3L7ahsLQ",
          "width": 336,
          "height": 188
        }
      ]
    },
    "title": {
      "accessibility": {
        "accessibilityData": {
          "label": "Story Time! - WAN Show May 27, 2022 by Linus Tech Tips Streamed 10 days ago 1 hour, 39 minutes 472,838 views"
        }
      },
      "simpleText": "Story Time! - WAN Show May 27, 2022"
    },
    "longBylineText": {
      "runs": [
        {
          "text": "Linus Tech Tips",
          "navigationEndpoint": {
            "clickTrackingParams": "CIIDEKQwGAAiEwiepezTp5v4AhXmzBEIHdgTAKkyBnJlbG1mdQ==",
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/user/LinusTechTips"
            }
          }
        }
      ]
    },
    "publishedTimeText": {
      "simpleText": "Streamed 10 days ago"
    },
    "viewCountText": {
      "simpleText": "472,838 views"
    },
    "lengthText": {
      "accessibility": {
        "accessibilityData": {
          "label": "1 hour, 39 minutes, 36 seconds"
        }
      },
      "simpleText": "1:39:36"
    },
    "navigationEndpoint": {
      "clickTrackingParams": "CIIDEKQwGAAiEwiepezTp5v4AhXmzBEIHdgTAKkyBnJlbG1mdUiCxNrXuOes77UBmgEFCAEQ-B0=",
      "commandMetadata": {
        "webCommandMetadata": {
          "url": "/watch?v=qjw8ohwZ4nY",
          "webPageType": "WEB_PAGE_TYPE_WATCH",
          "rootVe": 3832
        }
      },
      "watchEndpoint": {
        "videoId": "qjw8ohwZ4nY",
        "nofollow": true,
        "watchEndpointSupportedOnesieConfig": {
          "html5PlaybackOnesieConfig": {
            "commonConfig": {
              "url": "https://rr3---sn-4g5e6nzl.googlevideo.com/initplayback?source=youtube&orc=1&oeis=1&c=WEB&oad=3200&ovd=3200&oaad=11000&oavd=11000&ocs=700&oewis=1&oputc=1&ofpcc=1&msp=1&odeak=1&odepv=1&osfc=1&id=aa3c3ca21c19e276&ip=159.48.53.142&initcwndbps=1865000&mt=1654603496&oweuc=&pxtags=Cg4KAnR4EggyNDE5MDk2MA&rxtags=Cg4KAnR4EggyNDE5MDk2MA%2CCg4KAnR4EggyNDE5MDk2MQ"
            }
          }
        }
      }
    },
    "shortBylineText": {
      "runs": [
        {
          "text": "Linus Tech Tips",
          "navigationEndpoint": {
            "clickTrackingParams": "CIIDEKQwGAAiEwiepezTp5v4AhXmzBEIHdgTAKkyBnJlbG1mdQ==",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/user/LinusTechTips",
                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                "rootVe": 3611,
                "apiUrl": "/youtubei/v1/browse"
              }
            },
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/user/LinusTechTips"
            }
          }
        }
      ]
    },
    "channelThumbnail": {
      "thumbnails": [
        {
          "url": "https://yt3.ggpht.com/ytc/AKedOLSZnNA3eg_fDwUgsCxpqMnVWu3UGo-Rln4621ncIQ=s68-c-k-c0x00ffffff-no-rj",
          "width": 68,
          "height": 68
        }
      ]
    },
    "ownerBadges": [
      {
        "metadataBadgeRenderer": {
          "icon": {
            "iconType": "CHECK_CIRCLE_THICK"
          },
          "style": "BADGE_STYLE_TYPE_VERIFIED",
          "tooltip": "Verified",
          "trackingParams": "CIIDEKQwGAAiEwiepezTp5v4AhXmzBEIHdgTAKk=",
          "accessibilityData": {
            "label": "Verified"
          }
        }
      }
    ],
    "trackingParams": "CIIDEKQwGAAiEwiepezTp5v4AhXmzBEIHdgTAKlA9sTn4KGUj56qAQ==",
    "shortViewCountText": {
      "accessibility": {
        "accessibilityData": {
          "label": "472K views"
        }
      },
      "simpleText": "472K views"
    },
    "accessibility": {
      "accessibilityData": {
        "label": "Story Time! - WAN Show May 27, 2022 - 1 hour, 39 minutes - Go to channel - Linus Tech Tips - 472K views - Streamed 10 days ago - play video"
      }
    }
  });
  let u: CompactVideoRenderer = serde_json::from_value(j).unwrap();
  assert_eq!(u.title.simple_text.unwrap(), "Story Time! - WAN Show May 27, 2022");
}
#[tokio::test]
async fn test_video_renderer(){
  let j: serde_json::Value = json!({
    "videoId": "Ti8scviDiYc",
    "thumbnail": {
      "thumbnails": [
        {
          "url": "https://i.ytimg.com/vi/Ti8scviDiYc/hqdefault.jpg?sqp=-oaymwEjCOADEI4CSFryq4qpAxUIARUAAAAAGAElAADIQj0AgKJDeAE=&rs=AOn4CLAE07nKdjfmXi0ZH5ZEmSn7C_R9XA",
          "width": 480,
          "height": 270
        }
      ]
    },
    "title": {
      "runs": [
        {
          "text": "Dream has Too Much Money"
        }
      ],
      "accessibility": {
        "accessibilityData": {
          "label": "Dream has Too Much Money by Linus Tech Tips 23 hours ago 23 minutes 1,332,629 views"
        }
      }
    },
    "longBylineText": {
      "runs": [
        {
          "text": "Linus Tech Tips",
          "navigationEndpoint": {
            "clickTrackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA=",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/c/LinusTechTips",
                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                "rootVe": 3611,
                "apiUrl": "/youtubei/v1/browse"
              }
            },
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/c/LinusTechTips"
            }
          }
        }
      ]
    },
    "publishedTimeText": {
      "simpleText": "23 hours ago"
    },
    "lengthText": {
      "accessibility": {
        "accessibilityData": {
          "label": "23 minutes, 55 seconds"
        }
      },
      "simpleText": "23:55"
    },
    "viewCountText": {
      "simpleText": "1,332,629 views"
    },
    "navigationEndpoint": {
      "clickTrackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMAyBnNlYXJjaFIMbHR0IHBsYXlsaXN0mgEDEPQk",
      "commandMetadata": {
        "webCommandMetadata": {
          "url": "/watch?v=Ti8scviDiYc",
          "webPageType": "WEB_PAGE_TYPE_WATCH",
          "rootVe": 3832
        }
      },
      "watchEndpoint": {
        "videoId": "Ti8scviDiYc",
        "params": "qgMMbHR0IHBsYXlsaXN0ugMkEiJQTDhtRy1Sa04ydVR5NXpCbFFzdHVUbklVRVFQZTVyREh4ugMkEiJQTDhtRy1Sa04ydVR3MmRLZWUydXRYbWRFQmhtU2JBYmMwugMkEiJQTDhtRy1Sa04ydVR4THNRaE95TTVUQmdNSEY5VjRHZnFhugMkEiJQTGc5a011V3VUaEctLVY2ZVdpSUc4UGFwYjhWb0dmdllOugMkEiJQTGlUeE9nREIzQ3dBeTJjWko0S18tbmlmUFh2MG5zUV9PugMkEiJQTFpYVDdvMmRabFhmVzFiRUxwREFPSWxENXNSbWkyVDBHugMkEiJQTDhtRy1Sa04ydVR6Z3lBOHp6RTh2UkIzX1pYUWZ1RlJ6ugMkEiJQTDhtRy1Sa04ydVR3bGNMVV9ydjI5R3Q0NTc4WGZnSm1fugMkEiJQTDhtRy1Sa04ydVR5dUV1dFFhNzlSWjBRNXU1Z3RlVWNpugMkEiJQTGVxZW92cGl5TjFSY2tzUW1rYTRrenFQOEYzWEJPLVozugMkEiJQTG5DangzRXlFZkg2NEpIa1dFNDBtcnBLUDNkeVFhcGh6ugMkEiJQTDhtRy1Sa04ydVR3N1BobG5BcjRwWlp6MlF1YklidWpIugMkEiJQTG5DangzRXlFZkg2THowa041YVV4OWdjM0ctZUR1Y3U1ugMkEiJQTDhtRy1Sa04ydVR3ZTJWNVhNbTdtbmNOR25FcUJiZlhnugMkEiJQTDhtRy1Sa04ydVR4dmtHR0JfQ0gzNHBOdlJkWVJuTHk0ugMkEiJQTEttNFR0WHRTUjVJV0w0c2oyZzVjLTZwUlM3dVJOSHh6ugMkEiJQTDhtRy1Sa04ydVR4Zks2WEtPQmtDQ3Q5d2xnMDl4c19KugMkEiJQTEttNFR0WHRTUjVJWDNJQnNtVFFWMzB1QkZ6N2pfcmRP",
        "watchEndpointSupportedOnesieConfig": {
          "html5PlaybackOnesieConfig": {
            "commonConfig": {
              "url": "https://rr3---sn-4g5edndz.googlevideo.com/initplayback?source=youtube&orc=1&oeis=1&c=WEB&oad=3200&ovd=3200&oaad=11000&oavd=11000&ocs=700&oewis=1&oputc=1&ofpcc=1&msp=1&odeak=1&odepv=1&osfc=1&id=4e2f2c72f8838987&ip=159.48.53.240&initcwndbps=1530000&mt=1654534394&oweuc="
            }
          }
        }
      }
    },
    "badges": [
      {
        "metadataBadgeRenderer": {
          "style": "BADGE_STYLE_TYPE_SIMPLE",
          "label": "New",
          "trackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA="
        }
      },
      {
        "metadataBadgeRenderer": {
          "style": "BADGE_STYLE_TYPE_SIMPLE",
          "label": "4K",
          "trackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA="
        }
      }
    ],
    "ownerBadges": [
      {
        "metadataBadgeRenderer": {
          "icon": {
            "iconType": "CHECK_CIRCLE_THICK"
          },
          "style": "BADGE_STYLE_TYPE_VERIFIED",
          "tooltip": "Verified",
          "trackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA=",
          "accessibilityData": {
            "label": "Verified"
          }
        }
      }
    ],
    "ownerText": {
      "runs": [
        {
          "text": "Linus Tech Tips",
          "navigationEndpoint": {
            "clickTrackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA=",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/c/LinusTechTips",
                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                "rootVe": 3611,
                "apiUrl": "/youtubei/v1/browse"
              }
            },
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/c/LinusTechTips"
            }
          }
        }
      ]
    },
    "shortBylineText": {
      "runs": [
        {
          "text": "Linus Tech Tips",
          "navigationEndpoint": {
            "clickTrackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMA=",
            "commandMetadata": {
              "webCommandMetadata": {
                "url": "/c/LinusTechTips",
                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                "rootVe": 3611,
                "apiUrl": "/youtubei/v1/browse"
              }
            },
            "browseEndpoint": {
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "canonicalBaseUrl": "/c/LinusTechTips"
            }
          }
        }
      ]
    },
    "trackingParams": "CPYBENwwGAQiEwj4hLWZpZn4AhWFRHoFHRgkAMBAh5OOxK-Oy5dO",
    "showActionMenu": false,
    "shortViewCountText": {
      "accessibility": {
        "accessibilityData": {
          "label": "1.3 million views"
        }
      },
      "simpleText": "1.3M views"
    },
  });
  let u: VideoRenderer = serde_json::from_value(j).unwrap();
  assert_eq!(u.title.runs.get(0).unwrap().text, "Dream has Too Much Money")
}
#[tokio::test]
async fn test_next_query(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::next_with_data(json!({
    "videoId": "td6zO4r2ogI"
  }),client_config).await.unwrap();
  let result: NextResult = serde_json::from_value(j).unwrap();
  assert_ne!(result.contents.unwrap().two_column_watch_next_results.unwrap().results.results.contents.len(),0)
}
#[tokio::test]
async fn test_next_query_continuation(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::next("Eg0SC2dDNmRRclNjbUhFGAYyVSIuIgtnQzZkUXJTY21IRTAAeAKqAhpVZ3d1eFlpV0dWYlV2SVRVdUZSNEFhQUJBZzABQiFlbmdhZ2VtZW50LXBhbmVsLWNvbW1lbnRzLXNlY3Rpb24%3D",client_config).await.unwrap();
  let result: NextResult = serde_json::from_value(j).unwrap();
  assert_eq!(result.contents.is_none(),true)
}
#[tokio::test]
async fn test_browse_query_browse_id(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::browse_browseid("UCXuqSBlHAE6Xw-yeJA0Tunw","EgZ2aWRlb3O4AQA%3D",client_config).await.unwrap();
  let result: BrowseResult = serde_json::from_value(j).unwrap();
  assert_eq!(result.contents.unwrap().two_column_browse_results_renderer.is_some(),true);
  assert_eq!(result.metadata.channel_metadata_renderer.title, "Linus Tech Tips");
}
#[tokio::test]
async fn test_browse_query_continuation(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::browse_continuation("4qmFsgJ_EhhVQ1h1cVNCbEhBRTZYdy15ZUpBMFR1bncaNEVnWjJhV1JsYjNNWUF5QUFNQUU0QWVvREZFVm5jMGx5WDBOQmJWcExWQzFpZG1aQlUyZDWaAixicm93c2UtZmVlZFVDWHVxU0JsSEFFNlh3LXllSkEwVHVud3ZpZGVvczEwMg%3D%3D",client_config).await.unwrap();
  let result: BrowseResult = serde_json::from_value(j).unwrap();
  assert_eq!(result.contents.is_none(),true);
  assert_eq!(result.on_response_received_actions.is_some(), true);
  assert_eq!(result.metadata.channel_metadata_renderer.title, "Linus Tech Tips");
}

#[tokio::test]
async fn test_player_query(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::player("nr1JnAmy5BA","",client_config).await.unwrap();
  let result: PlayerResult = serde_json::from_value(j).unwrap();
  assert_eq!(result.playability_status.status, "OK");
}
#[tokio::test]
async fn test_search_query(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::search("ltt","",client_config).await.unwrap();
  let result: SearchResult = serde_json::from_value(j).unwrap();
  assert_ne!(result.contents.two_column_search_results_renderer.unwrap().primary_contents.section_list_renderer.contents.len(), 0);
  assert_eq!(result.refinements.is_none(), true);
}
#[tokio::test]
async fn test_search_query_with_refinements(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::search("ltt playlist","",client_config).await.unwrap();
  let result: SearchResult = serde_json::from_value(j).unwrap();
  assert_ne!(result.contents.two_column_search_results_renderer.unwrap().primary_contents.section_list_renderer.contents.len(), 0);
  assert_eq!(result.refinements.is_none(), false);
}
#[tokio::test]
async fn test_search_query_different_query(){
  let client_config = &default_client_config();
  let j: serde_json::Value = endpoints::search("sdjfjds","",client_config).await.unwrap();
  let result: SearchResult = serde_json::from_value(j).unwrap();
  assert_ne!(result.contents.two_column_search_results_renderer.unwrap().primary_contents.section_list_renderer.contents.len(), 0);
  assert_eq!(result.refinements.is_none(), true);
}
}