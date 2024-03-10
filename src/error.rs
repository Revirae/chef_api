use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[error("generic error: {0}")]
    Generic(String),

    #[error("database error")]
    Db,

    // #[error("failed to initialize static database")]
    // FailedToInitDb,
    #[error("failed to lock db in a {0} operation")]
    FailedToLockDb(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Generic(msg) => {
                HttpResponse::InternalServerError()
                    .body(msg.clone())
            }
            _ => HttpResponse::InternalServerError()
                .body(self.to_string()),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
