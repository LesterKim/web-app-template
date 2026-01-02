use axum::routing::get;
use axum::Router;
use core_ports::GreetingRepository;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::ordering;
use crate::routes;
use crate::ordering::OrderingState;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn GreetingRepository>,
    pub ordering: OrderingState,
}

impl AppState {
    pub fn new(repo: Arc<dyn GreetingRepository>, ordering: OrderingState) -> Self {
        Self { repo, ordering }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index))
        .nest("/ordering", ordering::routes::router())
        .with_state(state)
        .nest_service("/static", ServeDir::new("apps/web/static"))
}
