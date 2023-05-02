use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::model::message;
use crate::model::space::{Space, SpaceId};
use crate::model::user::User;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new_from_config(config: &crate::config::Config) -> Self {
        Store::new_from_url(
            &format!(
                "postgres://{}:{}@{}:{}/{}",
                config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
            ),
            &format!(
                "postgres://{}:{}@{}:{}/{}",
                config.db_api_user,
                config.db_api_password,
                config.db_host,
                config.db_port,
                config.db_name
            ),
        )
        .await
    }

    async fn new_from_url(db_url: &str, db_api_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(db_url)
            .await
        {
            Ok(pool) => {
                tracing::event!(
                    tracing::Level::INFO,
                    "store::connected success for migration"
                );
                pool
            }
            Err(e) => panic!(
                "store::new Unable to connect to the Db for migration: {:?}",
                e
            ),
        };

        match sqlx::migrate!("./src/store/migrations/")
            .run(&db_pool)
            .await
        {
            Ok(res) => tracing::event!(tracing::Level::INFO, "store::migrated success {:?}", res),
            Err(e) => tracing::event!(tracing::Level::ERROR, "store::migrated error {:?}", e),
        };
        db_pool.close().await;

        let db_api_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_api_url)
            .await
        {
            Ok(pool) => {
                tracing::event!(tracing::Level::INFO, "store::connected success for api");
                pool
            }
            Err(e) => panic!("store::new Unable to connect to the Db: {:?}", e),
        };

        Store {
            connection: db_api_pool,
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

    pub async fn create_user(&self, new_user: User) -> Result<User, crate::error::Error> {
        match sqlx::query(
            "INSERT INTO users(user_id, pw_hash) VALUES ($1, $2) RETURNING user_id, pw_hash;",
        )
        .bind(new_user.user_id)
        .bind(new_user.pw_hash)
        .map(map_to_user)
        .fetch_one(&self.connection)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "store::create_user {:?}", e);
                Err(crate::error::Error::DatabaseQueryError(e))
            }
        }
    }
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, crate::error::Error> {
        match sqlx::query("SELECT user_id, pw_hash FROM users WHERE user_id = $1;")
            .bind(user_id)
            .map(map_to_user)
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "store::create_user {:?}", e);
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

fn map_to_user(row: PgRow) -> User {
    User {
        user_id: row.get("user_id"),
        pw_hash: row.get("pw_hash"),
    }
}
