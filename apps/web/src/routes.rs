use askama::Template;
use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use core_entities::{Employee, EmployeeId, ProductId, QuoteId};
use core_use_cases::{
    AddItemInput, AddItemToCartInteractor, ConfirmOrderInteractor, GetQuoteInteractor,
    GetQuoteDetailsInteractor, ListCatalogInteractor, ListQuotesInteractor, RegisterEmployeeInput,
    RegisterEmployeeInteractor, SignInInput, SignInInteractor, SignOutInput, SignOutInteractor,
    UpdateCartInput, UpdateCartInteractor, UseCaseError, ViewCartInteractor,
};
use serde::Deserialize;

use crate::http::AppState;
use crate::presenters::{
    CartPresenter, CatalogPresenter, ConfirmPresenter, QuoteDetailPresenter,
    QuoteHistoryPresenter, QuotePresenter, SignInPresenter,
};
use crate::view_models::{
    AuthViewModel, CatalogViewModel, CartViewModel, ErrorViewModel, InvoiceViewModel,
    LayoutViewModel, QuoteDetailViewModel, QuoteHistoryViewModel, QuoteViewModel,
};

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {
    title: String,
    layout: LayoutViewModel,
}

#[derive(Template)]
#[template(path = "signup.html")]
struct SignUpTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: AuthViewModel,
}

#[derive(Template)]
#[template(path = "signin.html")]
struct SignInTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: AuthViewModel,
}

#[derive(Template)]
#[template(path = "catalog.html")]
struct CatalogTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: CatalogViewModel,
}

#[derive(Template)]
#[template(path = "cart.html")]
struct CartTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: CartViewModel,
}

#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: QuoteViewModel,
}

#[derive(Template)]
#[template(path = "quotes.html")]
struct QuotesTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: QuoteHistoryViewModel,
}

#[derive(Template)]
#[template(path = "quote_detail.html")]
struct QuoteDetailTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: QuoteDetailViewModel,
}

#[derive(Template)]
#[template(path = "invoice.html")]
struct InvoiceTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: InvoiceViewModel,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    title: String,
    layout: LayoutViewModel,
    vm: ErrorViewModel,
}

#[derive(Template)]
#[template(path = "partials/cart_count.html")]
struct CartCountTemplate {
    count: u32,
}

#[derive(Template)]
#[template(path = "partials/cart_items.html")]
struct CartItemsTemplate {
    vm: CartViewModel,
}

#[derive(Template)]
#[template(path = "partials/add_to_cart_success.html")]
struct AddToCartSuccessTemplate;

#[derive(Deserialize)]
pub struct SignUpForm {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct AddToCartForm {
    product_id: u64,
    quantity: u32,
}

#[derive(Deserialize)]
pub struct UpdateCartForm {
    product_id: u64,
    quantity: u32,
}

pub async fn landing(State(state): State<AppState>, jar: CookieJar) -> Response {
    if current_employee(&state, &jar).await.is_ok() {
        return Redirect::to("/catalog").into_response();
    }

    let layout = LayoutViewModel {
        employee_name: None,
        cart_count: 0,
    };
    render_template(LandingTemplate {
        title: "NYC Pantry Collective".to_string(),
        layout,
    })
}

pub async fn signup(State(state): State<AppState>, jar: CookieJar) -> Response {
    if current_employee(&state, &jar).await.is_ok() {
        return Redirect::to("/catalog").into_response();
    }

    render_signup(&state, None).await
}

pub async fn signup_submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<SignUpForm>,
) -> Response {
    let interactor = RegisterEmployeeInteractor::new(state.employees.as_ref());
    if let Err(err) = interactor
        .execute(RegisterEmployeeInput {
            name: form.name.clone(),
            email: form.email.clone(),
            password: form.password.clone(),
        })
        .await
    {
        return render_signup(&state, Some(user_error_message(&err))).await;
    }

    let sign_in_interactor =
        SignInInteractor::new(state.employees.as_ref(), state.sessions.as_ref());
    let sign_in_output = match sign_in_interactor
        .execute(SignInInput {
            email: form.email,
            password: form.password,
        })
        .await
    {
        Ok(output) => output,
        Err(err) => return render_signup(&state, Some(user_error_message(&err))).await,
    };

    let session = SignInPresenter::present(sign_in_output);
    let jar = set_session_cookie(jar, &session.session_token);
    (jar, Redirect::to("/catalog")).into_response()
}

pub async fn signin(State(state): State<AppState>, jar: CookieJar) -> Response {
    if current_employee(&state, &jar).await.is_ok() {
        return Redirect::to("/catalog").into_response();
    }

    render_signin(&state, None).await
}

pub async fn signin_submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<SignInForm>,
) -> Response {
    let interactor = SignInInteractor::new(state.employees.as_ref(), state.sessions.as_ref());
    let sign_in_output = match interactor
        .execute(SignInInput {
            email: form.email,
            password: form.password,
        })
        .await
    {
        Ok(output) => output,
        Err(err) => return render_signin(&state, Some(user_error_message(&err))).await,
    };

    let session = SignInPresenter::present(sign_in_output);
    let jar = set_session_cookie(jar, &session.session_token);
    (jar, Redirect::to("/catalog")).into_response()
}

pub async fn signout(State(state): State<AppState>, jar: CookieJar) -> Response {
    if let Some(token) = jar.get("session_id").map(|cookie| cookie.value().to_string()) {
        let interactor = SignOutInteractor::new(state.sessions.as_ref());
        let _ = interactor
            .execute(SignOutInput { token })
            .await
            .map_err(|err| {
                eprintln!("signout error: {}", err);
                err
            });
    }

    let jar = clear_session_cookie(jar);
    (jar, Redirect::to("/")).into_response()
}

pub async fn catalog(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    render_catalog(&state, &employee, None).await
}

pub async fn add_to_cart(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Form(form): Form<AddToCartForm>,
) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor =
        AddItemToCartInteractor::new(state.catalog.as_ref(), state.carts.as_ref());
    let add_result = interactor
        .execute(AddItemInput {
            employee_id: employee.id.clone(),
            product_id: ProductId(form.product_id),
            quantity: form.quantity,
        })
        .await;

    if let Err(err) = add_result {
        if is_htmx_request(&headers) {
            let mut vm = CatalogViewModel {
                products: vec![],
                error: Some(user_error_message(&err)),
            };
            let interactor = ListCatalogInteractor::new(state.catalog.as_ref());
            if let Ok(output) = interactor.execute().await {
                vm = CatalogPresenter::present(output);
                vm.error = Some(user_error_message(&err));
            }
            let layout = build_layout(&state, Some(&employee)).await;
            return render_template(CatalogTemplate {
                title: "Order supplies".to_string(),
                layout,
                vm,
            });
        }
        return render_catalog(&state, &employee, Some(user_error_message(&err))).await;
    }

    if is_htmx_request(&headers) {
        let cart_count = cart_count(&state, &employee.id).await;
        if let (Ok(count_html), Ok(success_html)) = (
            CartCountTemplate { count: cart_count }.render(),
            AddToCartSuccessTemplate.render(),
        ) {
            return Html(format!("{}\n{}", count_html, success_html)).into_response();
        }
        return (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response();
    }

    Redirect::to("/cart").into_response()
}

pub async fn cart(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    render_cart(&state, &employee, None).await
}

pub async fn cart_count_htmx(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(_) => return (StatusCode::UNAUTHORIZED, "").into_response(),
    };

    let count = cart_count(&state, &employee.id).await;
    render_template(CartCountTemplate { count })
}

pub async fn update_cart(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Form(form): Form<UpdateCartForm>,
) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor = UpdateCartInteractor::new(state.catalog.as_ref(), state.carts.as_ref());
    let update_output = match interactor
        .execute(UpdateCartInput {
            employee_id: employee.id.clone(),
            product_id: ProductId(form.product_id),
            quantity: form.quantity,
        })
        .await
    {
        Ok(output) => output,
        Err(err) => {
            if is_htmx_request(&headers) {
                let view_interactor =
                    ViewCartInteractor::new(state.catalog.as_ref(), state.carts.as_ref());
            let mut vm = match view_interactor.execute(employee.id.clone()).await {
                Ok(output) => CartPresenter::present(output),
                Err(_) => CartViewModel {
                    items: Vec::new(),
                    subtotal: "$0.00".to_string(),
                    is_empty: true,
                    error: None,
                },
            };
            vm.error = Some(user_error_message(&err));
            return render_template(CartItemsTemplate { vm });
        }
            return render_cart(&state, &employee, Some(user_error_message(&err))).await;
        }
    };

    if is_htmx_request(&headers) {
        let vm = CartPresenter::present(update_output);
        let cart_count = cart_count(&state, &employee.id).await;
        if let (Ok(items_html), Ok(count_html)) = (
            CartItemsTemplate { vm: vm.clone() }.render(),
            CartCountTemplate { count: cart_count }.render(),
        ) {
            return Html(format!("{}\n{}", items_html, count_html)).into_response();
        }
        return render_template(CartItemsTemplate { vm });
    }

    Redirect::to("/cart").into_response()
}

pub async fn quote(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor = GetQuoteInteractor::new(
        state.catalog.as_ref(),
        state.carts.as_ref(),
        state.quotes.as_ref(),
        state.pricing.clone(),
    );

    match interactor.execute(employee.id.clone()).await {
        Ok(output) => {
            let vm = QuotePresenter::present(output);
            let layout = build_layout(&state, Some(&employee)).await;
            render_template(QuoteTemplate {
                title: "Quote".to_string(),
                layout,
                vm,
            })
        }
        Err(UseCaseError::EmptyCart) => Redirect::to("/cart").into_response(),
        Err(err) => render_error(&state, &employee, "Quote error", err).await,
    }
}

pub async fn quotes(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor = ListQuotesInteractor::new(state.quotes.as_ref());
    let output = match interactor.execute(employee.id.clone()).await {
        Ok(output) => output,
        Err(err) => return render_error(&state, &employee, "Quotes error", err).await,
    };

    let vm = QuoteHistoryPresenter::present(output);
    let layout = build_layout(&state, Some(&employee)).await;
    render_template(QuotesTemplate {
        title: "Submitted quotes".to_string(),
        layout,
        vm,
    })
}

pub async fn quote_details(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(quote_id): Path<u64>,
) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor = GetQuoteDetailsInteractor::new(state.quotes.as_ref());
    let output = match interactor
        .execute(employee.id.clone(), QuoteId(quote_id))
        .await
    {
        Ok(output) => output,
        Err(err) => return render_error(&state, &employee, "Quote details error", err).await,
    };

    let vm = QuoteDetailPresenter::present(output);
    let layout = build_layout(&state, Some(&employee)).await;
    render_template(QuoteDetailTemplate {
        title: "Quote details".to_string(),
        layout,
        vm,
    })
}

pub async fn confirm_order(State(state): State<AppState>, jar: CookieJar) -> Response {
    let employee = match current_employee(&state, &jar).await {
        Ok(employee) => employee,
        Err(redirect) => return redirect.into_response(),
    };

    let interactor = ConfirmOrderInteractor::new(
        state.catalog.as_ref(),
        state.carts.as_ref(),
        state.invoices.as_ref(),
        state.employees.as_ref(),
        state.email.as_ref(),
        state.pricing.clone(),
    );

    match interactor.execute(employee.id.clone()).await {
        Ok(output) => {
            let vm = ConfirmPresenter::present(output);
            let layout = build_layout(&state, Some(&employee)).await;
            render_template(InvoiceTemplate {
                title: "Invoice confirmed".to_string(),
                layout,
                vm,
            })
        }
        Err(UseCaseError::EmptyCart) => Redirect::to("/cart").into_response(),
        Err(err) => render_error(&state, &employee, "Invoice error", err).await,
    }
}

async fn render_signup(state: &AppState, error: Option<String>) -> Response {
    let layout = build_layout(state, None).await;
    render_template(SignUpTemplate {
        title: "Create account".to_string(),
        layout,
        vm: AuthViewModel { error },
    })
}

async fn render_signin(state: &AppState, error: Option<String>) -> Response {
    let layout = build_layout(state, None).await;
    render_template(SignInTemplate {
        title: "Sign in".to_string(),
        layout,
        vm: AuthViewModel { error },
    })
}

async fn render_catalog(
    state: &AppState,
    employee: &Employee,
    error: Option<String>,
) -> Response {
    let interactor = ListCatalogInteractor::new(state.catalog.as_ref());
    let output = match interactor.execute().await {
        Ok(output) => output,
        Err(err) => return render_error(state, employee, "Catalog error", err).await,
    };
    let mut vm = CatalogPresenter::present(output);
    vm.error = error;
    let layout = build_layout(state, Some(employee)).await;
    render_template(CatalogTemplate {
        title: "Order supplies".to_string(),
        layout,
        vm,
    })
}

async fn render_cart(
    state: &AppState,
    employee: &Employee,
    error: Option<String>,
) -> Response {
    let interactor =
        ViewCartInteractor::new(state.catalog.as_ref(), state.carts.as_ref());
    let output = match interactor.execute(employee.id.clone()).await {
        Ok(output) => output,
        Err(err) => return render_error(state, employee, "Cart error", err).await,
    };
    let mut vm = CartPresenter::present(output);
    vm.error = error;
    let layout = build_layout(state, Some(employee)).await;
    render_template(CartTemplate {
        title: "Your cart".to_string(),
        layout,
        vm,
    })
}

async fn render_error(
    state: &AppState,
    employee: &Employee,
    title: &str,
    err: UseCaseError,
) -> Response {
    let layout = build_layout(state, Some(employee)).await;
    render_template(ErrorTemplate {
        title: title.to_string(),
        layout,
        vm: ErrorViewModel {
            title: title.to_string(),
            message: user_error_message(&err),
        },
    })
}

async fn build_layout(state: &AppState, employee: Option<&Employee>) -> LayoutViewModel {
    if let Some(employee) = employee {
        let cart_count = cart_count(state, &employee.id).await;
        LayoutViewModel {
            employee_name: Some(employee.name.clone()),
            cart_count,
        }
    } else {
        LayoutViewModel {
            employee_name: None,
            cart_count: 0,
        }
    }
}

async fn cart_count(state: &AppState, employee_id: &EmployeeId) -> u32 {
    match state.carts.get_cart(employee_id.clone()).await {
        Ok(Some(cart)) => cart.items.iter().map(|item| item.quantity).sum(),
        Ok(None) => 0,
        Err(err) => {
            eprintln!("cart count error: {}", err.message);
            0
        }
    }
}

async fn current_employee(
    state: &AppState,
    jar: &CookieJar,
) -> Result<Employee, Redirect> {
    let token = jar
        .get("session_id")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| Redirect::to("/signin"))?;

    let session = state
        .sessions
        .get_session(&token)
        .await
        .map_err(|_| Redirect::to("/signin"))?
        .ok_or_else(|| Redirect::to("/signin"))?;

    let employee = state
        .employees
        .get_by_id(session.employee_id)
        .await
        .map_err(|_| Redirect::to("/signin"))?
        .ok_or_else(|| Redirect::to("/signin"))?;

    Ok(employee)
}

fn set_session_cookie(jar: CookieJar, token: &str) -> CookieJar {
    let cookie = Cookie::build(("session_id", token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    jar.add(cookie)
}

fn clear_session_cookie(jar: CookieJar) -> CookieJar {
    let cookie = Cookie::build(("session_id", "")).path("/").build();
    jar.remove(cookie)
}

fn user_error_message(err: &UseCaseError) -> String {
    match err {
        UseCaseError::Validation(message) => message.clone(),
        UseCaseError::NotFound(message) => message.clone(),
        UseCaseError::Unauthorized => "Incorrect email or password.".to_string(),
        UseCaseError::EmptyCart => "Your cart is empty.".to_string(),
        UseCaseError::Repo(_) | UseCaseError::Email(_) => {
            "Something went wrong. Please try again.".to_string()
        }
    }
}

fn render_template<T: Template>(template: T) -> Response {
    match template.render() {
        Ok(body) => Html(body).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response(),
    }
}

fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers
        .get("hx-request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false)
}
