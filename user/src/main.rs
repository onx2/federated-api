use chrono::{DateTime, Utc};
use database::postgres::Db;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;
use poem::{
    get, handler, listener::TcpListener, middleware::AddData, web::Data, EndpointExt, Route, Server,
};
use std::env;
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

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables.");
    let port = env::var("PORT").expect("PORT must be set in environment variables.");
    
    let db = Db::new(&database_url)?;

    db.run_migrations(MIGRATIONS)?;

    let app = Route::new().at("/", get(index)).with(AddData::new(db));

    println!("Listening for requests on port {port}");
    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(app)
        .await?;

    Ok(())
}
