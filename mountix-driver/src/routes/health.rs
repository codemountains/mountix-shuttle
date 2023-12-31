use crate::module::{Modules, ModulesExt};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::error;

pub async fn hc() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn hc_mongodb(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .health_check_use_case()
        .diagnose_mongo_db_conn()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}
