use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::model::message;
use crate::model::space::{Space, SpaceId};

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

        match sqlx::migrate!("./src/store/migrations/")
            .run(&db_pool)
            .await
        {
            Ok(res) => tracing::event!(tracing::Level::INFO, "store::migrated success {:?}", res),
            Err(e) => tracing::event!(tracing::Level::ERROR, "store::migrated error {:?}", e),
        }

        Store {
            connection: db_pool,
        }
    }

    pub async fn create_space(&self, new_space: Space) -> Result<Space, crate::error::Error> {
        match sqlx::query("INSERT INTO spaces (space_id, name, owner) VALUES (nextval('space_id_seq'), $1, $2) RETURNING space_id, name, owner;")
            .bind(new_space.name)
            .bind(new_space.owner)
            .map(map_to_space)
            .fetch_one(&self.connection)
            .await
        {
            Ok(space) => Ok(space),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "store::create_space {:?}", e);
                Err(crate::error::Error::DatabaseQueryError(e))
            }
        }
    }
}

fn map_to_space(row: PgRow) -> Space {
    Space {
        space_id: Some(SpaceId(row.get("space_id"))),
        name: row.get("name"),
        owner: row.get("owner"),
    }
}
