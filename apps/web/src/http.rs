use axum::routing::{get, post};
use axum::Router;
use core_entities::PricingPolicy;
use core_ports::{
    CartRepository, CatalogRepository, EmailGateway, EmployeeRepository, InvoiceRepository,
    QuoteRepository, SessionRepository,
};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub employees: Arc<dyn EmployeeRepository>,
    pub catalog: Arc<dyn CatalogRepository>,
    pub carts: Arc<dyn CartRepository>,
    pub sessions: Arc<dyn SessionRepository>,
    pub invoices: Arc<dyn InvoiceRepository>,
    pub quotes: Arc<dyn QuoteRepository>,
    pub email: Arc<dyn EmailGateway>,
    pub pricing: PricingPolicy,
}

impl AppState {
    pub fn new(
        employees: Arc<dyn EmployeeRepository>,
        catalog: Arc<dyn CatalogRepository>,
        carts: Arc<dyn CartRepository>,
        sessions: Arc<dyn SessionRepository>,
        invoices: Arc<dyn InvoiceRepository>,
        quotes: Arc<dyn QuoteRepository>,
        email: Arc<dyn EmailGateway>,
        pricing: PricingPolicy,
    ) -> Self {
        Self {
            employees,
            catalog,
            carts,
            sessions,
            invoices,
            quotes,
            email,
            pricing,
        }
    }
}

fn static_dir() -> PathBuf {
    let manifest_static = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    if manifest_static.exists() && manifest_static.is_dir() {
        return manifest_static;
    }

    let paths = ["apps/web/static", "static", "../static"];
    for path in paths {
        let candidate = PathBuf::from(path);
        if candidate.exists() && candidate.is_dir() {
            return candidate;
        }
    }

    PathBuf::from("apps/web/static")
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::landing))
        .route("/signup", get(routes::signup))
        .route("/signup", post(routes::signup_submit))
        .route("/signin", get(routes::signin))
        .route("/signin", post(routes::signin_submit))
        .route("/signout", post(routes::signout))
        .route("/catalog", get(routes::catalog))
        .route("/cart", get(routes::cart))
        .route("/cart/add", post(routes::add_to_cart))
        .route("/cart/update", post(routes::update_cart))
        .route("/cart/count", get(routes::cart_count_htmx))
        .route("/quote", get(routes::quote))
        .route("/quotes", get(routes::quotes))
        .route("/quotes/:id", get(routes::quote_details))
        .route("/confirm", post(routes::confirm_order))
        .with_state(state)
        .nest_service("/static", ServeDir::new(static_dir()))
}
