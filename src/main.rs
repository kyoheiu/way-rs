mod html;

use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use axum::{
    routing::{get, post},
    Router,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey, VerifyingAlgorithm};
use serde::Deserialize;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "way_auth";

type HmacSha256 = Hmac<Sha256>;

struct Core {
    key: HmacSha256,
}

impl Core {
    fn default() -> Self {
        Core {
            key: HmacSha256::new_from_slice(
                env::var("WAY_SECRET_KEY")
                    .unwrap_or_else(|_| "some_secret".to_string())
                    .as_bytes(),
            )
            .unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct LogIn {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let core = Arc::new(Core::default());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/auth", get(auth))
        .route("/api/login", post(login))
        .route("/logout", get(logout))
        .layer(CookieManagerLayer::new())
        .with_state(core);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:9090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn health() -> Html<&'static str> {
    Html("Hello, developer.")
}

#[debug_handler]
async fn index(cookies: Cookies, State(core): State<Arc<Core>>) -> Html<&'static str> {
    if verify(cookies, &core.key) {
        println!("logged in");
        Html(html::VERIFIED)
    } else {
        println!("not verified");
        Html(html::INDEX)
    }
}

#[debug_handler]
async fn auth(
    Query(params): Query<BTreeMap<String, String>>,
    cookies: Cookies,
    State(core): State<Arc<Core>>,
) -> impl IntoResponse {
    if verify(cookies, &core.key) {
        (StatusCode::OK, "Verified").into_response()
    } else {
        let rf = params.get("ref").unwrap();
        Redirect::to(&format!("/?ref={}", rf)).into_response()
    }
}

#[debug_handler]
async fn login(
    Query(params): Query<BTreeMap<String, String>>,
    cookies: Cookies,
    State(core): State<Arc<Core>>,
    Form(log_in): Form<LogIn>,
) -> impl IntoResponse {
    let username = log_in.username.trim();
    let password = log_in.password.trim();
    println!("{}", env::var("WAY_USERNAME").unwrap());
    println!("{}", env::var("WAY_PASSWORD").unwrap());
    if username == env::var("WAY_USERNAME").unwrap().trim()
        && password == env::var("WAY_PASSWORD").unwrap().trim()
    {
        let mut claims = BTreeMap::new();
        claims.insert("sub", env::var("WAY_SECRET_SUB").unwrap_or_default());
        println!("{}", env::var("WAY_SECRET_SUB").unwrap());
        let token_str = claims.sign_with_key(&core.key).unwrap();
        let cookie = Cookie::build(COOKIE_NAME, token_str)
            .domain("localhost")
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);

        if let Some(rf) = params.get("ref") {
            Redirect::to(rf).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    } else {
        Redirect::to("/").into_response()
    }
}

#[debug_handler]
async fn logout(cookies: Cookies) -> impl IntoResponse {
    if cookies.get(COOKIE_NAME).is_some() {
        cookies.remove(Cookie::new(COOKIE_NAME, ""));
        println!("cookie removed");
    }
    Redirect::to("/").into_response()
}

fn verify(cookies: Cookies, key: &impl VerifyingAlgorithm) -> bool {
    if let Some(jwt) = cookies.get(COOKIE_NAME) {
        let claims: Result<BTreeMap<String, String>, jwt::error::Error> = jwt.value().verify_with_key(key);
        claims.is_ok() && claims.unwrap()["sub"] == env::var("WAY_SECRET_SUB").unwrap_or_default()
    } else {
        false
    }
}
