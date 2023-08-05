use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubredditPosts {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub after: String,
    pub dist: i64,
    pub modhash: Value,
    pub geo_filter: String,
    pub children: Vec<Children>,
    pub before: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data2 {
    pub approved_at_utc: Value,
    pub subreddit: String,
    pub selftext: String,
    pub author_fullname: String,
    pub saved: bool,
    pub mod_reason_title: Value,
    pub gilded: i64,
    pub clicked: bool,
    pub title: String,
    pub link_flair_richtext: Vec<Value>,
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: i64,
    pub link_flair_css_class: Option<String>,
    pub downs: i64,
    pub thumbnail_height: Option<i64>,
    pub top_awarded_type: Value,
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    pub link_flair_text_color: Option<String>,
    pub upvote_ratio: f64,
    pub author_flair_background_color: Value,
    pub subreddit_type: String,
    pub ups: i64,
    pub total_awards_received: i64,
    pub media_embed: MediaEmbed,
    pub thumbnail_width: Option<i64>,
    pub author_flair_template_id: Option<String>,
    pub is_original_content: bool,
    pub user_reports: Vec<Value>,
    pub secure_media: Option<SecureMedia>,
    pub is_reddit_media_domain: bool,
    pub is_meta: bool,
    pub category: Value,
    pub secure_media_embed: SecureMediaEmbed,
    pub link_flair_text: Value,
    pub can_mod_post: bool,
    pub score: i64,
    pub approved_by: Value,
    pub is_created_from_ads_ui: bool,
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: Value,
    pub author_flair_css_class: Option<String>,
    pub author_flair_richtext: Vec<AuthorFlairRichtext>,
    pub gildings: Gildings,
    pub post_hint: Option<String>,
    pub content_categories: Value,
    pub is_self: bool,
    pub mod_note: Value,
    pub created: f64,
    pub link_flair_type: String,
    pub wls: i64,
    pub removed_by_category: Value,
    pub banned_by: Value,
    pub author_flair_type: String,
    pub domain: String,
    pub allow_live_comments: bool,
    pub selftext_html: Option<String>,
    pub likes: Value,
    pub suggested_sort: Value,
    pub banned_at_utc: Value,
    pub url_overridden_by_dest: Option<String>,
    pub view_count: Value,
    pub archived: bool,
    pub no_follow: bool,
    pub is_crosspostable: bool,
    pub pinned: bool,
    pub over_18: bool,
    pub preview: Option<Preview>,
    pub all_awardings: Vec<AllAwarding>,
    pub awarders: Vec<Value>,
    pub media_only: bool,
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    pub author_flair_text: Option<String>,
    pub treatment_tags: Vec<Value>,
    pub visited: bool,
    pub removed_by: Value,
    pub num_reports: Value,
    pub distinguished: Value,
    pub subreddit_id: String,
    pub author_is_blocked: bool,
    pub mod_reason_by: Value,
    pub removal_reason: Value,
    pub link_flair_background_color: Option<String>,
    pub id: String,
    pub is_robot_indexable: bool,
    pub report_reasons: Value,
    pub author: String,
    pub discussion_type: Value,
    pub num_comments: i64,
    pub send_replies: bool,
    pub whitelist_status: String,
    pub contest_mode: bool,
    pub mod_reports: Vec<Value>,
    pub author_patreon_flair: bool,
    pub author_flair_text_color: Option<String>,
    pub permalink: String,
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub url: String,
    pub subreddit_subscribers: i64,
    pub created_utc: f64,
    pub num_crossposts: i64,
    pub media: Option<Media>,
    pub is_video: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaEmbed {
    pub content: Option<String>,
    pub width: Option<i64>,
    pub scrolling: Option<bool>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecureMedia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub oembed: Oembed,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Oembed {
    pub provider_url: String,
    pub version: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub thumbnail_width: i64,
    pub height: i64,
    pub width: i64,
    pub html: String,
    pub author_name: Option<String>,
    pub provider_name: String,
    pub thumbnail_url: String,
    pub thumbnail_height: i64,
    pub author_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecureMediaEmbed {
    pub content: Option<String>,
    pub width: Option<i64>,
    pub scrolling: Option<bool>,
    pub media_domain_url: Option<String>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorFlairRichtext {
    pub e: String,
    pub t: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gildings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preview {
    pub images: Vec<Image>,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub variants: Variants,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variants {
    pub obfuscated: Option<Obfuscated>,
    pub nsfw: Option<Nsfw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obfuscated {
    pub source: Source2,
    pub resolutions: Vec<Resolution2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nsfw {
    pub source: Source3,
    pub resolutions: Vec<Resolution3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source3 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution3 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllAwarding {
    pub giver_coin_reward: Value,
    pub subreddit_id: Value,
    pub is_new: bool,
    pub days_of_drip_extension: Value,
    pub coin_price: i64,
    pub id: String,
    pub penny_donate: Value,
    pub award_sub_type: String,
    pub coin_reward: i64,
    pub icon_url: String,
    pub days_of_premium: Value,
    pub tiers_by_required_awardings: Value,
    pub resized_icons: Vec<ResizedIcon>,
    pub icon_width: i64,
    pub static_icon_width: i64,
    pub start_date: Value,
    pub is_enabled: bool,
    pub awardings_required_to_grant_benefits: Value,
    pub description: String,
    pub end_date: Value,
    pub sticky_duration_seconds: Value,
    pub subreddit_coin_reward: i64,
    pub count: i64,
    pub static_icon_height: i64,
    pub name: String,
    pub resized_static_icons: Vec<ResizedStaticIcon>,
    pub icon_format: String,
    pub icon_height: i64,
    pub penny_price: Option<i64>,
    pub award_type: String,
    pub static_icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResizedIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResizedStaticIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "type")]
    pub type_field: String,
    pub oembed: Oembed2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Oembed2 {
    pub provider_url: String,
    pub version: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub thumbnail_width: i64,
    pub height: i64,
    pub width: i64,
    pub html: String,
    pub author_name: Option<String>,
    pub provider_name: String,
    pub thumbnail_url: String,
    pub thumbnail_height: i64,
    pub author_url: Option<String>,
}
