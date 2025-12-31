mod config;
mod http;
mod presenters;
mod routes;
mod view_models;

use core_entities::PricingPolicy;
use core_ports::{
    CartRepository, CatalogRepository, EmailGateway, EmployeeRepository, InvoiceRepository,
    QuoteRepository, SessionRepository,
};
use datastore::{
    postgres::PostgresStore, InMemoryCartRepository, InMemoryCatalogRepository,
    InMemoryEmailGateway, InMemoryEmployeeRepository, InMemoryInvoiceRepository,
    InMemoryQuoteRepository, InMemorySessionRepository,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();

    let pricing = PricingPolicy::default();

    let (employees, catalog, carts, sessions, invoices, quotes, email): (
        Arc<dyn EmployeeRepository>,
        Arc<dyn CatalogRepository>,
        Arc<dyn CartRepository>,
        Arc<dyn SessionRepository>,
        Arc<dyn InvoiceRepository>,
        Arc<dyn QuoteRepository>,
        Arc<dyn EmailGateway>,
    ) = match config.database_url {
        Some(url) => {
            let store = Arc::new(PostgresStore::new(url));
            (
                store.clone(),
                store.clone(),
                store.clone(),
                store.clone(),
                store.clone(),
                store.clone(),
                store.clone(),
            )
        }
        None => (
            Arc::new(InMemoryEmployeeRepository::new(Vec::new())),
            Arc::new(InMemoryCatalogRepository::seeded()),
            Arc::new(InMemoryCartRepository::new()),
            Arc::new(InMemorySessionRepository::new()),
            Arc::new(InMemoryInvoiceRepository::new()),
            Arc::new(InMemoryQuoteRepository::new()),
            Arc::new(InMemoryEmailGateway::new()),
        ),
    };

    let state = http::AppState::new(
        employees, catalog, carts, sessions, invoices, quotes, email, pricing,
    );
    let app = http::router(state);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
