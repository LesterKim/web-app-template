use core_entities::{
    Cart, Employee, EmployeeId, Invoice, InvoiceDraft, InvoiceId, Product, ProductId, QuoteDraft,
    QuoteId, QuoteRecord, Session,
};
use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug)]
pub struct RepoError {
    pub message: String,
}

impl RepoError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmailError {
    pub message: String,
}

impl EmailError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait EmployeeRepository: Send + Sync {
    fn add_employee<'a>(
        &'a self,
        name: String,
        email: String,
        password: String,
    ) -> BoxFuture<'a, Result<Employee, RepoError>>;

    fn find_by_email<'a>(&'a self, email: &'a str)
        -> BoxFuture<'a, Result<Option<Employee>, RepoError>>;

    fn get_by_id<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>>;
}

pub trait CatalogRepository: Send + Sync {
    fn list_products<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Product>, RepoError>>;
    fn find_product<'a>(
        &'a self,
        product_id: ProductId,
    ) -> BoxFuture<'a, Result<Option<Product>, RepoError>>;
}

pub trait CartRepository: Send + Sync {
    fn get_cart<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Cart>, RepoError>>;
    fn save_cart<'a>(&'a self, cart: Cart) -> BoxFuture<'a, Result<(), RepoError>>;
    fn clear_cart<'a>(&'a self, employee_id: EmployeeId) -> BoxFuture<'a, Result<(), RepoError>>;
}

pub trait SessionRepository: Send + Sync {
    fn create_session<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Session, RepoError>>;
    fn get_session<'a>(
        &'a self,
        token: &'a str,
    ) -> BoxFuture<'a, Result<Option<Session>, RepoError>>;
    fn remove_session<'a>(&'a self, token: &'a str) -> BoxFuture<'a, Result<(), RepoError>>;
}

pub trait InvoiceRepository: Send + Sync {
    fn create_invoice<'a>(
        &'a self,
        draft: InvoiceDraft,
    ) -> BoxFuture<'a, Result<Invoice, RepoError>>;
    fn get_invoice<'a>(
        &'a self,
        invoice_id: InvoiceId,
    ) -> BoxFuture<'a, Result<Option<Invoice>, RepoError>>;
}

pub trait QuoteRepository: Send + Sync {
    fn create_quote<'a>(
        &'a self,
        draft: QuoteDraft,
    ) -> BoxFuture<'a, Result<QuoteRecord, RepoError>>;
    fn list_quotes<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Vec<QuoteRecord>, RepoError>>;
    fn get_quote<'a>(
        &'a self,
        employee_id: EmployeeId,
        quote_id: QuoteId,
    ) -> BoxFuture<'a, Result<Option<QuoteRecord>, RepoError>>;
}

pub trait EmailGateway: Send + Sync {
    fn send_invoice<'a>(
        &'a self,
        to: &'a str,
        invoice: &'a Invoice,
    ) -> BoxFuture<'a, Result<(), EmailError>>;
}
