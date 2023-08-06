use std::fs::File;
use std::io::Read;

use log::error;
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
use log::warn;
use reqwest::Client;
use std::io::Write;
use webbrowser;

mod serializables;
use serializables::auth::*;
use serializables::subreddit_posts::*;
use serializables::user::*;

mod auth;
use auth::constants::*;

use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

const REFRESH_TOKEN_FILE: &str = ".refresh_token";

#[derive(PartialEq)]
enum GrantType {
    AuthCode,
    RefreshToken,
}

use anyhow::Result;

const PARAMS_GRANT_TYPE_AUTH_CODE: &str =
    "grant_type=authorization_code&code={CODE}&redirect_uri=http://{REDIRECT_URI}";
const PARAMS_GRANT_TYPE_REFRESH_TOKEN: &str = "grant_type=refresh_token&refresh_token={CODE}";

const REDIRECT_SERVER: &str = "127.0.0.1:8080";
const REDDIT_URL: &str = "https://www.reddit.com";
const REDDIT_API_BASE_URL: &str = "https://oauth.reddit.com";

const API_AUTHORIZE: &str = concat!(
    "/api/v1/authorize?client_id={CLIENT_ID}&response_type=code",
    "&state={NONCE}&redirect_uri=http://{REDIRECT_SERVER}&duration=permanent&scope=read"
);
const API_PATH_ACCESS_TOKEN: &str = "/api/v1/access_token";
const API_PATH_SUBREDDIT_POSTS: &str = "/r/{SUBREDDIT}/new";
const API_PATH_SUBREDDIT_USER: &str = "/user/{REDDIT_USER}/about";

#[derive(Clone)]
struct AppState {
    nonce: String,
    client: Client,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let state = AppState {
        nonce: thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect(),
        client: reqwest::Client::new(),
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .with_state(state.clone());

    let f = File::open(REFRESH_TOKEN_FILE);

    if let Ok(mut f) = f {
        info!(
            "Attempting to read refresh token from file [{}]",
            REFRESH_TOKEN_FILE
        );
        let mut refresh_token = String::new();
        if let Ok(_num_bytes_read) = f.read_to_string(&mut refresh_token) {
            if !refresh_token.is_empty() {
                let access_token = if let Ok(auth_token) =
                    get_and_cache_token(&state.client, &refresh_token, GrantType::RefreshToken)
                        .await
                {
                    auth_token.access_token
                } else {
                    error!("Error obtaining the access token");
                    return;
                };

                if let Ok(users) = get_subreddit_post_users(&state.client, &access_token).await {
                    info!("{}", users);
                } else {
                    error!("get_subreddit_post_users failed");
                }
                return;
            }
        }
    }

    let auth_url = format!(
        "{REDDIT_URL}{API_AUTHORIZE}",
        REDDIT_URL = REDDIT_URL,
        API_AUTHORIZE = API_AUTHORIZE
            .replace("{CLIENT_ID}", CLIENT_ID)
            .replace("{NONCE}", &state.nonce)
            .replace("{REDIRECT_SERVER}", REDIRECT_SERVER)
    );

    info!("Redirecting user to URL=[{}]", auth_url);
    webbrowser::open(auth_url.as_str()).unwrap();

    info!("Server starting at [{}] ...", REDIRECT_SERVER);

    axum::Server::bind(&REDIRECT_SERVER.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_subreddit_post_users(client: &Client, access_token: &String) -> Result<String> {
    let subreddit_posts = oath_get(
        &client,
        API_PATH_SUBREDDIT_POSTS.replace("{SUBREDDIT}", "gaming"),
        &access_token,
    )
    .await?;
    let subreddit_posts: SubredditPosts = serde_json::from_str(&subreddit_posts)?;

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
        .await?;
        let reddit_user: RedditUser = serde_json::from_str(&reddit_user)?;
        html += format!(
            r##"<p>{}</p>"##,
            reddit_user.data.subreddit.public_description
        )
        .as_str();
    }
    Ok(html)
}

async fn root(query: Query<AuthParams>, State(state): State<AppState>) -> Html<String> {
    info!(
        "Auth params retrieved: state=[{}], code=[{}]",
        query.state, query.code
    );

    if state.nonce != query.code {
        warn!("Invalid state ([{}] != [{}])", state.nonce, query.code);
    }

    let access_token = if let Ok(auth_token) =
        get_and_cache_token(&state.client, &query.code, GrantType::AuthCode).await
    {
        auth_token.access_token
    } else {
        return Html("Error obtaining the access token".to_owned());
    };

    Html(format!(
        "{:?}<h1>You can now close this browser tab...</h1>",
        get_subreddit_post_users(&state.client, &access_token).await
    ))
}

async fn oath_get(client: &Client, api_path: String, access_token: &String) -> Result<String> {
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
        .await?;

    let data = res.text().await?;

    info!("Obtained data [{}]...", &data[..100]);
    Ok(data)
}

async fn get_and_cache_token(
    client: &Client,
    auth_code: &String,
    grant_type: GrantType,
) -> Result<AuthToken> {
    let auth_request = if grant_type == GrantType::AuthCode {
        PARAMS_GRANT_TYPE_AUTH_CODE
            .replace("{REDIRECT_URI}", REDIRECT_SERVER)
            .replace("{CODE}", auth_code)
    } else {
        PARAMS_GRANT_TYPE_REFRESH_TOKEN.replace("{CODE}", &auth_code)
    };

    info!(
        "Sending auth request [{AUTH_REQUEST}]",
        AUTH_REQUEST = auth_request
    );

    let res = client
        .post(format!(
            "{REDDIT_URL}{API_PATH_ACCESS_TOKEN}",
            REDDIT_URL = REDDIT_URL,
            API_PATH_ACCESS_TOKEN = API_PATH_ACCESS_TOKEN
        ))
        .body(auth_request)
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .header(reqwest::header::USER_AGENT, "reqwest")
        .send()
        .await?;

    let resp_text = res.text().await?;
    info!("Retrieved auth info: [{:?}]", resp_text);
    let auth_token: AuthToken = serde_json::from_str(&resp_text)?;

    let mut f = File::create(REFRESH_TOKEN_FILE)?;
    write!(f, "{}", auth_token.refresh_token)?;
    Ok(auth_token)
}
