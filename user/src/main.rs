use chrono::{DateTime, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use poem::{
    get, handler, listener::TcpListener, middleware::AddData, web::Data, EndpointExt, Route, Server,
};
use std::env;
use database::postgres::Db;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[handler]
fn index(_db: Data<&Db>) -> String {
    "Hello from user service!".to_string()
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables.");
    let db = Db::new(&database_url)?;

    db.run_migrations(MIGRATIONS)?;

    let app = Route::new().at("/", get(index)).with(AddData::new(db));

    Server::new(TcpListener::bind("0.0.0.0:4001"))
        .run(app)
        .await?;

    Ok(())
}
