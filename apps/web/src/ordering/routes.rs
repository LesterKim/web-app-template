use askama::Template;
use axum::extract::{Form, Query, State};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use core_ports::ordering::{CartRepository, InvoiceRepository, SessionStore};
use core_use_cases::ordering::{
    AddItemToCartInput, AddItemToCartInteractor, AuthError, QuoteError, SignInEmployeeInput,
    SignInEmployeeInteractor, SignOutEmployeeInput, SignOutEmployeeInteractor, SignUpEmployeeInput,
    SignUpEmployeeInteractor, SubmitQuoteInput, SubmitQuoteInteractor, ViewQuoteInput,
    ViewQuoteInteractor,
};
use serde::Deserialize;

use crate::http::AppState;
use crate::ordering::presenters::OrderingPresenter;
use crate::ordering::view_models::{
    CartItemsViewModel, CartViewModel, InvoicePageViewModel, QuotePageViewModel, QuotePreviewViewModel,
    SignInFormViewModel, SignInViewModel, SignUpFormViewModel, SignUpViewModel, SchoolOptionViewModel,
};
use crate::ordering::OrderingState;

const SESSION_COOKIE_NAME: &str = "employee_email";

#[derive(Template)]
#[template(path = "ordering/sign_in.html")]
struct SignInTemplate {
    vm: SignInViewModel,
}

#[derive(Template)]
#[template(path = "ordering/partials/sign_in_form.html")]
struct SignInFormTemplate {
    vm: SignInViewModel,
}

#[derive(Template)]
#[template(path = "ordering/sign_up.html")]
struct SignUpTemplate {
    vm: SignUpViewModel,
}

#[derive(Template)]
#[template(path = "ordering/partials/sign_up_form.html")]
struct SignUpFormTemplate {
    vm: SignUpViewModel,
}

#[derive(Template)]
#[template(path = "ordering/cart.html")]
struct CartTemplate {
    vm: CartViewModel,
}

#[derive(Template)]
#[template(path = "ordering/partials/cart_items.html")]
struct CartItemsTemplate {
    vm: CartItemsViewModel,
}

#[derive(Template)]
#[template(path = "ordering/partials/quote_preview.html")]
struct QuotePreviewTemplate {
    vm: QuotePreviewViewModel,
}

#[derive(Template)]
#[template(path = "ordering/quote.html")]
struct QuoteTemplate {
    vm: QuotePageViewModel,
}

#[derive(Template)]
#[template(path = "ordering/invoice.html")]
struct InvoiceTemplate {
    vm: InvoicePageViewModel,
}

#[derive(Deserialize)]
struct SignInQuery {
    welcome: Option<String>,
}

#[derive(Deserialize)]
struct SignInForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SignUpForm {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    title: String,
    school: String,
    phone: String,
    delivery_window: String,
}

#[derive(Deserialize)]
struct AddToCartForm {
    description: String,
    quantity: u32,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(ordering_root))
        .route("/sign-in", get(sign_in_form).post(sign_in))
        .route("/sign-up", get(sign_up_form).post(sign_up))
        .route("/sign-out", post(sign_out))
        .route("/cart", get(cart))
        .route("/cart/add", post(add_to_cart))
        .route("/quote", get(quote))
        .route("/quote/preview", get(quote_preview))
        .route("/quote/submit", post(submit_quote))
}

async fn ordering_root(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    match authenticated_email(ordering, &headers) {
        Ok(_) => redirect_with_cookie("/ordering/cart", None),
        Err(_) => redirect_with_cookie("/ordering/sign-in", None),
    }
}

async fn sign_in_form(Query(query): Query<SignInQuery>) -> impl IntoResponse {
    let message = if query.welcome.is_some() {
        Some("Account created. Sign in to continue.".to_string())
    } else {
        None
    };

    let vm = SignInViewModel {
        title: "Sign in".to_string(),
        subtitle: "Welcome back to the NYC DOE ordering desk.".to_string(),
        form: SignInFormViewModel::empty(),
        error: None,
        message,
    };

    render_template(SignInTemplate { vm })
}

async fn sign_in(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<SignInForm>,
) -> impl IntoResponse {
    let interactor = SignInEmployeeInteractor::new(&state.ordering.employees, &state.ordering.sessions);
    let result = interactor.execute(SignInEmployeeInput::new(&form.email, &form.password));

    match result {
        Ok(output) => {
            let session = OrderingPresenter::sign_in_session(&output);
            let cookie = session_cookie(&session.email);
            redirect_with_cookie("/ordering/cart", Some(cookie))
        }
        Err(err) => {
            let vm = SignInViewModel {
                title: "Sign in".to_string(),
                subtitle: "Welcome back to the NYC DOE ordering desk.".to_string(),
                form: SignInFormViewModel {
                    email: form.email,
                    password: String::new(),
                },
                error: Some(err.message),
                message: None,
            };
            if is_htmx(&headers) {
                render_template(SignInFormTemplate { vm })
            } else {
                render_template(SignInTemplate { vm })
            }
        }
    }
}

async fn sign_up_form(State(state): State<AppState>) -> impl IntoResponse {
    let vm = build_sign_up_vm(
        &state.ordering,
        SignUpFormViewModel::empty(),
        None,
        None,
    );
    render_template(SignUpTemplate { vm })
}

async fn sign_up(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<SignUpForm>,
) -> impl IntoResponse {
    let delivery_window = if form.delivery_window.trim().is_empty() {
        None
    } else {
        Some(form.delivery_window.as_str())
    };

    let input = SignUpEmployeeInput::new(
        &form.email,
        &form.password,
        &form.first_name,
        &form.last_name,
        &form.title,
        &form.school,
        &form.phone,
        delivery_window,
    );

    let interactor = SignUpEmployeeInteractor::new(&state.ordering.employees, &state.ordering.schools);
    match interactor.execute(input) {
        Ok(output) => {
            let success = OrderingPresenter::sign_up_success(&output).message;
            let vm = build_sign_up_vm(
                &state.ordering,
                SignUpFormViewModel::empty(),
                None,
                Some(success),
            );
            if is_htmx(&headers) {
                render_template(SignUpFormTemplate { vm })
            } else {
                render_template(SignUpTemplate { vm })
            }
        }
        Err(err) => {
            let vm = build_sign_up_vm(
                &state.ordering,
                SignUpFormViewModel {
                    email: form.email,
                    password: String::new(),
                    first_name: form.first_name,
                    last_name: form.last_name,
                    title: form.title,
                    school: form.school,
                    phone: form.phone,
                    delivery_window: form.delivery_window,
                },
                Some(err.message),
                None,
            );
            if is_htmx(&headers) {
                render_template(SignUpFormTemplate { vm })
            } else {
                render_template(SignUpTemplate { vm })
            }
        }
    }
}

async fn sign_out(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let interactor = SignOutEmployeeInteractor::new(&ordering.sessions);
    if let Err(AuthError { message }) =
        interactor.execute(SignOutEmployeeInput::new(&email))
    {
        eprintln!("sign out failed: {message}");
    }

    redirect_with_cookie("/ordering/sign-in", Some(clear_session_cookie()))
}

async fn cart(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let cart_items = ordering
        .carts
        .items_for(&email)
        .unwrap_or_default();
    let cart_items_vm = OrderingPresenter::cart_items(&cart_items);
    let catalog_items = OrderingPresenter::catalog_items(&ordering.catalog_items);

    let quote_preview = build_quote_preview(ordering, &email);

    let vm = CartViewModel {
        title: "Cart".to_string(),
        employee_email: email,
        catalog_items,
        cart_items: cart_items_vm,
        cart_error: None,
        quote: quote_preview.quote,
        quote_error: quote_preview.quote_error,
        message: None,
    };

    render_template(CartTemplate { vm })
}

async fn add_to_cart(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<AddToCartForm>,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let mut error = None;
    if form.quantity == 0 {
        error = Some("Quantity must be at least 1.".to_string());
    } else {
        let interactor = AddItemToCartInteractor::new(&ordering.carts, &ordering.catalog);
        if let Err(err) = interactor.execute(AddItemToCartInput::new(
            &email,
            &form.description,
            form.quantity,
        )) {
            error = Some(err.message);
        }
    }

    let cart_items = ordering.carts.items_for(&email).unwrap_or_default();
    let vm = CartItemsViewModel {
        cart_items: OrderingPresenter::cart_items(&cart_items),
        cart_error: error,
    };
    let mut response = render_template(CartItemsTemplate { vm });
    response
        .headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("cart-updated"));
    response
}

async fn quote_preview(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let vm = build_quote_preview(ordering, &email);
    render_template(QuotePreviewTemplate { vm })
}

async fn quote(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let result = view_quote(ordering, &email);
    let (quote, error) = match result {
        Ok(quote) => (Some(OrderingPresenter::quote(quote)), None),
        Err(err) => (None, Some(err.message)),
    };
    let vm = QuotePageViewModel {
        title: "Quote review".to_string(),
        employee_email: email,
        quote,
        error,
    };

    render_template(QuoteTemplate { vm })
}

async fn submit_quote(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ordering = &state.ordering;
    let email = match authenticated_email(ordering, &headers) {
        Ok(email) => email,
        Err(response) => return response,
    };

    let interactor = SubmitQuoteInteractor::new(
        &ordering.carts,
        &ordering.employees,
        &ordering.schools,
        &ordering.rates,
        &ordering.clock,
        &ordering.quotes,
        &ordering.invoices,
        &ordering.email_outbox,
        &ordering.invoice_renderer,
    );

    match interactor.execute(SubmitQuoteInput::new(&email)) {
        Ok(output) => {
            let invoice = ordering
                .invoices
                .last()
                .unwrap_or(None)
                .map(|invoice| OrderingPresenter::invoice(invoice, &email));
            let message = OrderingPresenter::submit_quote_confirmation(&output);
            let vm = InvoicePageViewModel {
                title: "Invoice sent".to_string(),
                employee_email: email,
                invoice,
                message: Some(message),
                error: None,
            };
            render_template(InvoiceTemplate { vm })
        }
        Err(err) => {
            let vm = InvoicePageViewModel {
                title: "Invoice sent".to_string(),
                employee_email: email,
                invoice: None,
                message: None,
                error: Some(err.message),
            };
            render_template(InvoiceTemplate { vm })
        }
    }
}

fn build_quote_preview(ordering: &OrderingState, email: &str) -> QuotePreviewViewModel {
    let result = view_quote(ordering, email)
        .map_err(|err| err.message);
    OrderingPresenter::quote_preview(result)
}

fn view_quote(ordering: &OrderingState, email: &str) -> Result<core_entities::ordering::Quote, QuoteError> {
    let interactor = ViewQuoteInteractor::new(
        &ordering.carts,
        &ordering.employees,
        &ordering.schools,
        &ordering.rates,
        &ordering.clock,
        &ordering.quotes,
    );
    interactor.execute(ViewQuoteInput::new(email))
}

fn build_sign_up_vm(
    ordering: &OrderingState,
    form: SignUpFormViewModel,
    error: Option<String>,
    success: Option<String>,
) -> SignUpViewModel {
    let schools = ordering
        .school_options
        .iter()
        .map(|school| SchoolOptionViewModel {
            name: school.name.clone(),
        })
        .collect();

    SignUpViewModel {
        title: "Create an account".to_string(),
        subtitle: "Order supplies with a verified NYC DOE email.".to_string(),
        form,
        schools,
        error,
        success,
    }
}

fn authenticated_email(
    ordering: &OrderingState,
    headers: &HeaderMap,
) -> Result<String, Response> {
    let email = match session_cookie_value(headers) {
        Some(value) => value,
        None => return Err(redirect_with_cookie("/ordering/sign-in", None)),
    };
    let is_active = ordering.sessions.is_active(&email).unwrap_or(false);
    if is_active {
        Ok(email)
    } else {
        Err(redirect_with_cookie(
            "/ordering/sign-in",
            Some(clear_session_cookie()),
        ))
    }
}

fn session_cookie_value(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get("cookie")?.to_str().ok()?;
    raw.split(';').find_map(|pair| {
        let mut parts = pair.trim().splitn(2, '=');
        let name = parts.next()?;
        let value = parts.next()?;
        if name == SESSION_COOKIE_NAME {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn session_cookie(email: &str) -> HeaderValue {
    let value = format!(
        "{SESSION_COOKIE_NAME}={email}; Path=/; HttpOnly; SameSite=Lax"
    );
    HeaderValue::from_str(&value).unwrap_or_else(|_| HeaderValue::from_static(""))
}

fn clear_session_cookie() -> HeaderValue {
    let value = format!(
        "{SESSION_COOKIE_NAME}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax"
    );
    HeaderValue::from_str(&value).unwrap_or_else(|_| HeaderValue::from_static(""))
}

fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|value| value.to_str().ok())
        .map(|value| value == "true")
        .unwrap_or(false)
}

fn redirect_with_cookie(url: &str, cookie: Option<HeaderValue>) -> Response {
    let mut headers = HeaderMap::new();
    if let Some(cookie) = cookie {
        headers.insert(SET_COOKIE, cookie);
    }
    headers.insert(LOCATION, HeaderValue::from_str(url).unwrap());
    headers.insert(
        "HX-Redirect",
        HeaderValue::from_str(url).unwrap(),
    );
    (StatusCode::SEE_OTHER, headers).into_response()
}

fn render_template<T: Template>(template: T) -> Response {
    match template.render() {
        Ok(body) => Html(body).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response(),
    }
}
