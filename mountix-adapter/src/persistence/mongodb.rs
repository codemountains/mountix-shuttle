use std::sync::Arc;

use crate::model::mountain::MountainDocument;
use crate::persistence::{init_data_1, init_data_2, init_data_3, init_data_4};
use mongodb::Database;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new(db: Database) -> Db {
        // insert initial data
        let _ = init(db.clone()).await;

        Db(Arc::new(db))
    }
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

    // MEMO: To prevent overflow, `mountains` was divided
    //
    // ```
    // thread 'tokio-runtime-worker' has overflowed its stack
    // fatal runtime error: stack overflow
    // ```
    for idx in 1..=4 {
        let mountains = match idx {
            1 => init_data_1(),
            2 => init_data_2(),
            3 => init_data_3(),
            _ => init_data_4(),
        };

        let _ = db
            .collection("mountains")
            .insert_many(mountains, None)
            .await
            .expect("Failed to insert initial data.");
    }
}
