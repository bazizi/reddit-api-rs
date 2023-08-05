/*
Description:
    OAuth authorization to Reddit using Rust
Author: Behnam Azizi
Useful docs:
    https://github.com/reddit-archive/reddit/wiki/OAuth2#token-retrieval-code-flow
    https://www.reddit.com/prefs/apps
    https://www.reddit.com/dev/api/oauth#GET_subreddits_{where}
*/
use log::info;
use reqwest::Client;
use webbrowser;

mod serializables;
use serializables::auth::*;
use serializables::subreddit_posts::*;
use serializables::user::*;

mod auth;
use auth::constants::*;

use axum::{extract::Query, response::Html, routing::get, Router};

const REDIRECT_SERVER: &str = "127.0.0.1:8080";
const REDDIT_URL: &str = "https://www.reddit.com";
const REDDIT_API_BASE_URL: &str = "https://oauth.reddit.com";

const API_PATH_ACCESS_TOKEN: &str = "/api/v1/access_token";
const API_PATH_SUBREDDIT_POSTS: &str = "/r/{SUBREDDIT}/new";
const API_PATH_SUBREDDIT_USER: &str = "/user/{REDDIT_USER}/about";

#[tokio::main]
async fn main() {
    env_logger::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    webbrowser::open(
        format!(
            concat!(
            "{REDDIT_URL}/api/v1/authorize?client_id={CLIENT_ID}&response_type=code",
            "&state=terminator&redirect_uri=http://{REDIRECT_SERVER}&duration=temporary&scope=read"
        ),
            REDDIT_URL = REDDIT_URL,
            CLIENT_ID = CLIENT_ID,
            REDIRECT_SERVER = REDIRECT_SERVER
        )
        .as_str(),
    )
    .unwrap();

    info!("Server starting at [{}] ...", REDIRECT_SERVER);

    axum::Server::bind(&REDIRECT_SERVER.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(query: Query<AuthParams>) -> Html<String> {
    info!(
        "Auth params retrieved: state=[{}], code=[{}]",
        query.state, query.code
    );

    let client = reqwest::Client::new();

    let access_token = get_token(&client, &query.code).await.access_token;

    let subreddit_posts = oath_get(
        &client,
        API_PATH_SUBREDDIT_POSTS.replace("{SUBREDDIT}", "gaming"),
        &access_token,
    )
    .await;

    let subreddit_posts: SubredditPosts = serde_json::from_str(&subreddit_posts).unwrap();

    let mut html = String::new();

    for subreddit_post in subreddit_posts.data.children {
        let subreddit_post = subreddit_post.data;
        html += format!(r##"<p>{}</p>"##, subreddit_post.author).as_str();
        html += format!(
            r##"<img src={}></img><br/><hr/>"##,
            subreddit_post.thumbnail
        )
        .as_str();

        let reddit_user = oath_get(
            &client,
            API_PATH_SUBREDDIT_USER.replace("{REDDIT_USER}", &subreddit_post.author),
            &access_token,
        )
        .await;
        let reddit_user: RedditUser = serde_json::from_str(&reddit_user).unwrap();
        html += format!(
            r##"<p>{}</p>"##,
            reddit_user.data.subreddit.public_description
        )
        .as_str();
    }

    Html(format!(
        "{:?}<h1>You can now close this browser tab...</h1>",
        html
    ))
}

async fn oath_get(client: &Client, api_path: String, access_token: &String) -> String {
    let api_full_url = format!(
        "{REDDIT_API_BASE_URL}{API_PATH}",
        REDDIT_API_BASE_URL = REDDIT_API_BASE_URL,
        API_PATH = api_path
    );

    info!("Attempting to call GET on URL=[{}]", api_full_url);

    let res = client
        .get(api_full_url)
        .header(reqwest::header::USER_AGENT, "reqwest")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token),
        )
        .send()
        .await
        .unwrap();

    let data = res.text().await.unwrap();

    info!("Obtained data [{}]...", &data[..100]);
    data
}

async fn get_token(client: &Client, auth_code: &String) -> AuthToken {
    let auth_request = format!(
        "grant_type=authorization_code&code={CODE}&redirect_uri=http://{REDIRECT_URI}",
        REDIRECT_URI = REDIRECT_SERVER,
        CODE = auth_code
    );

    info!(
        "Sending auth request [{auth_request}]",
        auth_request = auth_request
    );

    let res = client
        .post(format!("{REDDIT_URL}", REDDIT_URL = REDDIT_URL) + API_PATH_ACCESS_TOKEN)
        .body(auth_request)
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .header(reqwest::header::USER_AGENT, "reqwest")
        .send()
        .await
        .unwrap();

    let resp_text = res.text().await.unwrap();
    let auth_token: AuthToken = serde_json::from_str(&resp_text).unwrap();
    info!("Retrieved auth info: [{:?}]", auth_token);
    auth_token
}
