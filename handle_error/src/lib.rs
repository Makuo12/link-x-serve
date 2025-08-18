use std::env::VarError;

use axum::{extract::FromRequest, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    CannotDecryptToken,
    Unauthorized,
    ArgonLibraryError(ArgonError),
    DatabaseQueryError(sqlx::Error),
    MigrationError(sqlx::migrate::MigrateError),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
    ApiKeyRejection,
    EnvError(VarError),
    AcmError(aes_gcm::Error),
    DeviceNotFound,
    InvalidSessionKey(String),
    TokenCreationError(String)
    // other variants...
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::CannotDecryptToken => write!(f, "Cannot decrypt error"),
            Error::Unauthorized => write!(f, "No permission to change the underlying resource"),
            Error::ArgonLibraryError(_) => write!(f, "Cannot verifiy password"),
            Error::DatabaseQueryError(_) => write!(f, "Cannot update, invalid data"),
            Error::MigrationError(_) => write!(f, "Cannot migrate data"),
            Error::ReqwestAPIError(err) => write!(f, "External API error: {}", err),
            Error::MiddlewareReqwestAPIError(err) => write!(f, "External API error: {}", err),
            Error::ClientError(err) => write!(f, "External Client error: {}", err),
            Error::ServerError(err) => write!(f, "External Server error: {}", err),
            Error::ApiKeyRejection => write!(f, "the application api key was not authorized or was not found"),
            Error::EnvError(var_error) => write!(f, "Environment variable error: {}", var_error),
            Error::AcmError(error) => write!(f, "Aes GCM error: {}", error),
            Error::DeviceNotFound => write!(f, "Device not found"),
            Error::InvalidSessionKey(var) => write!(f, "Session key invalid {}", *var),
            Error::TokenCreationError(var) => write!(f, "Token generation key invalid {}", *var)
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // How we want error responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Error::ApiKeyRejection => (
                        StatusCode::UNAUTHORIZED,
                        "the application api key was not authorized or was not found".to_owned(),
                    ),
            Error::EnvError(msg) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("environment error: {}", msg),
                    ),
            Error::AcmError(_) => (
                        StatusCode::UNAUTHORIZED,
                        "you do not have permission to access this resource".to_owned(),
                    ),
            Error::DeviceNotFound => (
                        StatusCode::NOT_FOUND,
                        "device not found".to_owned(),
                    ),
            Error::ParseError(parse_int_error) => (
                        StatusCode::BAD_REQUEST,
                        format!("failed to parse input: {}", parse_int_error),
                    ),
            Error::MissingParameters => (
                        StatusCode::BAD_REQUEST,
                        "missing required parameters".to_owned(),
                    ),
            Error::WrongPassword => (
                        StatusCode::UNAUTHORIZED,
                        "incorrect password".to_owned(),
                    ),
            Error::CannotDecryptToken => (
                        StatusCode::UNAUTHORIZED,
                        "could not decrypt authentication token".to_owned(),
                    ),
            Error::Unauthorized => (
                        StatusCode::UNAUTHORIZED,
                        "unauthorized access".to_owned(),
                    ),
            Error::ArgonLibraryError(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("argon2 hashing error: {}", error),
                    ),
            Error::DatabaseQueryError(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("database query failed: {}", error),
                    ),
            Error::MigrationError(migrate_error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("database migration failed: {}", migrate_error),
                    ),
            Error::ReqwestAPIError(error) => (
                        StatusCode::BAD_GATEWAY,
                        format!("external API error: {}", error),
                    ),
            Error::MiddlewareReqwestAPIError(error) => (
                        StatusCode::BAD_GATEWAY,
                        format!("middleware API error: {}", error),
                    ),
            Error::ClientError(apilayer_error) => (
                        StatusCode::BAD_REQUEST,
                        format!("client error: {}", apilayer_error),
                    ),
            Error::ServerError(apilayer_error) => (
                        StatusCode::BAD_GATEWAY,
                        format!("server error: {}", apilayer_error),
                    ),
            Error::InvalidSessionKey(session_error) => (
                StatusCode::BAD_REQUEST,
                format!("session error {}", session_error)
            ),
            Error::TokenCreationError(token_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("session error {}", token_error)
            )
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}
