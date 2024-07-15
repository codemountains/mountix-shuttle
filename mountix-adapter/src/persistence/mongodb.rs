use std::sync::Arc;

use crate::model::mountain::MountainDocument;
use mongodb::bson::doc;
use mongodb::{Database, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new(db: Database) -> Db {
        // insert initial data
        let _ = init(db.clone()).await;

        Db(Arc::new(db))
    }
}

const MOUNTAINS_JSON: &'static [u8] = include_bytes!("../data/mountains.json");

#[derive(Debug, Deserialize, Serialize)]
pub struct InitialData {
    mountains: Vec<MountainDocument>,
}

async fn init(db: Database) {
    let count = db
        .collection::<MountainDocument>("mountains")
        .count_documents(None, None)
        .await
        .expect("Failed to connect database.");

    if count > 0 {
        return;
    }

    // Initialization
    let initial_data: InitialData =
        serde_json::from_slice(MOUNTAINS_JSON).expect("Failed to read mountains data.");
    let _ = db
        .collection::<MountainDocument>("mountains")
        .insert_many(initial_data.mountains, None)
        .await
        .expect("Failed to insert initial data.");

    // Create a 2dsphere Index
    // https://www.mongodb.com/docs/manual/core/indexes/index-types/geospatial/2dsphere/create/
    let geo_index = IndexModel::builder()
        .keys(doc! {"location": "2dsphere"})
        .build();
    let _ = db
        .collection::<MountainDocument>("mountains")
        .create_index(geo_index, None)
        .await
        .expect("Failed to create a 2dsphere index.");
}
