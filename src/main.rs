mod error;

use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use axum::{
    routing::{get, post},
    Router,
};
use error::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::sync::Arc;
use tera::{Context, Tera};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

const COOKIE_NAME: &str = "way_auth";

struct Core {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    templates: Tera,
}

impl Core {
    fn default() -> Result<Self, Error> {
        Ok(Core {
            encoding_key: EncodingKey::from_secret(env::var("WAY_SECRET_KEY")?.as_bytes()),
            decoding_key: DecodingKey::from_secret(env::var("WAY_SECRET_KEY")?.as_bytes()),
            templates: Tera::new("templates/*")?,
        })
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

#[derive(Serialize, Deserialize)]
struct WayContext {
    name: String,
    links: Links,
}

#[derive(Serialize, Deserialize)]
struct Links(Vec<Link>);

#[derive(Serialize, Deserialize)]
struct Link {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let core = Arc::new(Core::default()?);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/auth", get(auth))
        .route("/api/ldaplogin", post(ldaplogin))
        .route("/api/logout", get(logout))
        .layer(CookieManagerLayer::new())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(core);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:9090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[debug_handler]
async fn health() -> Html<&'static str> {
    Html("Hello, developer.")
}

#[debug_handler]
async fn index(cookies: Cookies, State(core): State<Arc<Core>>) -> Result<Html<String>, Error> {
    if let Ok(name) = is_valid(cookies, &core.decoding_key) {
        println!("logged in");
        if let Ok(config) = std::fs::read_to_string("config/config.yml") {
            let links: Result<Links, _> = serde_yaml::from_str(&config);
            match links {
                Ok(links) => {
                    let context = WayContext { name, links };
                    Ok(Html(core.templates.render(
                        "verified.html",
                        &Context::from_serialize(context)?,
                    )?))
                }
                Err(_) => {
                    let context = WayContext {
                        name,
                        links: Links(vec![]),
                    };
                    Ok(Html(core.templates.render(
                        "verified.html",
                        &Context::from_serialize(context)?,
                    )?))
                }
            }
        } else {
            let context = WayContext {
                name,
                links: Links(vec![]),
            };
            Ok(Html(core.templates.render(
                "verified.html",
                &Context::from_serialize(context)?,
            )?))
        }
    } else {
        println!("not verified");
        let context = Context::new();
        Ok(Html(core.templates.render("index.html", &context)?))
    }
}

#[debug_handler]
async fn auth(
    Query(params): Query<BTreeMap<String, String>>,
    cookies: Cookies,
    State(core): State<Arc<Core>>,
) -> impl IntoResponse {
    if let Ok(_name) = is_valid(cookies, &core.decoding_key) {
        (StatusCode::OK, "Verified").into_response()
    } else {
        let rf = params.get("ref").unwrap();
        Redirect::to(&format!("/?ref={}", rf)).into_response()
    }
}

// #[debug_handler]
// async fn login(
//     Query(params): Query<BTreeMap<String, String>>,
//     cookies: Cookies,
//     State(core): State<Arc<Core>>,
//     Form(log_in): Form<LogIn>,
// ) -> impl IntoResponse {
//     let username = log_in.username.trim();
//     let password = log_in.password.trim();
//     if username == env::var("WAY_USERNAME").unwrap().trim()
//         && password == env::var("WAY_PASSWORD").unwrap().trim()
//     {
//         let my_claims = Claims {
//             sub: env::var("WAY_USERNAME").unwrap(),
//             exp: 2000000000,
//         };
//         let token = encode(&Header::default(), &my_claims, &core.encoding_key).unwrap();
//         let cookie = Cookie::build(COOKIE_NAME, token)
//             .domain(env::var("WAY_DOMAIN").unwrap())
//             .path("/")
//             .max_age(cookie::time::Duration::days(7))
//             .secure(true)
//             .http_only(true)
//             .finish();
//         cookies.add(cookie);
//
//         if let Some(rf) = params.get("ref") {
//             Redirect::to(rf).into_response()
//         } else {
//             Redirect::to("/").into_response()
//         }
//     } else {
//         Redirect::to("/").into_response()
//     }
// }

#[debug_handler]
async fn ldaplogin(
    Query(params): Query<BTreeMap<String, String>>,
    cookies: Cookies,
    State(core): State<Arc<Core>>,
    Form(log_in): Form<LogIn>,
) -> Result<impl IntoResponse, Error> {
    let username = log_in.username.trim();
    let password = log_in.password.trim();
    let (con, mut ldap) = ldap3::LdapConnAsync::new("ldap://localhost:3890").await?;
    ldap3::drive!(con);
    if let Ok(_result) = ldap.simple_bind(username, password).await?.success() {
        println!("{:#?}", _result);
        let my_claims = Claims {
            sub: username.to_string(),
            exp: 2000000000,
        };
        let token = encode(&Header::default(), &my_claims, &core.encoding_key)?;
        let cookie = Cookie::build(COOKIE_NAME, token)
            .domain(env::var("WAY_DOMAIN")?)
            .path("/")
            .max_age(cookie::time::Duration::days(7))
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);

        if let Some(rf) = params.get("ref") {
            Ok(Redirect::to(rf).into_response())
        } else {
            Ok(Redirect::to("/").into_response())
        }
    } else {
        Ok(Redirect::to("/").into_response())
    }
}

#[debug_handler]
async fn logout(cookies: Cookies) -> Result<impl IntoResponse, Error> {
    if cookies.get(COOKIE_NAME).is_some() {
        let cookie = Cookie::build(COOKIE_NAME, "")
            .domain(env::var("WAY_DOMAIN")?)
            .path("/")
            .max_age(cookie::time::Duration::seconds(0))
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(cookie);
        println!("cookie removed");
    }
    Ok(Redirect::to("/").into_response())
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
