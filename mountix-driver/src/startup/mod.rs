use crate::module::Modules;
use crate::routes::health::{hc, hc_mongodb};
use crate::routes::information::info;
use crate::routes::mountain::{find_mountains, find_mountains_by_box, get_mountain};
use crate::routes::surrounding_mountain::find_surroundings;
use axum::http::Method;
use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(modules: Arc<Modules>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS, Method::HEAD])
        .allow_origin(Any);

    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb));

    let mountain_router = Router::new()
        .route("/", get(find_mountains))
        .route("/:id", get(get_mountain))
        .route("/:id/surroundings", get(find_surroundings))
        .route("/geosearch", get(find_mountains_by_box));

    let info_router = Router::new().route("/", get(info));

    Router::new()
        .nest("/api/v1/", info_router)
        .nest("/api/v1/hc", hc_router)
        .nest("/api/v1/mountains", mountain_router)
        .layer(cors)
        .layer(Extension(modules))
}
