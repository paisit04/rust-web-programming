use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use lazy_static::lazy_static;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use crate::config::Config;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection_pool"),
        }
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    return DBCONNECTION.db_connection.get().unwrap();
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

#[derive(Debug)]
pub enum DBError {
    Unavailable,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB {
    type Error = DBError;
    async fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => return Outcome::Success(DB { connection }),
            Err(_) => return Outcome::Failure((Status::BadRequest, DBError::Unavailable)),
        }
    }
}
