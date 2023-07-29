mod error;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::{debug_handler, Json};
use axum::{
    routing::{get, post},
    Router,
};
use cookie::SameSite;
use error::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;
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
    fn default() -> Result<Self, Error> {
        Ok(Core {
            encoding_key: EncodingKey::from_secret(env::var("WAY_SECRET_KEY")?.as_bytes()),
            decoding_key: DecodingKey::from_secret(env::var("WAY_SECRET_KEY")?.as_bytes()),
        })
    }
}

#[derive(Deserialize)]
struct LogIn {
    dn: String,
    passwd: String,
    redirect: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Res {
    links: Option<Vec<Link>>,
    redirect: bool,
}

#[derive(Serialize, Deserialize)]
struct Link {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    info!("Initialized log.");
    let core = Arc::new(Core::default()?);

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/auth", get(auth))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .layer(CookieManagerLayer::new())
        .nest_service("/", ServeDir::new("static"))
        .with_state(core);

    axum::Server::bind(&"0.0.0.0:9090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[debug_handler]
async fn health() -> Result<String, Error> {
    let mut result = String::new();
    let env = [
        env::var("WAY_SECRET_KEY"),
        env::var("WAY_NETWORK"),
        env::var("WAY_DOMAIN"),
    ];
    if env.iter().any(|x| x.is_err()) {
        result.push_str("(1/2) 🚨 Not all environment variables are set correctly.");
    } else {
        result.push_str("(1/2) 👍 Found all environment variables.");
    }
    result.push('\n');

    if let Err(_con) = ldap3::LdapConnAsync::new(&env::var("WAY_NETWORK")?.to_string()).await {
        result.push_str("(2/2) 🚨 Failed to connect to LDAP server: Check your `WAY_NETWORK` env variable or state of LDAP server.")
    } else {
        result.push_str("(2/2) 👍 Succeeded to connect to LDAP server.");
    }

    Ok(result)
}

#[debug_handler]
async fn auth(
    Query(params): Query<std::collections::BTreeMap<String, String>>,
    cookies: Cookies,
    State(core): State<Arc<Core>>,
) -> impl IntoResponse {
    if let Ok(_name) = is_valid(cookies, &core.decoding_key) {
        if let Ok(config) = std::fs::read_to_string("config.yml") {
            let links: Result<Vec<Link>, _> = serde_yaml::from_str(&config);
            match links {
                Ok(links) => Json(Res {
                    links: Some(links),
                    redirect: false,
                })
                .into_response(),
                Err(_) => Json(Res {
                    links: None,
                    redirect: false,
                })
                .into_response(),
            }
        } else {
            Json(Res {
                links: None,
                redirect: false,
            })
            .into_response()
        }
    } else {
        if let Some(rf) = params.get("ref") {
            Redirect::to(&format!("/?ref={}", rf)).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }
}

#[debug_handler]
async fn login(
    cookies: Cookies,
    State(core): State<Arc<Core>>,
    Json(log_in): Json<LogIn>,
) -> Result<impl IntoResponse, Error> {
    let username = log_in.dn.trim();
    let password = &log_in.passwd.trim();
    let (con, mut ldap) = ldap3::LdapConnAsync::new(&env::var("WAY_NETWORK")?.to_string()).await?;
    ldap3::drive!(con);
    if let Ok(_result) = ldap.simple_bind(username, password).await?.success() {
        info!("Logged in.");
        let my_claims = Claims {
            sub: username.to_string(),
            exp: 2000000000,
        };
        let token = encode(&Header::default(), &my_claims, &core.encoding_key)?;
        let cookie = Cookie::build(COOKIE_NAME, token)
            .domain(env::var("WAY_DOMAIN")?)
            .path("/")
            .max_age(cookie::time::Duration::days(7))
            .same_site(SameSite::None)
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);

        if let Ok(config) = std::fs::read_to_string("config.yml") {
            let links: Result<Vec<Link>, _> = serde_yaml::from_str(&config);
            match links {
                Ok(links) => Ok(Json(Res {
                    links: Some(links),
                    redirect: log_in.redirect,
                })
                .into_response()),
                Err(_) => Ok(Json(Res {
                    links: None,
                    redirect: log_in.redirect,
                })
                .into_response()),
            }
        } else {
            Ok(Json(Res {
                links: None,
                redirect: log_in.redirect,
            })
            .into_response())
        }
    } else {
        Err(Error::Login)
    }
}

#[debug_handler]
async fn logout(cookies: Cookies) -> Result<impl IntoResponse, Error> {
    if cookies.get(COOKIE_NAME).is_some() {
        let cookie = Cookie::build(COOKIE_NAME, "")
            .domain(env::var("WAY_DOMAIN")?)
            .path("/")
            .max_age(cookie::time::Duration::seconds(0))
            .same_site(SameSite::None)
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);
        info!("Cookie removed: Logged out.");
    }
    Ok(StatusCode::OK)
}

fn is_valid(cookies: Cookies, key: &DecodingKey) -> Result<String, Error> {
    if let Some(jwt) = cookies.get(COOKIE_NAME) {
        let claims = decode::<Claims>(jwt.value(), key, &Validation::new(Algorithm::HS256));
        if let Ok(claims) = claims {
            Ok(claims.claims.sub)
        } else {
            Err(Error::Io("error".to_string()))
        }
    } else {
        Err(Error::Io("error".to_string()))
    }
}
