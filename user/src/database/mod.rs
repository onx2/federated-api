use anyhow::{anyhow, Result};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

#[derive(Debug, Clone)]
pub struct Db {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Db {
    pub fn new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let connection_pool = match Pool::builder().max_size(15).build(manager) {
            Ok(pool) => pool,
            Err(err) => {
                return Err(anyhow!(err));
            }
        };

        Ok(Db { connection_pool })
    }

    /// Retrieves a connection from the Postgres connection pool.
    ///
    /// Errors produced from this method are automatically logged at `error` level
    pub fn connect(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        match self.get_pool().get() {
            Ok(conn) => Ok(conn),
            Err(err) => Err(err.into()),
        }
    }

    /// Retrieves a clone of the Db connection_pool
    fn get_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.connection_pool.clone()
    }
}
