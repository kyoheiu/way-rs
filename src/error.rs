use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Io(String),
    Env(String),
    Ldap(String),
    Jwt(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            Error::Io(s) => s,
            Error::Env(s) => s,
            Error::Ldap(s) => s,
            Error::Jwt(s) => s,
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(format!("IO Error: {}", err))
    }
}

impl From<ldap3::LdapError> for Error {
    fn from(err: ldap3::LdapError) -> Self {
        Error::Ldap(format!("LDAP Error: {}", err))
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::Env(format!("ENV Error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::Jwt(format!("JWT Error: {}", err))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Io(s) => s,
            Error::Env(s) => s,
            Error::Ldap(s) => s,
            Error::Jwt(s) => s,
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
