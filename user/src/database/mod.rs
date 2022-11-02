use anyhow::{anyhow, Result};
use diesel::{
    pg::{PgConnection, Pg},
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<()> {
    if connection.has_pending_migration(MIGRATIONS).unwrap_or_default() {
        println!("Starting to run pending migrations:");
    }
    for migration in connection.pending_migrations(MIGRATIONS).unwrap() {
        println!("{:?}", migration.name().to_string());
    }
    connection.run_pending_migrations(MIGRATIONS).expect("Failed to run pending migrations.");

    if connection.has_pending_migration(MIGRATIONS).unwrap_or_default() {
        println!("Finishing running pending migrations");
    }
    
    Ok(())
}

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

        println!("Created database connection pool.");
        
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
