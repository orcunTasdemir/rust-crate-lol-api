use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    ApiKeyWrongError,
    OtherApiSideError(reqwest::Error),
    NoChampionWithIdFound(u64),
    JsonError(serde_json::Error),
}
pub type Result<T> = core::result::Result<T, Error>;

// region: error boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), std::fmt::Error> {
        match self {
            Self::OtherApiSideError(err) => write!(fmt, "Request failed: {}", err),
            Self::JsonError(err) => write!(fmt, "Request failed: {}", err),
            Self::ApiKeyWrongError => write!(fmt, "API key is incorrect"),
            Self::NoChampionWithIdFound(id) => {
                write!(fmt, "Request failed for Champion with id: {}", id)
            }
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl std::error::Error for Error {}
// endregion: error boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESPONSE");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::OtherApiSideError(err)
    }
}
