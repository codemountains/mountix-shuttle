use mountix_driver::module::Modules;
use mountix_driver::startup::startup;
use shuttle_secrets::SecretStore;
use std::sync::Arc;
use tracing::info;

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let db_url = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL is undefined.");

    let _ = write_env(&secret_store);

    let modules = Modules::new(&db_url).await;
    let router = startup(Arc::new(modules)).await;

    Ok(router.into())
}

// Write to environment variables
// See more: https://docs.shuttle.rs/resources/shuttle-secrets#caveats
fn write_env(secret_store: &SecretStore) {
    info!("Write to environment variables");
    let mountains_url = secret_store
        .get("MOUNTAINS_URL")
        .expect("MOUNTAINS_URL is undefined.");
    std::env::set_var("MOUNTAINS_URL", mountains_url);
    let documents_url = secret_store
        .get("DOCUMENTS_URL")
        .expect("DOCUMENTS_URL is undefined.");
    std::env::set_var("DOCUMENTS_URL", documents_url);
    let default_distance = secret_store
        .get("DEFAULT_DISTANCE")
        .expect("DEFAULT_DISTANCE is undefined.");
    std::env::set_var("DEFAULT_DISTANCE", default_distance);
    let max_distance = secret_store
        .get("MAX_DISTANCE")
        .expect("MAX_DISTANCE is undefined.");
    std::env::set_var("MAX_DISTANCE", max_distance);
}
