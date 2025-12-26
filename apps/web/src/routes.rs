use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use core_use_cases::ListGreetingsInteractor;

use crate::http::AppState;
use crate::presenters::GreetingPresenter;
use crate::view_models::GreetingViewModel;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    vm: GreetingViewModel,
}

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let presenter = GreetingPresenter::new();
    let interactor = ListGreetingsInteractor::new(state.repo.as_ref(), &presenter);

    if let Err(err) = interactor.execute().await {
        eprintln!("use case error: {}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "use case error").into_response();
    }

    let view_model = presenter.take_view_model();
    let template = IndexTemplate { vm: view_model };

    match template.render() {
        Ok(body) => Html(body).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response(),
    }
}
