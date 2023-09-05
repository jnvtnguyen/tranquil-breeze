use axum::{Router, Server};
use clap::Parser;
use std::sync::Arc;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

use migration::{Migrator, MigratorTrait};
use service::sea_orm::{Database, DatabaseConnection};

mod activations;
mod config;
mod error;
mod extractor;
mod users;
mod workspaces;

pub use config::Config;
pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let config = Config::parse();

    let db_url = &config.database_url;
    let host = &config.host;
    let port = &config.port;
    let server_url = format!("{host}:{port}");

    let db = Database::connect(db_url).await.unwrap();
    Migrator::up(&db, None).await?;

    let app = api_router()
        .layer(AddExtensionLayer::new(ApiContext {
            config: Arc::new(config),
            db,
        }))
        .layer(TraceLayer::new_for_http());

    let addr = server_url.parse().unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

fn api_router() -> Router {
    users::router()
        .merge(workspaces::router())
        .merge(activations::router())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
