mod types;
mod store;
mod tools;
mod handlers;

use std::{collections::HashSet, env::{self, VarError}, sync::Arc};

use axum::{extract::{FromRequest, Request, State}, http::status::StatusCode, middleware::{self, Next}, response::{Html, IntoResponse, Response}, routing::{get, post}, Router};
use encrypt;
use handlers::api::{handle_api_key, handle_connect_msg, handle_device_pocket};
use serde::Serialize;
use store::AccountStore;
use tokio::sync::RwLock;
use tools::{constant::ENCRYPTION_CONNECT_KEY, setup::setup_log};
use tracing::{debug, info, warn};
use tracing_appender::rolling;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
use types::{api_key::ApiKey, pocket::PocketConnectMsgResponse};

#[tokio::main]
async fn main() {
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender); 
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "link_server=info,axum=error".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_filter))
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .init();


    match dotenvy::from_filename("app.env") {
        Ok(p) => p,
        Err(e) => {
            info!("Dotnev file failed to load: {}", e);
            return
        }
    };
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}





enum AppError {
    ApiKeyRejection,
    EnvError(VarError),
    AcmError(aes_gcm::Error),
    DeviceNotFound,
}

impl IntoResponse for AppError {
    
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        let (status, message) = match self {
            AppError::ApiKeyRejection => {
                (StatusCode::UNAUTHORIZED, "the application api key was not authorized or was not found".to_owned())
            }
            AppError::EnvError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
            },
            AppError::AcmError(_) => {
                (StatusCode::UNAUTHORIZED, "you do not have permission to access this resource".to_string())
            },
            AppError::DeviceNotFound => {
                (StatusCode::NOT_FOUND, "device not found".to_owned())
            }
        };
        (status, AppJson(ErrorResponse{ message })).into_response()
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

fn app() -> Router {
    let api_key = ApiKey::new();
    let account_store = Arc::new(RwLock::new(AccountStore::new()));
    let api_router = Router::new()
    .route("/connect", get(handle_connect_msg))
    // Minister -> Account
    .route("/controller", post(handle_device_pocket))
    .with_state(account_store)
    .route_layer(middleware::from_fn_with_state(api_key, handle_api_key));
    let app = Router::new()
        .nest("/api", api_router);
    // let app = app.fallback(handler);
    app
}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt};
    use types::pocket::PocketRequest; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn get_connect_msg() {
        dotenvy::from_filename("app.env").unwrap();
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let request = Request::builder()
            .method("GET")
            .uri("/api/connect")
            .header("XMinisterApiKey", "nl31kPIk8X7U7w15");
        
        let response = app
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        // panic!("{:?}", response.status());
        assert_eq!(response.status(), StatusCode::OK);

        let _ = response.into_body().collect().await.unwrap().to_bytes();
        // panic!("{:?}", );
        // assert!(!body.is_empty());
    }
    
    #[tokio::test]
    async fn post_device_id_price() {
        dotenvy::from_filename("app.env").unwrap();
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let request = Request::builder()
            .method("POST")
            .uri("/api/controller")
            .header("Content-Type", "application/json")
            .header("XMinisterApiKey", "nl31kPIk8X7U7w15");
        let data: Vec<u8> = vec![94, 187, 183, 118, 114, 78, 165, 255, 235, 240, 193, 125, 142, 163, 20, 45, 158, 201, 90, 72, 30, 179, 250, 109, 197, 100, 161, 62, 85, 55, 122, 32, 30, 121, 152, 44, 155, 28, 153, 135, 248, 189, 243, 198, 179, 152, 31, 39, 129, 119, 93, 152, 65, 45, 50];
        let price: Vec<u8> = vec![5, 6, 4, 0, b'a', b'c'];
        let req = PocketRequest {data,price};
        let req_json = serde_json::to_vec(&req).unwrap();
        let body = Body::from(req_json);
        let response = app
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap();
        // panic!("{:?}", response.status());
        assert_eq!(response.status(), StatusCode::OK);
        let _ = response.into_body().collect().await.unwrap().to_bytes();
    }
}