use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApiClientErrorCode {
    ApiKeyDisabled,
    ApiKeyExhausted,
    ApiKeyInvalid,
    ApiKeyMissing,
    ParameterInvalid,
    ParametersMissing,
    RateLimited,
    SourcesTooMany,
    SourceDoesNotExist,
    UnexpectedError,
}

#[derive(Debug)]
pub struct ApiClientErrorResponse {
    pub status: String,
    pub code: ApiClientErrorCode,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiClientError {
    Http(reqwest::Error),
    InvalidRequest(String),
    InvalidResponse(ApiClientErrorResponse),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

impl fmt::Display for ApiClientErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientErrorCode::ApiKeyDisabled => write!(f, "apiKeyDisabled"),
            ApiClientErrorCode::ApiKeyExhausted => write!(f, "apiKeyExhausted"),
            ApiClientErrorCode::ApiKeyInvalid => write!(f, "apiKeyInvalid"),
            ApiClientErrorCode::ApiKeyMissing => write!(f, "apiKeyMissing"),
            ApiClientErrorCode::ParameterInvalid => write!(f, "parameterInvalid"),
            ApiClientErrorCode::ParametersMissing => write!(f, "parametersMissing"),
            ApiClientErrorCode::RateLimited => write!(f, "rateLimited"),
            ApiClientErrorCode::SourcesTooMany => write!(f, "sourcesTooMany"),
            ApiClientErrorCode::SourceDoesNotExist => write!(f, "sourceDoesNotExist"),
            ApiClientErrorCode::UnexpectedError => write!(f, "unexpectedError"),
        }
    }
}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::Http(err) => write!(f, "HTTP error: {}", err),
            ApiClientError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiClientError::InvalidResponse(response) => write!(f, "Invalid response: {} - {}", response.code, response.message),
            ApiClientError::InvalidHeaderValue(err) => write!(f, "Invalid header value: {}", err),
        }
    }
}

impl Error for ApiClientError {}

impl From<reqwest::Error> for ApiClientError {
    fn from(err: reqwest::Error) -> ApiClientError {
        ApiClientError::Http(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ApiClientError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> ApiClientError {
        ApiClientError::InvalidHeaderValue(err)
    }
}
