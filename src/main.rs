mod types;
mod tools;
mod handlers;
mod db_store;

use std::{env::{self, VarError}, sync::Arc};

use axum::{http::Method, middleware::{self}, routing::{get, post, put}, Router};
use handle_error::Error;
use tracing::{debug, info, warn};
use tracing_appender::rolling;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
use tower_http::cors::{Any, CorsLayer};
use crate::{db_store::Store, handlers::{account::get_account, bank::get_bank, business::get_business, customer::create_customer, metal::get_metal_health, middleware::{auth_middleware, metal_apk, public_apk}, payment::{customer_pay, metal_pay}, user::{get_user_profile, login, refresh_token, register, update_user}}, tools::constant::DATABASE_URL, types::cache::Cache};

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
    let url = env::var(DATABASE_URL)
            .map_err(|e| Error::EnvError(e)).expect("failed at key");
    let store = Store::new(&url).await;
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot migrate DB");
    let cache = Arc::new(Cache::new(&store).await);
    // handlers::business::setup_device(&store).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app(store, cache)).await.unwrap();
}



fn app(store: Store, cache: Arc<Cache>) -> Router {
    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/profile", get(get_user_profile))
        .route("/update", put(update_user))
        .route("/businesses", get(get_business))
        .route("/bank", get(get_bank))
        .route("/account", get(get_account))
        .layer(middleware::from_fn(auth_middleware));

    // Public routes for user operations
    let public_user_routes = Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/register", post(register));

    // APK routes
    let public_apk_routes = Router::new()
        .route("/connect", post(create_customer))
        .route("/payment", post(customer_pay))
        .layer(middleware::from_fn(public_apk));

    let metal_apk_routes = Router::new()
        .route("/heathly", get(get_metal_health))
        .route("/payment", post(metal_pay))
        .layer(middleware::from_fn(metal_apk));

    let app_router = Router::new()
        .nest("/user", public_user_routes)
        .nest("/auth", protected_routes)
        .nest("/apk", public_apk_routes)
        .nest("/metal", metal_apk_routes)
        .with_state((store, cache));
        

    // ✅ FIX: Add allow_headers
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any); // <-- this allows Content-Type and Authorization headers

    let app = Router::new()
        .nest("/api", app_router)
        .layer(cors);

    app
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;    use tower::ServiceExt;
    use types::pocket::PocketRequest; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn get_connect_msg() {
        dotenvy::from_filename("app.env").unwrap();
        let store = Store::new("postgres://127.0.0.1:5432/metal_test").await;
        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Cannot migrate DB");
        let cache = Arc::new(Cache::new(&store).await);
        let app = app(store, cache);

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
        let store = Store::new("postgres://127.0.0.1:5432/metal_test").await;
        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Cannot migrate DB");
        let cache = Arc::new(Cache::new(&store).await);
        let app = app(store, cache);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let request = Request::builder()
            .method("POST")
            .uri("/api/controller")
            .header("Content-Type", "application/json")
            .header("XMinisterApiKey", "nl31kPIk8X7U7w15");
        let data: Vec<u8> = vec![94, 187, 183, 118, 114, 78, 165, 255, 235, 240, 193, 125, 142, 163, 20, 45, 158, 201, 90, 72, 30, 179, 250, 109, 197, 100, 161, 62, 85, 55, 122, 32, 30, 121, 152, 44, 155, 28, 153, 135, 248, 189, 243, 198, 179, 152, 31, 39, 129, 119, 93, 152, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100];
        let price: Vec<u8> = vec![242, 8, 63, 97, 232, 72, 68, 66, 39, 154, 177, 80, 194, 59, 135, 119];
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