use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::message;
use crate::types::space;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("store::new Unable to connect to the Db: {:?}", e),
        };

        match sqlx::migrate!("./src/store/migrations/").run(&db_pool).await {
            Ok(res) => tracing::event!(tracing::Level::INFO, "store::migrated success {:?}", res),
            Err(e) => tracing::event!(tracing::Level::ERROR, "store::migrated error {:?}", e),
        }

        Store {
            connection: db_pool,
        }
    }
}
