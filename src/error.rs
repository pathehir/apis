use thiserror::Error;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ParseError(#[from] std::io::Error),
    #[error("http error {}", .0)]
    HttpError(#[from] ureq::Error)
}
