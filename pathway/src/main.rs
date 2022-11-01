use dotenvy::dotenv;
use poem::{get, handler, listener::TcpListener, Route, Server};
use std::env;
use pathway::database::Db;

#[handler]
fn index() -> String {
    format!("Hello from pathway service!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables.");
    let _db = Db::new(&database_url)?;

    let app = Route::new().at("/", get(index));

    Server::new(TcpListener::bind("0.0.0.0:4002"))
        .run(app)
        .await?;

    Ok(())
}
