use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedditUser {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub is_employee: bool,
    pub is_friend: bool,
    pub subreddit: Subreddit,
    pub snoovatar_size: Value,
    pub awardee_karma: i64,
    pub id: String,
    pub verified: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub awarder_karma: i64,
    pub has_verified_email: bool,
    pub icon_img: String,
    pub hide_from_robots: bool,
    pub link_karma: i64,
    pub pref_show_snoovatar: bool,
    pub is_blocked: bool,
    pub total_karma: i64,
    pub accept_chats: bool,
    pub name: String,
    pub created: f64,
    pub created_utc: f64,
    pub snoovatar_img: String,
    pub comment_karma: i64,
    pub accept_followers: bool,
    pub has_subscribed: bool,
    pub accept_pms: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subreddit {
    pub default_set: bool,
    pub user_is_contributor: bool,
    pub banner_img: String,
    pub allowed_media_in_comments: Vec<Value>,
    pub user_is_banned: bool,
    pub free_form_reports: bool,
    pub community_icon: Value,
    pub show_media: bool,
    pub icon_color: String,
    pub user_is_muted: Value,
    pub display_name: String,
    pub header_img: Value,
    pub title: String,
    pub previous_names: Vec<Value>,
    pub over_18: bool,
    pub icon_size: Vec<i64>,
    pub primary_color: String,
    pub icon_img: String,
    pub description: String,
    pub submit_link_label: String,
    pub header_size: Value,
    pub restrict_posting: bool,
    pub restrict_commenting: bool,
    pub subscribers: i64,
    pub submit_text_label: String,
    pub is_default_icon: bool,
    pub link_flair_position: String,
    pub display_name_prefixed: String,
    pub key_color: String,
    pub name: String,
    pub is_default_banner: bool,
    pub url: String,
    pub quarantine: bool,
    pub banner_size: Value,
    pub user_is_moderator: bool,
    pub accept_followers: bool,
    pub public_description: String,
    pub link_flair_enabled: bool,
    pub disable_contributor_requests: bool,
    pub subreddit_type: String,
    pub user_is_subscriber: bool,
}
