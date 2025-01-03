use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use thiserror::Error;

use crate::prelude::ResponseStatusType;
use crate::util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(util::gen_response(ResponseStatusType::Error("".to_string()), self.to_string())))
            .into_response() // TODO: implement other error types
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
