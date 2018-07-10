use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;
use redis::Connection;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use std::ops::Deref;


type RedisPool = Pool<RedisConnectionManager>;

pub fn init_pool(redis_url: &str) -> RedisPool {
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    Pool::new(manager).expect("Failed in initializing the redis connection pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub PooledConnection<RedisConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<RedisPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}