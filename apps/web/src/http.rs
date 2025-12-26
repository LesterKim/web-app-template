use axum::routing::get;
use axum::Router;
use core_ports::GreetingRepository;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn GreetingRepository>,
}

impl AppState {
    pub fn new(repo: Arc<dyn GreetingRepository>) -> Self {
        Self { repo }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index))
        .with_state(state)
        .nest_service("/static", ServeDir::new("apps/web/static"))
}
