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
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

const COOKIE_NAME: &str = "way_auth";

struct Core {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Core {
    fn default() -> Self {
        Core {
            encoding_key: EncodingKey::from_secret(env::var("WAY_SECRET_KEY").unwrap().as_bytes()),
            decoding_key: DecodingKey::from_secret(env::var("WAY_SECRET_KEY").unwrap().as_bytes()),
        }
    }
}

#[derive(Deserialize)]
struct LogIn {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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
        .route("/api/logout", get(logout))
        .layer(CookieManagerLayer::new())
        .nest_service("/static", ServeDir::new("static"))
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
    if is_valid(cookies, &core.decoding_key) {
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
    if is_valid(cookies, &core.decoding_key) {
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
    if username == env::var("WAY_USERNAME").unwrap().trim()
        && password == env::var("WAY_PASSWORD").unwrap().trim()
    {
        let my_claims = Claims {
            sub: env::var("WAY_USERNAME").unwrap(),
            exp: 2000000000,
        };
        let token = encode(&Header::default(), &my_claims, &core.encoding_key).unwrap();
        let cookie = Cookie::build(COOKIE_NAME, token)
            .domain(env::var("WAY_DOMAIN").unwrap())
            .path("/")
            .max_age(cookie::time::Duration::days(7))
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
        let cookie = Cookie::build(COOKIE_NAME, "")
            .domain(env::var("WAY_DOMAIN").unwrap())
            .path("/")
            .max_age(cookie::time::Duration::seconds(0))
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);
        println!("cookie removed");
    }
    Redirect::to("/").into_response()
}

fn is_valid(cookies: Cookies, key: &DecodingKey) -> bool {
    if let Some(jwt) = cookies.get(COOKIE_NAME) {
        let claims = decode::<Claims>(jwt.value(), key, &Validation::new(Algorithm::HS256));
        claims.is_ok() && claims.unwrap().claims.sub == env::var("WAY_USERNAME").unwrap()
    } else {
        false
    }
}
