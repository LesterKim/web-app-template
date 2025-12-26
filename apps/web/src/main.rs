mod config;
mod http;
mod presenters;
mod routes;
mod view_models;

use core_entities::Greeting;
use core_ports::GreetingRepository;
use datastore::postgres::PostgresGreetingRepository;
use datastore::MemoryGreetingRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();

    let repo: Arc<dyn GreetingRepository> = match config.database_url {
        Some(url) => Arc::new(PostgresGreetingRepository::new(url)),
        None => {
            let seed = vec![Greeting::new(1, "Hello from Clean Architecture")];
            Arc::new(MemoryGreetingRepository::new(seed))
        }
    };

    let state = http::AppState::new(repo);
    let app = http::router(state);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
