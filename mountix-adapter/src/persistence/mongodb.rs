use std::sync::Arc;

use mongodb::{Client, Database};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new(uri: &str) -> Db {
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Could not connect to MongoDB.");
        let db = client.database("mountix_db");
        Db(Arc::new(db))
    }
}
