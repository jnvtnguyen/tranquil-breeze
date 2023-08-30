use axum::{Router, Server};
use std::env;
use tower_http::{add_extension::AddExtensionLayer, services::ServeDir, trace::TraceLayer};

use service::sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

mod error;

mod users;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let host = env::var("HOST").expect("HOST must be set.");
    let port = env::var("PORT").expect("PORT must be a 16 bit integer.");
    let static_path = env::var("STATIC_FILE_PATH").expect("STATIC_FILE_PATH must be set.");
    let static_files = String::from(static_path.strip_suffix("/").unwrap_or(&static_path));
    let server_url = format!("{host}:{port}");

    let db = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let app = api_router()
        .fallback_service(ServeDir::new(static_files))
        .layer(AddExtensionLayer::new(ApiContext { db }))
        .layer(TraceLayer::new_for_http());

    let addr = server_url.parse().unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

fn api_router() -> Router {
    users::router()
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
