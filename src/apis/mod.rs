use std::fmt::{self, Display, Formatter};

mod requests;

pub mod tts;

const TEXT_TO_SPEECH: &str = "text-to-speech";

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An Error returned by the API
    ApiError(u16, String),
    /// An Error not related to the API
    RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(status, msg) => write!(f, "API responded with status {} error: {}", status, msg),
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Status(status, response) => {
                let error_msg = response.into_json::<Json>().unwrap();
                return Error::ApiError(status, format!("{error_msg}"));
            }
            ureq::Error::Transport(e) => {
                return Error::RequestError(e.to_string());
            }
        }
    }
}
