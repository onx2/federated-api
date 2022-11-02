use chrono::{DateTime, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use poem::{get, handler, listener::TcpListener, middleware::AddData, EndpointExt, Route, Server, web::Data};
use std::env;
use user::{
    database::{run_migrations, Db},
    schema::user::dsl,
};
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[handler]
fn index(db: Data<&Db>) -> String {
    let pool = &mut db.connect().expect("Couldn't connect to db.");
    
    let results = dsl::user
        .limit(5)
        .load::<User>(pool)
        .expect("Error loading users");

    format!("Hello from user service!\n {:?}", results)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables.");
    let db = Db::new(&database_url)?;
    
    // Run migrations as part of compiled binary
    let pool = &mut db.connect()?;
    run_migrations(pool)?;

    let app = Route::new().at("/", get(index)).with(AddData::new(db));

    Server::new(TcpListener::bind("0.0.0.0:4001"))
        .run(app)
        .await?;

    Ok(())
}
