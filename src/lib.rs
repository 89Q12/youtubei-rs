/// contains functions to generate the data to make an API call.
/// private because they return raw data and or errors
pub(crate) mod endpoints;
/// private because they are intended to be called by query functions only
pub(crate) mod extractors;
/// Contains the main functions of this library
/// Refer to the tests for usage information
pub mod query;
/// Contains all the structs used to represent the data returned by the API
pub mod types;
/// Contains utilities for various tasks
pub mod utils;
/// Contains test functions
#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::{
        channel::*,
        video::{CompactVideoRenderer, VideoPrimaryInfoRenderer, VideoSecondaryInfoRenderer},
    };
    use crate::{
        endpoints, extractors,
        query::{
            get_channel_info_legacy, get_comments_legacy, get_playlist_legacy, get_video_legacy,
            load_related_videos_legacy,
        },
        types::{
            filter,
            query_results::{BrowseResult, NextResult, PlayerResult, ResolveResult, SearchResult},
            video::VideoRenderer,
        },
        utils::default_client_config,
    };

    #[tokio::test]
    async fn fetch_video_legacy() {
        let client_config = &default_client_config();
        // gC6dQrScmHE is the id of an LTT video, used here because LTT doesn't delete videos or make them private
        let video_query =
            get_video_legacy("gC6dQrScmHE".to_string(), "".to_string(), &client_config).await;
        // Checks that there is indeed a video
        assert_eq!(video_query.is_ok(), true);
        // unwrap video
        let video = &video_query.as_ref().unwrap().video;
        let comment_token = &video_query.as_ref().unwrap().continuation_comments;

        // Assert that the video has a title
        assert_eq!(
            video.title,
            String::from("Running a YouTube Business is EASY (just kidding)")
        );
        // Assert that there is a token to fetch_comments
        assert_eq!(comment_token.is_empty(), false);
    }
    #[tokio::test]
    async fn fetch_comments_legacy() {
        let client_config = &default_client_config();
        // Comments for video gC6dQrScmHE
        let comment_query = get_comments_legacy("Eg0SC2dDNmRRclNjbUhFGAYyVSIuIgtnQzZkUXJTY21IRTAAeAKqAhpVZ3d1eFlpV0dWYlV2SVRVdUZSNEFhQUJBZzABQiFlbmdhZ2VtZW50LXBhbmVsLWNvbW1lbnRzLXNlY3Rpb24%3D".to_string(),&client_config).await;
        // Checks that there are comments
        assert_eq!(comment_query.is_ok(), true);
        // unwrap into comments
        let comments = &comment_query.as_ref().unwrap().comments;
        let ctoken = &comment_query.as_ref().unwrap().continuation;
        // Assert that there are comments in the vector
        assert_ne!(comments.len(), 0);
        // Assert that there is a ctoken
        assert_eq!(ctoken.is_empty(), false);
    }

    #[tokio::test]
    async fn fetch_related_legacy() {
        let client_config = &default_client_config();
        // Related videos for video gC6dQrScmHE
        let related_query = load_related_videos_legacy("CBQSDRILZ0M2ZFFyU2NtSEUYACqIBjJzNkw2d3lfQkFxOEJBb0Q4ajRBQ2c3Q1Bnc0k1X2o1cGJENTNxZk9BUW9EOGo0QUNnM0NQZ29JMHZ1Y29wYlczT292Q2dQeVBnQUtEc0ktQ3dpdnRkTzJydV9uaGZ3QkNnUHlQZ0FLRGNJLUNnakR5TDZzZ19taXVWOEtBX0ktQUFvT3dqNExDTVBTeWEycm9McnltQUVLQV9JLUFBb053ajRLQ0w3RWpNM3NpcFdaWmdvRDhqNEFDZzNDUGdvSWtlZnoxOVR1eC1ZdUNnUHlQZ0FLRGNJLUNnam13XzJidFpHSnhrb0tBX0ktQUFvT3dqNExDTjZZa2EzVHZkNmQtd0VLQV9JLUFBb093ajRMQ0p6Rm1mM3VvdURwa1FFS0FfSS1BQW9Od2o0S0NMZXNvTGFieU9tU01Bb0Q4ajRBQ2czQ1Bnb0kySUxCMDZ6N3R0d05DZ1B5UGdBS0RjSS1DZ2lSdHBtN3U5RHduaGtLQV9JLUFBb093ajRMQ0lYem00eTZ5c2lFbndFS0FfSS1BQW9Pd2o0TENJMkN3LWFnc05yS3BRRUtBX0ktQUFvTndqNEtDUG41azRUUDJicTdiQW9EOGo0QUNnN0NQZ3NJdHEyR3A4R1E0dlBMQVFvRDhqNEFDZzNDUGdvSTg4eVRrLVR4N3MwMkNnUHlQZ0FLRGNJLUNnalIxTm14X01UWWpVb0tBX0ktQUFvTndqNEtDSmU1ME15OTRwV0NUaElVQUFJRUJnZ0tEQTRRRWhRV0dCb2NIaUFpSkNZYUJBZ0FFQUVhQkFnQ0VBTWFCQWdFRUFVYUJBZ0dFQWNhQkFnSUVBa2FCQWdLRUFzYUJBZ01FQTBhQkFnT0VBOGFCQWdRRUJFYUJBZ1NFQk1hQkFnVUVCVWFCQWdXRUJjYUJBZ1lFQmthQkFnYUVCc2FCQWdjRUIwYUJBZ2VFQjhhQkFnZ0VDRWFCQWdpRUNNYUJBZ2tFQ1VhQkFnbUVDY3FGQUFDQkFZSUNnd09FQklVRmhnYUhCNGdJaVFtag93YXRjaC1uZXh0LWZlZWR4AQ%3D%3D".to_string(),&client_config).await;
        // Checks that there are related videos
        assert_eq!(related_query.is_ok(), true);
        // unwrap into a related videos vector
        let related = related_query.unwrap();
        // Assert that there are 20 videos in the vector
        assert_eq!(related.len(), 20);
    }
    #[tokio::test]
    async fn fetch_playlist_legacy() {
        let client_config = &default_client_config();
        // PLe8jmEHFkvsbeJL2QNucGv00eO8PKbSUn is a long playlist from the channel Monstercat Uncaged
        let playlist_query = get_playlist_legacy(
            "PLe8jmEHFkvsbeJL2QNucGv00eO8PKbSUn".to_string(),
            &client_config,
        )
        .await;
        // Checks that there are related videos
        assert_eq!(playlist_query.is_ok(), true);
        // unwrap into a video playlist vector
        let playlist = playlist_query.unwrap();
        // Assert that there is the correct number of videos
        assert_eq!(playlist.video_count, "314 videos");
        // Assert that the videos vector has a len of 100
        assert_eq!(playlist.videos.len(), 100);
    }
    #[tokio::test]
    async fn fetch_channel_legacy() {
        let client_config = &default_client_config();
        // PLe8jmEHFkvsbeJL2QNucGv00eO8PKbSUn is a long playlist from the channel Monstercat Uncaged
        let channel_query =
            get_channel_info_legacy("UCXuqSBlHAE6Xw-yeJA0Tunw".to_string(), &client_config).await;
        // Checks that there are related videos
        assert_eq!(channel_query.is_ok(), true);
        // unwrap into a video playlist vector
        let channel = channel_query.unwrap().channel;
        // Assert that there is the correct number of videos
        assert_eq!(channel.name, "Linus Tech Tips");
    }
    #[tokio::test]
    async fn test_channel_renderer() {
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
    async fn test_video_primary_info_renderer() {
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
        assert_eq!(
            u.title.runs.get(0).unwrap().text,
            "The Lab is a Disaster - WAN Show June 3, 2022"
        );
    }

    #[tokio::test]
    async fn test_video_secondary_info_renderer() {
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
        assert_eq!(
            u.owner.video_owner_renderer.title.runs.get(0).unwrap().text,
            "Linus Tech Tips"
        );
        assert_eq!(u.description.runs.len(), 3);
    }
    #[tokio::test]
    async fn test_compact_video_renderer() {
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
                "hei&ght": 68
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
        assert_eq!(
            u.title.simple_text.unwrap(),
            "Story Time! - WAN Show May 27, 2022"
        );
    }
    #[tokio::test]
    async fn test_video_renderer() {
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
        assert_eq!(
            u.title.runs.get(0).unwrap().text,
            "Dream has Too Much Money"
        )
    }
    #[tokio::test]
    async fn test_next_query() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::next_with_data(
            json!({
              "videoId": "td6zO4r2ogI"
            }),
            client_config,
        )
        .await
        .unwrap();
        let result: NextResult = serde_json::from_value(j).unwrap();
        assert_ne!(
            result
                .contents
                .unwrap()
                .two_column_watch_next_results
                .unwrap()
                .results
                .results
                .contents
                .len(),
            0
        )
    }

    #[tokio::test]
    async fn test_next_query_continuation() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::next("Eg0SC2dDNmRRclNjbUhFGAYyVSIuIgtnQzZkUXJTY21IRTAAeAKqAhpVZ3d1eFlpV0dWYlV2SVRVdUZSNEFhQUJBZzABQiFlbmdhZ2VtZW50LXBhbmVsLWNvbW1lbnRzLXNlY3Rpb24%3D",client_config).await.unwrap();
        let result: NextResult = serde_json::from_value(j).unwrap();
        assert_eq!(result.contents.is_none(), true)
    }
    #[tokio::test]
    async fn test_browse_query_browse_hashtag() {
        let client_config = &default_client_config();
        let j: serde_json::Value =
            endpoints::browse_browseid("FEhashtag", "6gUECgJwYw%3D%3D", client_config)
                .await
                .unwrap();
        let result: BrowseResult = serde_json::from_value(j).unwrap();
        assert_eq!(
            result
                .contents
                .unwrap()
                .two_column_browse_results_renderer
                .is_some(),
            true
        );
    }
    #[tokio::test]
    async fn test_browse_query_browse_id() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EgZ2aWRlb3O4AQA%3D",
            client_config,
        )
        .await
        .unwrap();
        let result: BrowseResult = serde_json::from_value(j).unwrap();
        assert_eq!(
            result
                .contents
                .unwrap()
                .two_column_browse_results_renderer
                .is_some(),
            true
        );
        assert_eq!(
            result.metadata.unwrap().channel_metadata_renderer.title,
            "Linus Tech Tips"
        );
    }
    #[tokio::test]
    async fn test_browse_query_browse_id_no_params() {
        let client_config = &default_client_config();
        let j: serde_json::Value =
            endpoints::browse_browseid("UCXuqSBlHAE6Xw-yeJA0Tunw", "", client_config)
                .await
                .unwrap();
        let result: BrowseResult = serde_json::from_value(j).unwrap();
        assert_eq!(
            result
                .contents
                .unwrap()
                .two_column_browse_results_renderer
                .is_some(),
            true
        );
        assert_eq!(
            result.metadata.unwrap().channel_metadata_renderer.title,
            "Linus Tech Tips"
        );
    }
    #[tokio::test]
    async fn test_browse_query_continuation() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_continuation("4qmFsgJ_EhhVQ1h1cVNCbEhBRTZYdy15ZUpBMFR1bncaNEVnWjJhV1JsYjNNWUF5QUFNQUU0QWVvREZFVm5jMGx5WDBOQmJWcExWQzFpZG1aQlUyZDWaAixicm93c2UtZmVlZFVDWHVxU0JsSEFFNlh3LXllSkEwVHVud3ZpZGVvczEwMg%3D%3D",client_config).await.unwrap();
        let result: BrowseResult = serde_json::from_value(j).unwrap();
        assert_eq!(result.contents.is_none(), true);
        assert_eq!(result.on_response_received_actions.is_some(), true);
        assert_eq!(
            result.metadata.unwrap().channel_metadata_renderer.title,
            "Linus Tech Tips"
        );
    }

    #[tokio::test]
    async fn test_player_query() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::player("nr1JnAmy5BA", "", client_config)
            .await
            .unwrap();
        let result: PlayerResult = serde_json::from_value(j).unwrap();
        assert_eq!(result.playability_status.status, "OK");
    }
    #[tokio::test]
    async fn test_search_query_sort_by_newest() {
        let client_config = &default_client_config();
        let j: serde_json::Value =
            endpoints::search("ltt", filter::DateFilters::TODAY, client_config)
                .await
                .unwrap();
        let result: SearchResult = serde_json::from_value(j).unwrap();
        assert_ne!(
            result
                .contents
                .two_column_search_results_renderer
                .unwrap()
                .primary_contents
                .section_list_renderer
                .contents
                .len(),
            0
        );
    }
    #[tokio::test]
    async fn test_search_query() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::search("ltt", "", client_config).await.unwrap();
        let result: SearchResult = serde_json::from_value(j).unwrap();
        assert_ne!(
            result
                .contents
                .two_column_search_results_renderer
                .unwrap()
                .primary_contents
                .section_list_renderer
                .contents
                .len(),
            0
        );
    }
    #[tokio::test]
    async fn test_search_query_with_refinements() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::search("ltt playlist ", "", client_config)
            .await
            .unwrap();
        let result: SearchResult = serde_json::from_value(j).unwrap();
        assert_ne!(
            result
                .contents
                .two_column_search_results_renderer
                .unwrap()
                .primary_contents
                .section_list_renderer
                .contents
                .len(),
            0
        );
        assert_eq!(result.refinements.is_none(), false);
    }
    #[tokio::test]
    async fn test_search_query_different_query() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::search("sdjfjds", "", client_config)
            .await
            .unwrap();
        let result: SearchResult = serde_json::from_value(j).unwrap();
        assert_ne!(
            result
                .contents
                .two_column_search_results_renderer
                .unwrap()
                .primary_contents
                .section_list_renderer
                .contents
                .len(),
            0
        );
    }
    #[tokio::test]
    async fn test_resolve_url() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::resolve_url(
            "https://www.youtube.com//c/LinusTechTips/video",
            client_config,
        )
        .await
        .unwrap();
        let result: ResolveResult = serde_json::from_value(j).unwrap();
        assert_eq!(result.endpoint.browse_endpoint.is_some(), true);
        assert_ne!(result.endpoint.browse_endpoint.unwrap().browse_id, "")
    }
    #[tokio::test]
    async fn test_resolve_url_extractor_success() {
        let client_config = &default_client_config();
        let result = extractors::extract_resolve_result(
            &endpoints::resolve_url(
                "https://www.youtube.com//c/LinusTechTips/video",
                client_config,
            )
            .await
            .unwrap(),
        );
        assert_eq!(result.is_ok(), true);
        assert_ne!(
            result.unwrap().endpoint.browse_endpoint.unwrap().browse_id,
            ""
        )
    }

    #[tokio::test]
    async fn test_search_extractor_success() {
        let client_config = &default_client_config();
        let result = extractors::extract_search_result(
            &endpoints::search("ltt", "", client_config).await.unwrap(),
        );
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_search_extractor_error() {
        let client_config = &default_client_config();
        let result = extractors::extract_search_result(
            &endpoints::search("", "", client_config).await.unwrap(),
        );
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.unwrap_err().to_parse_type, "SearchResult");
    }

    #[tokio::test]
    async fn test_next_extractor_success() {
        let client_config = &default_client_config();
        let result = extractors::extract_next_result(
            &endpoints::next_with_data(
                json!({
                  "videoId": "td6zO4r2ogI"
                }),
                client_config,
            )
            .await
            .unwrap(),
        );
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_next_extractor_error() {
        let client_config = &default_client_config();
        let result = extractors::extract_next_result(
            &endpoints::next_with_data(
                json!({
                  "videoId": "e"
                }),
                client_config,
            )
            .await
            .unwrap(),
        );
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.unwrap_err().to_parse_type, "NextResult");
    }

    #[tokio::test]
    async fn test_browse_extractor_success() {
        let client_config = &default_client_config();
        let result = extractors::extract_browse_result(
            &endpoints::browse_browseid(
                "UCXuqSBlHAE6Xw-yeJA0Tunw",
                "EgZ2aWRlb3O4AQA%3D",
                client_config,
            )
            .await
            .unwrap(),
        );
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_browse_extractor_error() {
        let client_config = &default_client_config();
        // FAILS because FEwhat_to_watch isn't supported yet
        let result = extractors::extract_browse_result(
            &endpoints::browse_browseid("FEwhat_to_watch", "", client_config)
                .await
                .unwrap(),
        );
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.unwrap_err().to_parse_type, "BrowseResult");
    }
    #[tokio::test]
    async fn test_player_extractor_success() {
        let client_config = &default_client_config();
        let result = extractors::extract_player_result(
            &endpoints::player("nr1JnAmy5BA", "", client_config)
                .await
                .unwrap(),
        );
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_player_extractor_error() {
        let client_config = &default_client_config();
        let result = extractors::extract_player_result(
            &endpoints::player("", "", client_config).await.unwrap(),
        );
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.unwrap_err().to_parse_type, "PlayerResult");
    }

    #[tokio::test]
    async fn test_endpoint_query_fail() {
        let client_config = &default_client_config();
        let result = endpoints::browse_browseid("", "", client_config).await;
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.unwrap_err().status, 400);
    }
    #[tokio::test]
    async fn test_next_video_music() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::next_with_data(
            json!({
              "videoId": "zYRHFT_Z7VY"
            }),
            client_config,
        )
        .await
        .unwrap();
        let result: Result<NextResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_player_video_music() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::player("zYRHFT_Z7VY", "", client_config)
            .await
            .unwrap();
        let result: Result<PlayerResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_next_video_playlist() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::next_with_data(
            json!({
              "videoId":"hPQXVIBSd_o",
              "playlistId": "PLOd3J15SswK6yaIsQbEHJy73Xx1VNNLym",
            }),
            client_config,
        )
        .await
        .unwrap();
        let result: Result<NextResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result
                .unwrap()
                .contents
                .unwrap()
                .two_column_watch_next_results
                .unwrap()
                .playlist
                .unwrap()
                .playlist
                .title,
            "Bootleg Remixes"
        );
    }
    #[tokio::test]
    async fn test_channel_videos() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EgZ2aWRlb3O4AQDyBgQKAjoA",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_channel_playlists() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EglwbGF5bGlzdHO4AQDyBgQKAkIA",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_channel_channels() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EghjaGFubmVsc7gBAPIGBAoCUgA%3D",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_channel_community() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "Egljb21tdW5pdHm4AQDyBgQKAkoA",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_channel_about() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EgVhYm91dLgBAPIGBAoCEgA%3D",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
    }
    #[tokio::test]
    async fn test_channel_search() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_browseid(
            "UCXuqSBlHAE6Xw-yeJA0Tunw",
            "EgVhYm91dLgBAPIGBAoCEgA%3D",
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result
                .unwrap()
                .contents
                .unwrap()
                .two_column_browse_results_renderer
                .unwrap()
                .tabs
                .last()
                .unwrap()
                .expandable_tab_renderer
                .is_some(),
            true
        );
    }
    #[tokio::test]
    async fn test_query_channel_search() {
        let client_config = &default_client_config();
        let j: serde_json::Value = endpoints::browse_with_data(
            json!({
              "browseId": "UCXuqSBlHAE6Xw-yeJA0Tunw",
              "params": "EgZzZWFyY2i4AQDyBgQKAloA",
              "query": "best pc"
            }),
            client_config,
        )
        .await
        .unwrap();
        let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result
                .unwrap()
                .contents
                .unwrap()
                .two_column_browse_results_renderer
                .unwrap()
                .tabs
                .last()
                .unwrap()
                .expandable_tab_renderer
                .is_some(),
            true
        );
    }
}

#[cfg(test)]
mod fuzzer {
    use std::{vec, fs};
    use crate::{endpoints, utils::default_client_config, types::{self, query_results::BrowseResult}};

    #[tokio::test]
    async fn channel_fuzzer() {
        let channel_ids = vec![
            "UC001Akwv-hPOrpH4uYCci6w",
            "UC005KkNX0TJUh64ARQ-7mig",
            "UC-0078V3b7nUsAQodzIbOug",
            "UC00cZ56yxKpHD9TtV_5sQ0g",
            "UC00d6Dm1QdsTJ5VqW5aYsDg",
            "UC00IKbzqr7BnyKZLq_XmUDA",
            "UC00iNynkcwH5M-nxY2ew7-g",
            "UC0-0pQ9j3R2S-Vv2gTxJYAQ",
            "UC00q9EgMESf650K6i6Qy69w",
            "UC00q9Udz9vdljIfN-8fCkQQ",
            "UC00t3cqGktwp5VE_-l2M99A",
            "UC00uG71I6iPyx15EX6i_GDA",
            "UC00VfTvnXsopC0r0jeQlUOA",
            "UC00VfxvU5rWV48QshqnbDVQ",
            "UC00yIcQ7R3GJz_9bE5il2zQ",
            "UC01-dmjdy5cWBduKOmQwliw",
            "UC01fXAThaFAwXmN_c0tniHA",
            "UC01GJT61n1F7SWkrzw5vAqQ",
            "UC01K34_YKr83IdE7VmcXiFA",
            "UC01kfeLTKU0DGcRGI-OLSig",
            "UC01kJbL_58Ck-vc9DCRDELw",
            "UC01LQqj9YtMTsSa4dxMHLdQ",
            "UC01oU0AjXENXFOx7I2Ta3cA",
            "UC01sF0wMt4tB2O9krpJ_8Tg",
            "UC0-236CRknyhwCoz9SdWmeA",
            "UC023Zh9CowqW_kXPvmMs7Eg",
            "UC02coXfDPjEmU8uDT2G8Z2A",
            "UC0_2j4lNu1fRPb5c67SmeXQ",
            "UC02Mk2QHR9myF3VMrVYbCAA",
            "UC02rPC3Qfkbvg5HZL92iBgQ",
            "UC0_2X64KMo4UrarpRnz9q2g",
            "UC02X8_qvB3sncWwwD14YIig",
            "UC02x9yG9ZFF_VZp1VnMoptg",
            "UC02Z6oIaU0LS3H_7YxUUdSA",
            "UC034jlxsjmGqBH6qD1zMXYA",
            "UC034O5dHF4q1wbp5y9CIhRQ",
            "UC036PWSWq6NFrEwT7__AaIg",
            "UC03-cHK2tFT1bUiLNTBWs8A",
            "UC03cpKIZShIWoSBhfVE5bog",
            "UC03DUjaG30wy4T3S-JteywQ",
            "UC03IByk2jL6x1cZoNACF-5Q",
            "UC03j2jcKwm0wTJpDsu-9u4g",
            "UC03UlLPJ0O-oTNvB-6XXG6A",
            "UC03yWcN2vS2VFwRH6hQpi3g",
            "UC046lFvJZhiwSRWsoH8SFjg",
            "UC04dfXeTsgROBHSLm_C__JA",
            "UC04GkwUikshOM1l0a1RLCcw",
            "UC04kb2nMOeohM654SZT8IDA",
            "UC04KsGq3npibMCE9Td3mVDg",
            "UC04leoN79FGe6ZMmYytCdTw",
            "UC04ODTJuSkxdOujlAyPiWlw",
            "UC04QdEl71CFDogk8pzY7Geg",
            "UC04qEq4iKR6d80pI8CfoNqQ",
            "UC04rd2nSc91fg3sXVkn1-YQ",
            "UC058gnk0_LvV4IYBzOpvWgg",
            "UC059kFc5oApruDS2QFmZfeA",
            "UC059SA_tX-AptG0JdMgDG3g",
            "UC05_iIGvXue0sR01JNpRHzw",
            "UC05irBQFIIgHnAK5a2bosdQ",
            "UC05m1jT_pGm-DJKlpXsBYkA",
            "UC05mHIxrD6IG7dc3c1lEaJg",
            "UC05NSijsZpNM_mbd0-qagLA",
            "UC_05o96_jUAAeGXSKDJcKGg",
            "UC05phLqQlm0E2ni5H1L40lA",
            "UC05uI2AIsSIi6iwu3EydNuA",
            "UC05v8PDbY1pdVQ_SvbJik1g",
            "UC05XpvbHZUQOfA6xk4dlmcw",
            "UC0634WKsKy6w5NtSZAXVIgg",
            "UC0648KFtcXDZ_J_YHYdodnQ",
            "UC065Ka2Zz-HYi1t_ZbxdLkw",
            "UC06dmFBZtx_GyR4imIZ3uKQ",
            "UC-06dNQZmnR2ziCjrJjtm8A",
            "UC06E4Y_-ybJgBUMtXx8uNNw",
            "UC06fO6LNH_AUgjbmqaZRV5Q",
            "UC06gsEqFYvaEjEyd4kJVCQA",
            "UC06HVrkOL33D5lLnCPjr6NQ",
            "UC06JnFeyjkGQWivm8I0slRw",
            "UC06mx5YFrzdrVxr5ULkOLhQ",
            "UC06rf_bBTUmWWitQOQee3mQ",
            "UC06RuN3J1Hx1z5ULbkfWFZA",
            "UC0762vo1KLFJMV-4YQZk12g",
            "UC076_AwpalANIyMdla2hr6A",
            "UC077_u7Nlc9nprT16swSDCw",
            "UC07cgLSoTF81C2GSauwB3SA",
            "UC07diUZlEks3SntFATptoQA",
            "UC07DLXbOs9JK3aUZhNYtE9g",
            "UC0-7EfFwG4zvU28I5mA_OrA",
            "UC07F26kHKkpW_qqvXzEGALA",
            "UC07JWf9A0B1scApbS1Te7Ww",
            "UC07obEUk41ln4UeamTSwFGQ",
            "UC07qAfGd_Tqu8xRG6yPtw3A",
            "UC07s3Eb9-CkzGCzNXmn1l4Q",
            "UC07W8bIiwY-EXZ9wpPQZLPw",
            "UC07z9r4VHsH1mIsWFpc0AVw",
            "UC083VJ23T17StE7R7Fm00jQ",
            "UC085VgzVYSHVve3p8MhVkDw",
            "UC08AFOLZoUQr1UMjT4O98WQ",
            "UC08d7XK940GukzhSJetE-hg",
            "UC0-8dnwt5otLEDIXlHpEhKw",
            "UC_08jhZG1YSX3nxMVgQcc5w",
            "UC08jxjsTE4I1N-LcVxSn_9A",
            "UC08nqw3NQERl0-zFrLZbN-Q",
            "UC08NRgcXTvMKP-FOZrNScHg",
            "UC08NS8fyPpkYgnhj69fMXLg",
            "UC08NVDi0tgVdM6YxHoBPTTw",
            "UC08QfQDLAd9D7aYPFgBUIng",
            "UC0-8RxyOBqTIb0r8qJK8SVw",
            "UC08uLkDM1V-dCohclJ18F7g",
            "UC08zV15YWBq_YmHGumdn5Rw",
            "UC0974nWSIcyJElXUfYDv9_w",
            "UC09A6MJun_diDegBrkEA9TQ",
            "UC09GemGjss3hPdljwtJ8TPA",
            "UC09IvZwjpunzrdHH1EHok-w",
            "UC09jljgpnK3lNrfO9AfnIcg",
            "UC09m1_rUTHJU_ieCkctGI7g",
            "UC09MfxKp58Tx0xCk2PjI8Ig",
            "UC09N-xTBgL9AYiPR-acjFmQ",
            "UC09qASY4ixFS-KXIH6Nw0rg",
            "UC09RRmPeURAd41XZvyEvA7g",
            "UC09un3HxINkfdGjHvE1kXgw",
            "UC0A0FkyRVnGIDtNf1kTnF6g",
            "UC0A0TIq-WDyKZcGcOt5WpPA",
            "UC-0A1y7C0vlToW42BUn6PWg",
            "UC0A3ldncnGQ1M_RU2Wb4L2A",
            "UC0A5REF21kXBkG2KziS76FA",
            "UC0a8nteER_pU4Aj6hmEyJAQ",
            "UC0aanx5rpr7D1M7KCFYzrLQ",
            "UC0AapM8P4m5JWZartfs2qwg",
            "UC0abAX9cuVB0klLobCewq-g",
            "UC0abLhC3Ju8706RNlP57Jcg",
            "UC0ABxG3__GwQSmJ8f8C-o4Q",
            "UC0aCXqg4vjKL1DLWrxCcFTw",
            "UC0ad71gmCJIW1w2rnfr5p2A",
            "UC0AE_22J0kAo30yjasaeFqw",
            "UC0aF15xhdixN4AaeY5K9Wug",
            "UC0aF5ybbUcxzKgY8TMTerhw",
            "UC0aFOAetT4Hur0qmOnmmbBA",
            "UC0aGBL0vJSXAnIJ_QVJigKg",
            "UC0ahC64OhIAS11TJX9Ig86A",
            "UC0a-HjsmOWHF1AjivaZ50kA",
            "UC0AiicMcEhafHnxpKaWaE0Q",
            "UC0ALBBgM4nA2bdFLeKdOVBg",
            "UC0ALBkHA2PpAsIoLU3fP8Ug",
            "UC0alwLbzL_MzSRfV-eQTALw",
            "UC0aM3dvMGxZ5qCajPYx8dQA",
            "UC0ARaU0Q7u44O4UqsoYnTCw",
            "UC0ArlFuFYMpEewyRBzdLHiw",
            "UC0ASolYU_Yh3yShLFQC0stg",
            "UC0ATu8D2ssr4P0tiQyjDIPg",
            "UC0AVyhbJ1kGqVOfX-MEzrxA",
            "UC0AXjvO0o-nclfmp3uIpk_Q",
            "UC0AXrjhtw-sW2nCGAXEsSrA",
            "UC0AzbhpTxR4QOyrrQzADDxg",
            "UC0b9XnNEE36m6zJhrqIDCYw",
            "UC0BAd8tPlDqFvDYBemHcQPQ",
            "UC0bbrObaHRKqou_NVPVfOHw",
            "UC0BCfHbNORhd7dsw1R5rw9w",
            "UC0BdmNwtNraDHYwRecghXVA",
            "UC0bef7myp4PwhFA40mBFfXA",
            "UC0BfPXleDan8dMbel6-C89Q",
            "UC0BGhWsIbV7Dm-lsvhdlMbA",
            "UC0BhAOfmm1tJaJkPyTM9D_g",
            "UC0bHEieZgXW_3y8vnBS6GVQ",
            "UC0BiVs5EYh57gzGVvhddjsA",
            "UC0bJeYlwpwTZDQsamtRKy6A",
            "UC0BJVNTIEbG8CLG-xVVWJnA",
            "UC0BKsl7jz9DtIQJn_VaVQ9A",
            "UC0BkzA33G4pjrTaCXok9hTw",
            "UC0BletW9phE4xHFM44q4qKA",
            "UC0bLweCpVaSAv9LptfD_yHw",
            "UC0BNMeyXj5EtqrkIp6VpZFQ",
            "UC0bo0F5nUfsHTgUWf3FKrIw",
            "UC0BP-IX_TViUEK8jaey3vew",
            "UC0bpxzOXueQV7rRPomDiX8A",
            "UC0BS2lLese-pldAtdOUgsfg",
            "UC0BtS97KQR7I4Xqa9VYlkvg",
            "UC0BvmQ-UWDnDc9seBcSanRA",
            "UC0BYDvK2woZjAnVxcO6ILeQ",
            "UC0c0LfwlQm7IvDvTJRh8nhw",
            "UC0C1bN_WAwurH377z-zwwew",
            "UC0C4p51aWCigIcg2VTjPKzA",
            "UC0CBhqBJoNsHMpaJK2i5TPQ",
            "UC0CC_MaeMKk6qRoQVwFdo_g",
            "UC0cd_-e49hZpWLH3UIwoWRA",
            "UC0CelqHTkd3P7xgWEtB8ywQ",
            "UC0Cf21ahxlbODHpQkrBtVHw",
            "UC0Ci5xTGZJUAj6QhCaxb5_g",
            "UC0ckjBtm9SBV7JAjbELiBqQ",
            "UC0ClWPkK1d15bhZNe9as1Vw",
            "UC0CM9l-7mgfD78xkr_0x_-A",
            "UC0cMCMm7FBniXEJmpnGevYg",
            "UC0CMQOzVxuly38QDQBpmWVg",
            "UC0cM-UZLk5FQ_xmArJ_LZ9Q",
            "UC0CO22xzyBNh2kxIUCqNpfQ",
            "UC0coc93EYxWqNX6E5169lfw",
            "UC0CQejilA0JdC0pgBZasJJw",
            "UC0cqkGpdSBUGdjycy2PlpEw",
            "UC0CQVmtT10QMU-_p0td3TSQ",
            "UC0Cqz37q69OeugC2SaU4MVA",
            "UC0CrEorVNCMZFpZhLHE5Gww",
            "UC0CRYvGlWGlsGxBNgvkUbAg",
            "UC0cT8BZdFUmC0vgr1XS7aPA",
            "UC0_CTubjGeDth7X0bxUTBTA",
            "UC_0CVCfC_3iuHqmyClu59Uw",
            "UC0C-w0YjGpqDXGB8IHb662A",
            "UC0Cx8U3zwetxjj4RyG9Wa1A",
            "UC0_CYeIk4l1Hxmdn9-QoR8A",
            "UC0d07LOERMkOyZ55q_9Z_rg",
            "UC0d5sKy-bIVtQn0gW78S8YQ",
            "UC0D737uQtQB07AcPYTsxj1A",
            "UC0Db5l-EGukzDYmNlbrGG_Q",
            "UC0DHyM8QXuotuVBDJopJ7pw",
            "UC0dKEC2YFGGHAtEt4zvfVkA",
            "UC0dlKJ3o2FWsmEjlf6vwpJg",
            "UC0do3QDDQuTqsvQaueqskxw",
            "UC0dvS-ghN9l7ZTCIgKFIRLw",
            "UC0dw_oLSiCBDg6jDoB_WoCA",
            "UC0DXdgJ9iWXOrtXlykFsUvQ",
            "UC0dXVDfY8fiYtXPqd9tZhog",
            "UC0DzLU-BJYR5CjiUhCMuzvA",
            "UC0DZmkupLYwc0yDsfocLh0A",
            "UC_0e1271DHhRyiDIuX6B6aQ",
            "UC0E1n0GRgBW5gR7y7H9TjZQ",
            "UC0e3QhIYukixgh5VVpKHH9Q",
            "UC0ebyhnP_Tm6Ji3KfO0HXUQ",
            "UC0E-CsUUgPl_3nAtkTSNpCw",
            "UC0efyK_-Athm8yhyjNSqndQ",
            "UC0ehWFbU8-KZYSxFmRcKDVw",
            "UC0-EJOvyRsLbRjcQ5Dh__HA",
            "UC0elp2101KAxbaAMzInGerA",
            "UC0ENB77eFsgo7p-Wbd_nyDw",
            "UC0EnYUt8dJUYcDgHUFG8vYA",
            "UC0EOuD5peuu-9H3I7qTNnyw",
            "UC0epdXvEWECWIPRfxYPYPoQ",
            "UC0Eqb484X8EiNrFLSUQVsaA",
            "UC0ETiVE1OVi1vazCOPsFNBQ",
            "UC0EuX0qwDX6__lYj1Kj6msA",
            "UC0evg4WqWFxO_0Xa-bjsSRQ",
            "UC0E_vIe1e1lVeojYOgVg_5Q",
            "UC0EXZm7W7F7px2rcqecyC6w",
            "UC0EyYxvdqvYlrd-a90zdZ2g",
            "UC0EYzvqKgbXUq_n8OXnOX4A",
            "UC0f1qBOx8JcpQEVvpjLYUNg",
            "UC0f4WJAjYdwl4XYHz-6FhyQ",
            "UC0F7Xa87Sp7ECXb46gxDO6A",
            "UC0FbFtZDr0EEX8OXahAEOgA",
            "UC0FBHkDIr2X7HljJTiuAfww",
            "UC0FDE8Q3js8e9cWuM_5tZrg",
            "UC0fDG3byEcMtbOqPMymDNbw",
            "UC0FEQBZ09nQWKZM6XPbFLnA",
            "UC0FF8rI64vIy1NKJg7OIJng",
            "UC0FFFneMi9GwRHUsuBjM0jA",
            "UC0fFj_FgksTyPJzGoUVFBqw",
            "UC0fGGprihDIlQ3ykWvcb9hg",
            "UC0FiBrUztpMrxtU2uyUjwbw",
            "UC0fiLCwTmAukotCXYnqfj0A",
            "UC0_Fkp6dZRWbTSWWuHEs_xg",
            "UC0fkyMAE7T3OFs7lnGCr33A",
            "UC0f-O7PPedxMI-ytv2GrHlA",
            "UC0FplT88irKbHeZucDiFplw",
            "UC0FqAhvrGxaoq5p6Rcj7LkA",
            "UC0fqX3pZwxMCGpPUtvgi27Q",
            "UC0fuGw6v8qg5udyac2jH2fQ",
            "UC0fWc3uP_ZBmSa4Ajlk4Zdg",
            "UC0Fwhzr1yeoQ84yC3Isd4Lg",
            "UC0FWjj7lOPiNjEia-nVUXdg",
            "UC0g03EUNiHIKcGwi7zo_4Kw",
            "UC0g1ukvK4tW7X7sjFpxJssw",
            "UC0G2qz-hoaCswQNgoWU_LTw",
            "UC0g5iiBLfRbDy1ZuRLHu6mQ",
            "UC0GaOqh0_8Oq2C9q8Q1K-6g",
            "UC0gbkS-xhcWWXfWugQUSRuw",
            "UC0gciqg5QuMorT_nEsldUIw",
            "UC0gdSRU1EBmBoRFSG9UypHg",
            "UC0gElXJLp_puscXA7vW11sg",
            "UC0gEw6pgNkLkkzMwzX4UtHA",
            "UC0ggvGjoR7-xVrW3P19fpkg",
            "UC0GH8qOUixYszP5iSKLPB_A",
            "UC0GhiZR9zyPorNmoWyPClrQ",
            "UC0G-HRAsU5b_zot7ICRAKDg",
            "UC0gkKMGpCgyun7OoEOseryg",
            "UC0GLNhtTu7yI8rl6ZZGeezw",
            "UC0gmgfXC2hJWUW6VKeV4VKg",
            "UC0GnafipndAvpwSrrp7z0gw",
            "UC0gOw4iy-6HwO01q-gA1B0Q",
            "UC0gOWBkEuF4EF3XAi-iA-nQ",
            "UC0gQe6RljqpKt7_7hjNTCpQ",
            "UC0GqjJBjrhEezD_yOf6xphA",
            "UC0grOkRio0blRg9BWThENcA",
            "UC0GTmj2WNIuu_A_hc9ATs8Q",
            "UC0g_U1QjfhMR-kx8XLmWTCA",
            "UC0gVm_oe-5g8ujr-cK3pxpA",
            "UC0h07r_UgTD0Tc-Dn5XLX3g",
            "UC0H4NiCX97DBa1sgAA7Cluw",
            "UC0h4tQZ8hM4DxW6PN47sF6A",
            "UC0HAW8tgFA_xEmeUupuRwiA",
            "UC0HAYWxWD0RCnb8WRSiU_bw",
            "UC0hbVIbYqMNoI5oCntS7l4g",
            "UC0hCj45yAvlOqDu7a9rhK5A",
            "UC0HDXvx-D7Qh52qdZlwr1cw",
            "UC0hfppBlEFWGzFl_AaGBvMg",
            "UC0Hg2Ks00kCekyjZG_LxOmg",
            "UC0HJdIzniS_Ys5HLVKvGRQg",
            "UC0hJWKXgAiwixe_QOA-oZJA",
            "UC-_0HkJG3-5n5NxlxGyZsDw",
            "UC0HkNaD01K5VNzE87eMAEvw",
            "UC0hkSohaNzDlMrICE0FWWWw",
            "UC0HLIKKTN4RzwInxDPv5MmA",
            "UC0HowZa_wwVoBQRKO3bwCeg",
            "UC0hpPbLaIop8hCtIpL-xNtQ",
            "UC0hqDcdjH4bCX8q-cMCioKg",
            "UC0HR_6nw2RYMfX_fwzrWAIQ",
            "UC0HrD4cTsQpAZ8KK9jPzzGg",
            "UC0hviIe62WA24FbsJ5hxVZQ",
            "UC0hvOidzfFXnX8TCZNv8h6A",
            "UC0hVVwpx_zoLEuKI8bOgFdg",
            "UC0HwLU0nqctFyNJOu6giV0Q",
            "UC0hwSUgu98_uX_xuTELSKSg",
            "UC0HxyEc_ojRJ1oJXS5K6oaA",
            "UC0HzEBLlJxlrwBAHJ5S9JQg",
            "UC0-H-_zjh7yFN2w8Mi3ZT1A",
            "UC0HZOn5OXJXYnNCk7G7hUcA",
            "UC0i0wRQNBW5q5fsEnpdoUYQ",
            "UC0I2BZDYwckzcK87N4T__7Q",
            "UC0i2cOOC9NeadlbVB7wJ12g",
            "UC0i7t1CC7T0xheeBahaWZYQ",
            "UC0IdFAmdGU5J-Ec3G4oc7OQ",
            "UC0iDG4D-WrS39T5SRg1_31w",
            "UC0IecRQWKlSU7dMS4MOh7AA",
            "UC0IFwBxAc-9CweT4RGRQyPw",
            "UC0IIEZuaOermmnQCzudUo-g",
            "UC0IlEuu4TA9wq1u60tLMqBw",
            "UC0IM0soYjDEfXacWGUM1-tA",
            "UC0IMY7nkVsUimZDMAWWcDsg",
            "UC0ImYliYmbYY4yoUqHCxKkw",
            "UC0intLFzLaudFG-xAvUEO-A",
            "UC0iP7jCbOe3XAGZr6fMrUEw",
            "UC0IseOoIWidDXtjcCyiH5hA",
            "UC0iSmf2Yz2D7PCu7Q249H6Q",
            "UC0it1cu-mt_IcvLTQ4CWcHw",
            "UC0iTb2U2nWkm5X3XRityENw",
            "UC0iug0J48KcaQlHIc6bv7Gg",
            "UC0IVBdIkQ7CXG3Q4zRmwhQQ",
            "UC0iwHRFpv2_fpojZgQhElEQ",
            "UC0iwoNM6evwwnegM2Y7gq4A",
            "UC0IY1BQiMehWMvezqWLyk4g",
            "UC0Ize0RLIbGdH5x4wI45G-A",
            "UC_0j0So_2n2yxLLFPhrD3Cg",
            "UC0J1CwKp2LTEXf9qLorBbkg",
            "UC0j1MpbiTFHYrUjOTwifW_w",
            "UC_0J8E3uVSmVm8Cr97YBOHA",
            "UC0JAcvt-Q5GSoSMr8NIOS4A",
            "UC0JB7TSe49lg56u6qH8y_MQ",
            "UC0jC0_-39GTyFSMpsg1IC1Q",
            "UC0Jc5HSSs3UP-YnfuGfBg9Q",
            "UC0jD1_OI86HNoFfur4c3nHw",
            "UC0JgyEEYNL4ekWupqpmv1Bg",
            "UC0jIctUPBK6lHw4AYnGHvCA",
            "UC0jih6biL7GICfBXiOknIaQ",
            "UC0Jkqm37e5cmDZCp7y6BDDA",
            "UC0Jm3RQtmtwxxTc2gZkqDNw",
            "UC0JmOGkqA4VYbMmBYOszT4w",
            "UC0JmuCig0FDntUL_yXsHHYg",
            "UC0joAa2geiCykT3t_9FAqdQ",
            "UC0JR9ZTJIvB9VX8xz7LyujA",
            "UC0jRczpEKxLbGOo45XGkrzg",
            "UC0jR-LY5IhpX0a4vIaQbWPQ",
            "UC0JSXBqfEM4PQt3s_zzaTog",
            "UC0JtZy5sUVCFhYoJRD7m_ew",
            "UC0jUFX1Oc6RCVIyv2wxe2Uw",
            "UC0JUkXAVVA4qWH1BQRs5N3A",
            "UC0JwRb8wApbyleLzFUxWTpA",
            "UC0k173Oca1nPZurW2ITHlYw",
            "UC0k238zFx-Z8xFH0sxCrPJg",
            "UC0k9PfNjEkzISBnSk43mzvQ",
            "UC0KAB8tVLOOOheC3IN0ktbA",
            "UC0KAFLxIiaR_FFNYDL3utGw",
            "UC0KaZd_ki4l2EUc1GY9u5Ew",
            "UC0KdD41_dzHdDF1b1bN4cSw",
            "UC0KEznRC9DLZAHr4bVOZr3A",
            "UC0KfjyvabuE2J-RBC6ko2Lw",
            "UC0KIh0ljAHCQL-3_TFQRqaQ",
            "UC0KkQzV-XvTLTToZbs2PrBw",
            "UC0KNnkNiNDOm4DgVrEOGUSA",
            "UC0KRQXMqzIkRkt7fzGlV2Qg",
            "UC0kSxCefZ9KG_5AfYwfb2Mg",
            "UC0kTe9yvmEDY6e-9BQy2noA",
            "UC0KU8F9jJqSLS11LRXvFWmg",
            "UC0Kv2PxEd8pS5omTGTz8LGQ",
            "UC0KwJ-ig7udwmMgGjZyR41A",
            "UC0KxdtZUmdroPZXkC8AcUUg",
            "UC0kYGyq4kzEAoolkOA6JhAA",
            "UC0kz9Pt1NY2Nf-rSFAqun3Q",
            "UC0L0PDO9kUS46JC1wEjLEbg",
            "UC0L1suV8pVgO4pCAIBNGx5w",
            "UC0L2rngjJh8Zb4U0vfqWnPQ",
            "UC0l4RhVzhsanM37MzPWunOw",
            "UC0l5_O1tyDxyLev5wkEjydw",
            "UC0laoKDQpA46SXxhwLR7gxA",
            "UC0ldS_HgPtmNTAnV_TQC0xA",
            "UC0LDvBBL3hwFsPMEzpdcICg",
            "UC0lfekBJlfMb5m-t2HG-ECw",
            "UC0l_FnZ6USWl-obEocIlaoA",
            "UC0lgfusYd1dSu92G37-A1Zw",
            "UC0LhAQls08wi1iNwI2Jny1g",
            "UC0LHEYTEAyndlUqRJYtBZEg",
            "UC0lhr4dvpfKJ0C3TPFXu3ww",
            "UC0Lhseg2kaIGTddl5HhLUvw",
            "UC0lkCn7D--uMrtwAXRj_cdw",
            "UC0lkYVyb-TC6OLl0HoTgXBQ",
            "UC0lLioyuZbOhaRgLcz4myoA",
            "UC0LNCcCM-t4kEV6p9lv_NCQ",
            "UC0l-qqiESy2hXquOKET1Ugw",
            "UC0lT9K8Wfuc1KPqm6YjRf1A",
            "UC0ltLFa4COGNNZa-XxJg1LA",
            "UC0LvBTc3ZCVc2NgY32DekqA",
            "UC0Lw4X_RLd4SeQKACx0jSGw",
            "UC0lxBRzTOFm38dR9w2uTpGg",
            "UC0LYWI1aKF9lihYiJlShU8w",
            "UC0M0rxSz3IF0CsSour1iWmw",
            "UC0M1k0PpjCXJR2xDEfD6Jlw",
            "UC0M4Ze6hxg20b8zeBB6GdjQ",
            "UC0MAAi_gjlf0kcaQ0IZiQwA",
            "UC0mfQ-DHb9iBU5VaKvwFf0g",
            "UC0mGjOMXvTEDPzHr0WLtwrQ",
            "UC0ML1BOQVPYv6GLOz854S7w",
            "UC0Mlls1-bCBxyQqXPk_Q6vQ",
            "UC0mM1OD-647BIe_nnkBwHyQ",
            "UC0Mr9LCioW3VfGexsz6VlGw",
            "UC0MuLo15FvfMIx4YCYAxXWA",
            "UC0MvKDF6mSYV3EMgi2wcWpQ",
            "UC0MvmT6rqlISD-1AWItPwAA",
            "UC0MXm9gTSXnAOfIlMlsk9Ww",
            "UC0N0KFHRS2o2lVezdeHTBcg",
            "UC0n24lonL5RMpzmCiCzIkIw",
            "UC0N2mghwqb4UzPGqE8yIX-Q",
            "UC0N4yNxj0RhuQTVctdlBzDg",
            "UC0n5su-W8t6OdO6znoL7THQ",
            "UC0n5xS8E-1xinQI45p1_Lpw",
            "UC0n7hqQFFEZnwxrcVh09uVA",
            "UC0n7Qp-Gmf3PzbhyaAYPnzQ",
            "UC0NaWwWghJD37rMK2AYDidg",
            "UC0Nc1Czq_KeJg54qY3CBkRQ",
            "UC0NCbj8CxzeCGIF6sODJ-7A",
            "UC0nChSOqQbA6tAi8_K7pD_A",
            "UC0nCpDDnDNsrfmIFsk40VLA",
            "UC0NEFJxGzqMUVzh7b0sc1Uw",
            "UC0nf3IVbDsodz_BBPtuUi7g",
            "UC_0NfufarVw04vDfWFm8z_Q",
            "UC0NjuaCHCtMt19y8U_eoO1g",
            "UC0NL1uCU3oYaIx_mRXhK7EQ",
            "UC0Nmdo1DVRHkuFA7QTthXGw",
            "UC0nOQ1R3Z-vRO7K6g-W7Jkg",
            "UC0nqWBKwrcxRSwTjjCOh_4g",
            "UC0NUcM8piNmnWhBtAlBrZyg",
            "UC0NWMNdyGGOqGnNMj9EXfeg",
            "UC0NwzCHb8Fg89eTB5eYX17Q",
            "UC0-nZ9dKQroaCr2MuAXeqyg",
            "UC0nZK_PW1XBgy4czaOnXHDQ",
            "UC0o3RUJeapfDZt2BqdPNg9Q",
            "UC0OCq2D_eKKIip19iFy2ljA",
            "UC0odmP6ffEw3iVwTEyt-UKA",
            "UC0oeLSPx__hMggLznMByDJQ",
            "UC0ofE7KDHEyaW94_YJiqrxw",
            "UC0oH5dI0f2bfZ2cPqD6jC7w",
            "UC0ohl1eKTtlJNqbD-HWRQVg",
            "UC0oI44ZUVCqq4meNOOaKPTQ",
            "UC0OiowQ-fTqw3Kq3bfopO_Q",
            "UC0oMPVfY9obEIkFaceKkwUg",
            "UC0oMYbkaO_bFN4Fh4J0LpYg",
            "UC0OnAjC52vtL_N3f76BU9dw",
            "UC0OnreqP55xLpA6W5nzxb5Q",
            "UC0oNWf8yB_p1xl3J2tj75ww",
            "UC0oObx_f7G_FVcJgOvujtIQ",
            "UC0ORyZxzeuDL6-XYeVVO1dQ",
            "UC0o_ULEdwva8ksC63qvxMAA",
            "UC0ovr76ue_IsMZMfqkNeTrA",
            "UC0OwoVriT0qQSRDyei20k7g",
            "UC0ox9NuTHYeRys63yZpBFuA",
            "UC0Oxxd2GlSjFoPHHXcQjODA",
            "UC0-oyl2hNpAShq-LHAYqr7A",
            "UC0p23Kelkf1oG2ErPnH-YKw",
            "UC0p2bdv0ZutbRZZg9D7gDcg",
            "UC0p2OAXVoo9M1Z126B3fe3w",
            "UC0p3rxtSGCnO-JjBz5bU5CQ",
            "UC0p5jTq6Xx_DosDFxVXnWaQ",
            "UC0PaoSxEK2nEmO-Mtk2vGWA",
            "UC0pATpD1LWs_L6XWeEjOR0g",
            "UC0patpmwYbhcEUap0bTX3JQ",
            "UC0pBauLp63yzf6sVdEOIUbA",
            "UC-0PBESd8vrkdAbd4csUzSw",
            "UC0PCEbpfHK8JHcX1ZrkQiow",
            "UC0pCsHlEEmCfxllZSlRB2Og",
            "UC0pdktx9M6EcOsRg5LdLlXg",
            "UC0P-ec6f8-oeueSf84RQZZg",
            "UC0pH88OuizewtTgDhuEdkDQ",
            "UC0pHpxSt_4gd63WylQL0cVQ",
            "UC0pHUMEOtul5NlaT-Rt-34w",
            "UC0pItfAzEqkgPIM01dWPPug",
            "UC0_PjDd7_yRKJfUFR-Dgw7w",
            "UC-0pkBeaC8lZowrcWs4P6sg",
            "UC0PLiCyZI7xXImDS9ZXLtPA",
            "UC0pMaTmLXjKMOLGqfXgospA",
            "UC0pmMCHEaIprpNrpGLAZOgg",
            "UC0PMQXAwF6O6aeTpv962miA",
            "UC0_pmzgQnvxX58YVSRt66Uw",
            "UC0pqB9kjDxQi4TKgKoxyeDw",
            "UC0pQs7ooKe3nHswzQfnK0yw",
            "UC0prtgxOikcgkc_2B3BbEqA",
            "UC0PsUG07plnP_uQQS9djFVQ",
            "UC0pTNUlwUFLyLsKiCFzySjA",
            "UC0PusNwH9ZIlhsWoVhWLBOQ",
            "UC0PXqiud6dbwOAk8RvslgpQ",
            "UC0pzFp7mX5PISkJSlp2HwwA",
            "UC0Q3_mNzRm7IAQ1eZt1ETRw",
            "UC0-Q-6GiLDLrAIkTqNrNr-Q",
            "UC0Q7Hlz75NYhYAuq6O0fqHw",
            "UC0q9RG5eel-tnak5T4NpMDw",
            "UC0QbcOX2gI5zruEvpSmnf6Q",
            "UC0qBeNneZtK_dUTILdn-QVA",
            "UC0Qc7-N8Z1VLLM4giiWkOJQ",
            "UC0QEucPrn0-Ddi3JBTcs5Kw",
            "UC0QHWhjbe5fGJEPz3sVb6nw",
            "UC0Q_jgccqVpyvaL177PEAzA",
            "UC0Qju82LW7749gOo6UgrliQ",
            "UC0qkoqJljrrMEhPO1FXCc0w",
            "UC0QnzkNHxrOq7eDLIjxxyDw",
            "UC0QOUoPVvqdA2bzX0SXzAdw",
            "UC0QpXwNHik1793EYNGZcOoA",
            "UC0QqYnPsd8MrPq-Nrfr5rgg",
            "UC0QrYO0lxSrel6Zz8W7roWQ",
            "UC0Qt8F4hbIsxQ_I-dQaNd7w",
            "UC0QuCui5pNF9k9fiNXkqn_w",
            "UC0-QuVLhCc4hmJaiLSCMwcw",
            "UC0qWcfyDpQXLq1AvrJKhXIg",
            "UC0QwGb6erlvTQE_bFbrGmKw",
            "UC0q-wlyfFsLOX_jJhKquLUw",
            "UC0qYa1oZIlJb_gakTNJNK5Q",
            "UC0qyk0UvaN1z80BQdVQM85g",
            "UC0Qz-QnUPboliP4NjGi9w4A",
            "UC_0r3EheCnp-wVvndYDGviQ",
            "UC0r6rOR7ZjSY8ATlwrnaETw",
            "UC0R7Bx0CwBgxZlx_6QTSjtg",
            "UC0r_-7jgl1yl0oWzGEJ6o3w",
            "UC0R_-7yQPoGpkPR9ITzDFFQ",
            "UC0r8LqyA5qccr2qB7Fgnttg",
            "UC0r96i8wMM5kJNnYNI95_mw",
            "UC0R9l-10gynk4PihK8edYpA",
            "UC0raexzbPdlZlK-y0imNybg",
            "UC0R-biFFkI-3CAFZy_NM8Fg",
            "UC0RBJwg8ZfRM8TLGOKTpArA",
            "UC0RBqZrCnNFR6abYBeRQX1Q",
            "UC0RBTQIYLEQbcahZWkmzeTQ",
            "UC0rcwmnZOGohIR35dPOwyGg",
            "UC0rDDvHM7u_7aWgAojSXl1Q",
            "UC0RfydqX6ErhTQn-_soyVAQ",
            "UC0rFZyc35e4rMpKcJswXwKQ",
            "UC0rgZGosPuTxo2sWM-8KLOg",
            "UC0RhatS1pyxInC00YKjjBqQ",
            "UC_0RHOCoe-FrrEu9GMAlL2w",
            "UC0Rjg-kUtUCEvdMYwPQTUSA",
            "UC0rNv0MdIm3XCyB5NkRkQUg",
            "UC0RO0RGDWZwRSMg_4AsUuYw",
            "UC0RO8Ex_8w1ZYp4VoCz7o5w",
            "UC0_ro8fbhqxem8ttKxWGlow",
            "UC0-ronCUQNwMIkouWQC5mJQ",
            "UC0rphoZhcdkMg8Cgv71Ekjg",
            "UC0rpPbtxnkegWkageb4eJFA",
            "UC0rqucBdTuFTjJiefW5t-IQ",
            "UC0rR8CeL90k2OQ2jtakoDdw",
            "UC0Rwxz4318EEQGHz_z58nVA",
            "UC0rZoXAD5lxgBHMsjrGwWWQ",
            "UC0rzsIrAxF4kCsALP6J2EsA",
            "UC0S3gopo6NQhRJfWNuLK69A",
            "UC0SaJHWefA7ey18l6LRPP6A",
            "UC0SCyBGeHX15hirOGbezkKQ",
            "UC0sG2N0Al-TT7ZRMmoFko1w",
            "UC0sHHy5A8VUTHE_ysXeCO2Q",
            "UC0sJKdOLV0jBYqA6jqAtkTg",
            "UC0SKQW7FXSPgsrwPrRRlOXQ",
            "UC0sL36b-Sc-MQYleWzktOIQ",
            "UC0sLQvUR1kV0HFp0KkeuSvg",
            "UC0SMGmU6X9zJBGCGarr7ODQ",
            "UC0SoJ-CLiAT6J_2aWM4TiWg",
            "UC0sowg7_1BCdNzE7CHLAOCw",
            "UC0Sq_s-xZ5G_muivOK17scA",
            "UC0suOBgRAZPCy5kD6IS23zw",
            "UC0sw6dimxEVCj-ak8scsfEQ",
            "UC0-swBG9Ne0Vh4OuoJ2bjbA",
            "UC0sx2UCRrPrZ_98J6oRtbwQ",
            "UC0sXjX_AWPdqnP7ND4ESZ7A",
            "UC0SYAB5AaDMeEQs11FbVzUA",
            "UC0SyhPy7VsMQCIYDszFaplQ",
            "UC0sytTpk1adR_cfjHhiJ08Q",
            "UC0szjylgdoG5SPykBz-IHFA",
            "UC0szxwii-u4CZkJIt2iu4_A",
            "UC0t0XeFvf8KQff1ALntgbzQ",
            "UC0T6MVd3wQDB5ICAe45OxaQ",
            "UC0T7tvy44mlQCjaTtparOZw",
            "UC0t9vh857fTnMIEVNYDroQw",
            "UC0TFalPRyuOWuOgVgm0Lg9Q",
            "UC-0THIvKRd6n7T2a5aKYaGg",
            "UC0t-HO1xkRAU3zOI9Bx9YUg",
            "UC0TIPxlCiCF1_lqsNtuxZgg",
            "UC0tK7FbLRl4jNbTswoSVDEw",
            "UC0tK9G9GMeWQzJJsVjBrEhQ",
            "UC0tly-7AhY5qJlGa_fDzXvw",
            "UC0tNjxAn1nQ8I9FlosGoBZQ",
            "UC0TnW9acNxqeojxXDMbohcA",
            "UC0toktujph3fSR51GMwB9Jg",
            "UC0ToswtJwEkApOQWL-SzLdw",
            "UC0tq0g5u2bo-TErZt7SJM6w",
            "UC0-TVddfhZk8pgtLqOSOzgQ",
            "UC0tXn1__HkU1KauwAXDb2vQ",
            "UC0TYCxolfXHwDWQRzkNV1Dg",
            "UC0tZXG5hDHwPk9CK_kS4gJQ",
            "UC0u7ZMWqkr7cKD_rvEXZUuQ",
            "UC0u9FKn2vM0HRhQx_eI5qwQ",
            "UC0u9soopPe3cLnd8z9VX2CA",
            "UC0UAB0vcEQw5bM3FA8oo8eA",
            "UC0uAeNHLLuAovOTHI-aOZiQ",
            "UC0u--alCXjlPkCHxM6d_KWw",
            "UC0UEHLshL2K-RaUmj9e1bEw",
            "UC0ueho2z6oNaRJezlzuhvXA",
            "UC0ufW3qv1DDE8ucnwK6jvTA",
            "UC0UiGT1q9em4wU4cb9fG-Kg",
            "UC0uiyGyneZQCsfjgXCr_HmQ",
            "UC0uJKUXiU5T41Fzawy5H6mw",
            "UC0ujXryUUwILURRKt9Eh7Nw",
            "UC0Uk8NnnFUtzzqGKmF54b6w",
            "UC0ulDfOIUVoZAhHPuCTiawg",
            "UC0ULkbHHJ2fj3bIDEbzy-8g",
            "UC0unTmz9AXc5-WT_1mIlg6w",
            "UC0UoMX5ed9Ax_up8lkM0N0g",
            "UC0uoP6ZxFvEIvOC7Qevco8Q",
            "UC-0Urs9tifl-7DDE1RLbBHQ",
            "UC0uTPqBCFIpZxlz_Lv1tk_g",
            "UC0Uu4XnRS1hiz3JCpNFIuUg",
            "UC0uvjXlpppwOWn69yWnViTA",
            "UC0uVZd8N7FfIZnPu0y7o95A",
            "UC0uy4HBuZxEIFyi8bwH_xWA",
            "UC0uzmCBMC34Co_xeYt0ziLw",
            "UC0UZVWdcnKcQOVVk8M0aY5w",
            "UC0v0QStmFF3uFch26JeQfcQ",
            "UC0v5vmsnLJzZll0NWE5IuVg",
            "UC0V8fUv0DB5Tq7uHdG7ixyw",
            "UC0v8JFEMhutwqW8EQe5dSEQ",
            "UC0vA960mPrQ1oroOqbIsR3w",
            "UC0vBXGSyV14uvJ4hECDOl0Q",
            "UC0VCN0pkJ-k6dVhdG9bbC1g",
            "UC0VcpXqaW4m_JsHVGlOqb-w",
            "UC0VdHy4M39D7tBxsS9PKxcg",
            "UC0vDTIi5YP1IVT7gZo0afQA",
            "UC0VdUGegQIGabf0-hUycRag",
            "UC0VeqhFeCCjtEms6iHhGaXQ",
            "UC0vFIgkGrbmfxKVhZ2hgMeg",
            "UC0VfpTZbpmqrmvfH5gvlADg",
            "UC0VG86SRO2X47HU20nQtDeA",
            "UC0VJr7Z-CAxtao2hEqbYsdw",
            "UC0vn8ISa4LKMunLbzaXLnOQ",
            "UC0VNRvMijMy6DtM9UgX46Og",
            "UC0VoI57B2_63MErt_1QBpxA",
            "UC0VonSTAfwiDWmQg_W_dKwQ",
            "UC0VOyT2OCBKdQhF3BAbZ-1g",
            "UC0vQW0xzUyKZGKFHzvDBd6g",
            "UC0vrEYGvjJ7iy7M2xD_qJvg",
            "UC0vSB46MmHTwwxQrAo62Dgg",
            "UC0vT2ZuZxpNSnJ5AO0FVqRw",
            "UC0v-tlzsn0QZwJnkiaUSJVQ",
            "UC0VVYtw21rg2cokUystu2Dw",
            "UC0VXFoOwCjfw_WdZtr6Draw",
            "UC0VyVZyDoJULrseFdmGzlNQ",
            "UC0w18B1p3-S89hrcgAftZlg",
            "UC0W3mD7KA-NW123wbUeHYOg",
            "UC_0W4CAOlXt3bmRPsxRpx3g",
            "UC0W4CEoQoVEAJAn9rlkGErw",
            "UC0w4VRGCSmFw26mLsMPO7mg",
            "UC0W6NYJ_y7cd3laVfaSOl0A",
            "UC0W84vlavGml--MZvASw6tw",
            "UC0W_BIuwk8D0Bv4THbVZZOQ",
            "UC0WBZp-YpmPF23-JV9-1xWw",
            "UC0wcVu0_LO-CZEULdWizKBQ",
            "UC0WhB6jq9VwXCIT_SIDMbvA",
            "UC0WMQTG_-WIWm8eacM8D8QQ",
            "UC0wNSTMWIL3qaorLx0jie6A",
            "UC0WNX0de8I1fgKUrjxiPaRA",
            "UC0woBco6Dgcxt0h8SwyyOmw",
            "UC0woU0rUu5YFZihzqAuCr6A",
            "UC0wPu3VP8zQ0l5kCJQC6Jog",
            "UC0wpvMuWLzIrThUm2qTKINQ",
            "UC0wqQ_xK8OB8X6BdKNdu5Kw",
            "UC0wRBOS3bQnssqTThBONLVw",
            "UC0Wu8k-3Yc6tro6KYmLZRvQ",
            "UC0WUfJjRy0d7JZQAWSqY35g",
            "UC0wVKVxtxs19WTghmDSjn1Q",
            "UC0WwPp42pQQKRLVId6Flm5g",
            "UC0Wy5tgAHG0qLb-IBkREpjg",
            "UC0x0BVyBJB3Y9aW9H6bv12A",
            "UC0x0FBRBFZ00XefDuBvYLng",
            "UC0x18Q59vY8zPaPCStjJxtQ",
            "UC0x2jHkM8QgpIIvODyI-TRA",
            "UC0x6FKgrSbMghzvrhpEonKA",
            "UC0X6o-aELH_Rt3ZXLoKxe-w",
            "UC0xbzNitcynZOG9A2Rb_wPw",
            "UC0-xeMXO_Vu8_wqqKf_Nipg",
            "UC0XgzrSeLIgmHwT2uEZEexw",
            "UC0xhru07vavdPOqJxKRzPrg",
            "UC0XKvSq8CcMBSQTKXZXnEiQ",
            "UC0XNssyypOLiq4vVgXm9NtQ",
            "UC0xpbxEWFnFrkdFkTJuJFQw",
            "UC0xpHqXbZVFLdrspOVt4m5w",
            "UC0XQAoaynJb0KLC-I-FR1gQ",
            "UC0xTE0Ofe4A65ru3aFZ1nyg",
            "UC0xtW5OGRiNCpWg-7OcCp5w",
            "UC0xTyovpi5dsPyx2U-jlIPw",
            "UC0XUlpem_qZHbRWuMRJttjA",
            "UC0Xuyhv0xzZVYqkd_2oEh9A",
            "UC0_xwrhQI0iDqTZlHCaSWig",
            "UC0xwya64BPDXNk6stj9aNYA",
            "UC0XXvmAHHzCFNx5p2-HaqGA",
            "UC0XZbspdsegJA4l1P0u6b5w",
            "UC-0y359D7xSFbFPttu_OUHA",
            "UC0y4bYNyLl1ykJAAo5Wqr9g",
            "UC0Y-4IDptetodcD2sP2cndA",
            "UC0Y9r0sIXKHHmRlZgsca5Ag",
            "UC0YAfafei2jsZ-aPQOCDZZg",
            "UC0yAOoAaC2X9ZYcvLrPE20A",
            "UC0YbmGeNSGPd-StauM1qajg",
            "UC0yEgH2COTqGKu1G59r6BZA",
            "UC0yeWw3a8ahgrjU-CQb6IQw",
            "UC0YFjwJc3b1otDerdv6CfWA",
            "UC0YgFo6f08yACw_Ay6E9iGQ",
            "UC_0YI7QyEpMiYWC4v26MeOQ",
            "UC0yjEh3E8KytBFbKEKcT3Ig",
            "UC0yKWSMhIBV9jWxBEyriaHQ",
            "UC0ylCLsfZ6HqHK_wjgsPQhg",
            "UC0YlhwQabxkHb2nfRTzsTTA",
            "UC0ynJ0vIJFXCFWUZrt7D3pQ",
            "UC0yPCUmdMZIGtnxSnx5_ifA",
            "UC0yPWMDP3RK2Rpx2jz79_8g",
            "UC0yQ2h4gQXmVUFWZSqlMVOA",
            "UC0YRM4H3WRhm5dLRxFU1rJA",
            "UC0yS46bbGpjewt6D5P9-Y8Q",
            "UC0yseF9rIc5z8vHeP9lyj4Q",
            "UC0ySQ9t9qNFW3Pxizg6h2lg",
            "UC0YvoAYGgdOfySQSLcxtu1w",
            "UC0yYK93rpsHgqlY0l6owwBg",
            "UC0yz-15m7LglrrjaVpQ4yOQ",
            "UC0yzH9LnQSqkU1UFSLZEQ7w",
            "UC0z1R8kCc28E67hwZt3BwTA",
            "UC0z3Sq7ZC5y_4MDieYDgBbQ",
            "UC0Z60kCcQ8VIk5c29sPS9Jw",
            "UC0-Z8g7559Zy0wHf_ueMVlg",
            "UC0_-_zaPCpfBU-281QpiVTw",
            "UC0zArNuGZKdvzSkfHbR9yLA",
            "UC0_Z_cnIC0JFX4WJ7SZts9Q",
            "UC0zeVIr7SSI-8ObUloYBnnQ",
            "UC0ZGpaIfJX8xJ_gHI9Zqaww",
            "UC0zGwzu0zzCImC1BwPuWyXQ",
            "UC0ZJvV2yQEAvJIbMdFBDaGQ",
            "UC0zjyOB2R-7EjbWa3EpgkUw",
            "UC0zNh-ziiMzBZqTGx_YpKXw",
            "UC0zNoCMMiPEAst0JrwUht0w",
            "UC0zrazFd2Qx6iSODhr3tkqw",
            "UC_0zsIRj0eS8SGKnC7maPRw",
            "UC0zSnbaXNROSM42h1fLWSQA",
            "UC0ztHTPra8eacnTzzJGDRsw",
            "UC0ZTPkdxlAKf-V33tqXwi3Q",
            "UC0zUmHNpkviI6UZ0uqCYrww",
            "UC0ZV6M2THA81QT9hrVWJG3A",
            "UC0zYuwJf9-i3ROZ1F30gV4w",
            "UC0zzmN5qzSBt1GxHQ6N1_mg",
            "UC0ZZRdU4EnQOGLvLzl7niAw",
            "UC101o-vQ2iOj9vr00JUlyKw",
            "UC102Hjg6YvX8bOh3JWn-vJA",
            "UC103AffNL-a1mCd_9ZlZBLA",
            "UC103IUfx8ImU5-iJ1XkBudw",
            "UC104cyA6ZOBhEb56wjrZgWw",
            "UC105eLLmckO6a1yA-WbpWBQ",
            "UC106_-bHW0S6t9nmPOkKF-w",
            "UC109KgW_j4eazJXyOvIAAnw",
            "UC10BM9XdLdrvB8japwmRUvA",
            "UC10BuhxlV7Y53R0l9RJimmA",
            "UC10bxQVVz1dJSIUC-f5hJdA",
            "UC10c5BgJsUB_nLWx6pKG9yg",
            "UC10CwKWuw7CPpjCvYbIyzAw",
            "UC10Gx6iJaFzj7bBGBfAFe8A",
            "UC10lecpry6dGjTPb2AZpsxg",
            "UC10MkHLwWeqADmF5i_wrW8Q",
            "UC10Ot7i1OpwzlopW8Y6oQSQ",
            "UC-10uJUkzxSXa9sEtV67YcA",
            "UC10yOdRvVjDdKU8X1ybDmIg",
            "UC11APFj5uqbeRj5Ef6ertqw",
            "UC11fgYXjk4jVQeSqSEYJOag",
            "UC11j-ApkeIcxSTFtBYBMq3g",
            "UC11kHJNUV88Qp-Sz06N1o1A",
            "UC11KpQJPTJlEORUfQTMWQfg",
            "UC11-LdZRubZwoVid03rJv6A",
            "UC1_1mcW4AwdcDApXLzMge4A",
            "UC11O6ps6J-vlwXmKF-r2VWQ",
            "UC11OPzwn5Wt0-LN3rARunmg",
            "UC11PvrGPzo6Y7Zc6-e9cAKg",
            "UC11YXOTZw0d_GL8iVsRv5iw",
            "UC1205kBUDBfSfOuoG0f9PCg",
            "UC125_RjzBU43o-f-c6HT37g",
            "UC126kKAgMILMnVIi7HvQGlw",
            "UC127Qy2ulgASLYvW4AuHJZQ",
            "UC12k5BZ86YHoeFzOvow_how",
            "UC12LqyqTQYbXatYS9AA7Nuw",
            "UC12Qza6UDoiZfSmFkrmQCgA",
            "UC12VtiJbDpqWbTxl52HFg-g",
            "UC12zKGLhMhDeDidoctM6BrA",
            "UC13043Ga8_N3kItV22oHazQ",
            "UC130JquAAdwXyJ19M9YVX-w",
            "UC131VFZMVnJ9qlWB3C7glVA",
            "UC13h36xaBvyTPVAES4-4rXw",
            "UC13jOfkwQtqqcLN5eoFPeZQ",
            "UC13nPkrDQH0A2s2vx_NAVdQ",
            "UC13OYglkZpyJsJTrubCzfWw",
            "UC13phhDM8jVw_K-koiKyx-g",
            "UC13VU76vNh9MLMlxOEdqv9Q",
            "UC13x8ujr2JictFvUFITYyMA",
            "UC142-qvFgwSS6UIU-fFCd6w",
            "UC146bwNqyzGk9GlAzQyqgMg",
            "UC146qqkUMTrn4nfSSOTNwiA",
            "UC14AIyq_mGphI3gs8HMxXNA",
            "UC1-4AnMxkLXLNL3-7UJ4KTQ",
            "UC14ap4T608Zz_Mz4eezhIqw",
            "UC14eU3aFsoR1o_VCRmfYa3A",
            "UC14fB9eWBu579yRWLUhbFdQ",
            "UC14F-P_0a_0nEJBY67aP-1A",
            "UC_14iObZKvDchq0QvGE9fmA",
            "UC14NFFw9RlCbpXwr0X7hz-A",
            "UC14oQ42xP6JnPvO_4GVq6yQ",
            "UC1-4pA90sD0l0QtmQai7EcQ",
            "UC14pqQcFKmnfH5iimfXKTqA",
            "UC14QT5j2nQI8lKBCGtrrBQA",
            "UC14SYqAvsxvMq4JEMF3zSaQ",
            "UC14WBFNvmBI3ImFywII3iLg",
            "UC14YfshXdnxkD4qOTWke4eQ",
            "UC1532ZHC-nyxEgnqzPCyOGg",
            "UC15a4SVvvaC5rjLas6nyoow",
            "UC15BJjhPr4d5gTClhmC4HRw",
            "UC15ebOqdhmvl1eb0s0jk5aw",
            "UC15FhQd0x8dYPkFsH0bTsMg",
            "UC15iQ_QzTPxB6yGzzifJfKA",
            "UC15_kTR9wWnmqLacd8Q-8RQ",
            "UC15NqZuY_dw513dCMXgVyWA",
            "UC15pq102hQe7FnGq68gKHoQ",
            "UC15VpzK4og3NLmCVZQOroFw",
            "UC169v8urEwyw59gnKIHwY4A",
            "UC16BeXfcf_aMbSBOoticKsQ",
            "UC16bljMGmi9JYoyL5x9jLGg",
            "UC16_Cpn2JQvjOuztiL6_NsQ",
            "UC16djuch7iIZISxv3prmsyQ",
            "UC16fG7-summGsrcqkkYb6hg",
            "UC16fss-5fnGp2Drqp1iT9pA",
            "UC16H8wYkVdNtFmHNcZArvgQ",
            "UC16hCs7XeniFuoJq0hm_-EA",
            "UC16niRr50-MSBwiO3YDb3RA",
            "UC16NT3_Eu4xSiKzZnalxT5g",
            "UC16-RbXKoJaxmuwqWGg3QEQ",
            "UC172sJW4o6cwEbPD22ndx2g",
            "UC1733iBvNpXIscZk4QnfYBw",
            "UC173FCaSv6VvqDxuoinDr3g",
            "UC176GAQozKKjhz62H8u9vQQ",
            "UC177ji4tmDiLlW5iKyfJShA",
            "UC_178WGDYraYPAc2U1FXDzg",
            "UC17cRT7p8Eh_ytdGd_wG37A",
            "UC17ewSS9f2EnkCyMztCdoKA",
            "UC17_IYMcWqFdD7gqrX5eIWQ",
            "UC17JNY6VVLoF7wgpWyZOxWg",
            "UC17LqcboKOCmJm0weoe05Mw",
            "UC1-7mA0mKsCTyCMG4JNO3EQ",
            "UC17mJJnvzAa_e9qQqLIfIeQ",
            "UC_17P1ruDpCVujf7C_SxhOA",
            "UC17R_dRJCCipEHbJaIwp5Rw",
            "UC-17SkH3vCuLLfD6yz-XIaA",
            "UC17Uwo5BkSTtGVbOkywJ7_Q",
            "UC_17vC75UDfHgTJjX5Ravgw",
            "UC17vsYVoIwch5UzPar1LDmQ",
            "UC17XVgBYGLs-1EId-VQJxCQ",
            "UC185EHpMwN1v5c-phjWbn5A",
            "UC188KLMYLLGqVJZdYq7mYFw",
            "UC18C-SMbOJnRNrgiPjTrJMA",
            "UC18exdGWh7piVWisrnDXiZg",
            "UC18fv3yDege-nPN5zO48eHg",
            "UC18jDfM8UIqYfrYGJNStUag",
            "UC18ju52OET36bdewLRHzPdQ",
            "UC-18mFj7O9b1XbyGbyX6gZg",
            "UC18NaGQLruOEw65eQE3bNbg",
            "UC18PybGeqgjxAdTPd-eMADQ",
            "UC18QXRPIwjjxNvCj5YUxAcw",
            "UC18qytfIhR9cNEjUcgGLl3A",
            "UC18RieOQuuvjWcHADpZqOFQ",
            "UC18rtg_plpXBUsq5X5gcboQ",
            "UC18S2KrX6YYdQxfNb4uGUCw",
            "UC18vz5hUUqxbGvym9ghtX_w",
            "UC18YhnNvyrU2kTwCyj9p5ag",
            "UC190PvMt3RK7VPLOhgltEwg",
            "UC193r5YXcpQJV34N99ZbhzQ",
            "UC194cPvPaGJjhJBEGwG6vxg",
            "UC199d-cIUJFxMgK6XMWOW-w",
            "UC19EGSO3O3DC7KHYgGo5zxg",
            "UC19hjxMLgvLmr8f8hU1kZbw",
            "UC19i1XD6k88KqHlET8atqFQ",
            "UC19k51irpIIexAsHgU8iRAg",
            "UC19RvZaoKbsuZhXdomSNkdw",
            "UC19RzB56EDDiSZ3ukGc70Jw",
            "UC19s_YEQU9_tMQ-m59BsIzw",
            "UC19tEGPtxdnSMZnoqYUVvzA",
            "UC19tqkkWHvi6Qu81GDt82Lg",
            "UC19U0h0sS72el1ht5GjSOKg",
            "UC19u-fSHSQkvOt2xDQoAqaA",
            "UC19xLluI7dG093Gmw57BhHw",
            "UC1A0w492Xi2yNUHat9K9WAQ",
            "UC1A16dAPbDIoWT9wBGihOaQ",
            "UC1a2eXeDqSyFxrq1gKMJLRA",
            "UC1a2rnuwCw6rEbAxclIHkng",
            "UC1A70xg7qyQycQs8-pz8A6Q",
            "UC_1a7kjYz43wjLMxqfkG24A",
            "UC1a86oSvtZG8gz4d9BWzpJA",
            "UC1a-AF00U8RCaZiePMlgqUA",
            "UC1_aBBbg4vzI_Y8QAolPcCA",
            "UC1AD5L_bwNIWpLwZWVQ8_6g",
            "UC1AF30bJkIVzif2POp4JY7Q",
            "UC1Afj2bQJ3TdG5mggqMOyrA",
            "UC1AgWV7PLk_uVDkh35VVf9w",
            "UC1AhzkpXFX6_kAc7niTcc3w",
            "UC1Ak7Ir1WMOWauY_oH00-Qg",
            "UC1ANGnuKs1WB-eZBMv-1pyw",
            "UC1AoCwUpqPJtxMjt7sOohow",
            "UC1As3uk1ROhGfAhaT6B2_zA",
            "UC1AsDZxLEwjix1HzSOZhtsg",
            "UC1asQO41bDFNc2Le3-6IKUQ",
            "UC1At8WvjglVCC8TfSXit9Bw",
            "UC1Att7MG0JmaTY14L70tu8Q",
            "UC1-AV1dS3DoYdoY7vvTzHFA",
            "UC1AW267Tl_dJVHRBuC-OOlA",
            "UC_1awbvccFZOnVRjAIkCG7Q",
            "UC1AWYKpGNZhH699x1-2J4IQ",
            "UC1aXiUqUBeUpDC5ipyBj70g",
            "UC1axY_ac9OvG79ghqxxj9PA",
            "UC1axYBuo8spmzxzFtJGAD_w",
            "UC1B1LltEAWYIaZKFxU2yJLg",
            "UC1B3Dhu4PWXz7g39xkBxswA",
            "UC1b4TuSzgCg4pUBr6-pvuUw",
            "UC1B5ofDUrYUgVfh_HGDcV1g",
            "UC1b_aUzptvSaal89zzB2o3g",
            "UC1bBNTqAcdyUUfO7bPn2LAQ",
            "UC1bgOiYchyEu0bMri8Or7fw",
            "UC1B_JfwK3vkhm7VmB-3X_hA",
            "UC1BjrXqfmHmBkM2FjBTdg-w",
            "UC1bk_IRanUvKLTb_JmF_l8A",
            "UC1bL_jwVb4HGaae16_yNtbw",
            "UC1bMdy0JTGprmCbun6xLxjw",
            "UC1BMFYZxc1pBWw6bUnYkorw",
            "UC1Bm-HbJd2QmyzglNlJPa_Q",
            "UC1bmnUH1ffc63zohkPvtXcQ",
            "UC1bMtaXv-7LMXDJvF6WNcxw",
            "UC1BneC0eiVdR3YGfWq-TW6g",
            "UC1bpPvbeB-s3OLa9DoR4jxw",
            "UC1bqs6hSCsYepjeRNUSwcqg",
            "UC1bwliGvJogr7cWK0nT2Eag",
            "UC1BWMtZbNLVMSFgwSukjqCw",
            "UC1ByBSFKxTtV4JNRs6Ch8gw",
            "UC-1c7ebjoZoh1yTM6qL3R7g",
            "UC1caN5JbCSiNI9m8bL3cFUA",
            "UC1cbIkRS5-QfXfgKij2n11Q",
            "UC1Cbp8Fjy7iMfjp0jzGY3rQ",
            "UC1CchA0SjApw4T-AYkN7ytg",
            "UC1CFfZCWz8xMU683hkLTtfw",
            "UC1cfTDDO6vGOviS7Tw4R3sQ",
            "UC1CfXB_kRs3C-zaeTG3oGyg",
            "UC1CHH3kE_FmEZWfYJCK0s_w",
            "UC1CL9_o8c7sXfiMWJuTLp-Q",
            "UC1CLnXBqQKYTWXys9GbYMUA",
            "UC1Cp5caGwHvJZxHoevGoDdA",
            "UC1cpad0b9MBEUyH0ITXkJcA",
            "UC1Cp_XvaV5PLLMZS-r7E78w",
            "UC1cpzJUpWqih2i1FBXWJ1gQ",
            "UC1cQzKmbx9x0KipvoCt4NJg",
            "UC1Crbon9_pHDEXm5aKHTeAQ",
            "UC1CSCMwaDubQ4rcYCpX40Eg",
            "UC1CSl_vchvU8Iv6O8k0S0hA",
            "UC1CSVKVkJHXkPy56CcaoNWQ",
            "UC1CTpswOmqaxN5NXKVD1dUg",
            "UC1CVzH-XVr3E-kTT6D8hhfg",
            "UC1CWk_8OvBCWW9IZFcxHokQ",
            "UC1D26ByrbZVwuMgjVWmsOuw",
            "UC1D3yD4wlPMico0dss264XA",
            "UC1D57f34_TgI04OU7zF9MAA",
            "UC1DbpEM6ve_ugCGwRVRmAMA",
            "UC1DCedRgGHBdm81E1llLhOQ",
            "UC1DcHQ2znwMdf_EZqYWK-9A",
            "UC1DCPS2j-o0bvfIilkc8RSw",
            "UC1DeK07P-BoQLaI61Dt__Og",
            "UC1dH33m7DoQxS4cSZ4Vs5hg",
            "UC1dI4tO13ApuSX0QeX8pHng",
            "UC1DIvHYKNW_R9nej-9wc6Gw",
        ];
        let params = vec!["EgZ2aWRlb3O4AQDyBgQKAjoA","EglwbGF5bGlzdHO4AQDyBgQKAkIA","Egljb21tdW5pdHm4AQDyBgQKAkoA","EgVhYm91dLgBAPIGBAoCEgA%3D",""];
        let client_config = default_client_config();
        for param in params{          
          for channel_id in channel_ids.iter() {
            let j: serde_json::Value = match endpoints::browse_browseid(channel_id, param, &client_config).await{
                Ok(value) =>value,
                Err(_) => continue,
            };
            let result: Result<BrowseResult, serde_json::Error> = serde_json::from_value(j.to_owned());
            match result{
              Ok(result) => {}
              Err(err) => {
                let mut log = String::from(&format!("---------------------------------------------------- \n Browse ID: {} \n params: {} \n Error {} \n ----------------------------------------------------", channel_id,param, err));
                log += &j.to_string();
                match fs::create_dir(format!("channels/{}",channel_id)){
                  Ok(_) => {},
                  Err(_) => {},
                }
                fs::write(format!("channels/{0}/json_dump_endpoint_browse_{0}_{1}_.json",channel_id,param), log).unwrap();
              }
            }
          }
        }
    }
}
