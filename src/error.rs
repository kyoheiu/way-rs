use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Io(String),
    Env(String),
    Tera(String),
    Ldap(String),
    Jwt(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            Error::Io(s) => s,
            Error::Env(s) => s,
            Error::Tera(s) => s,
            Error::Ldap(s) => s,
            Error::Jwt(s) => s,
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<ldap3::LdapError> for Error {
    fn from(err: ldap3::LdapError) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Io(s) => s,
            Error::Env(s) => s,
            Error::Tera(s) => s,
            Error::Ldap(s) => s,
            Error::Jwt(s) => s,
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
