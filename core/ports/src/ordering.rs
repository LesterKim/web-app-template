use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Invoice, Quote, School};

use crate::RepoError;

#[derive(Clone, Debug)]
pub struct EmailMessage {
    pub to: String,
    pub body: EmailBody,
    pub attachments: Vec<EmailAttachment>,
}

#[derive(Clone, Debug)]
pub enum EmailBody {
    Structured(StructuredEmailBody),
}

#[derive(Clone, Debug)]
pub struct StructuredEmailBody {
    pub school: String,
    pub delivery_window: String,
    pub line_item_description: String,
    pub line_item_quantity: u32,
    pub line_item_price: f64,
}

#[derive(Clone, Debug)]
pub struct EmailAttachment {
    pub content_type: String,
    pub file_name: Option<String>,
    pub bytes: Vec<u8>,
}

pub trait SchoolRepository {
    fn upsert(&self, school: School) -> Result<(), RepoError>;
    fn find_by_name(&self, name: &str) -> Result<Option<School>, RepoError>;
}

pub trait EmployeeRepository {
    fn insert(&self, employee: Employee) -> Result<(), RepoError>;
    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, RepoError>;
}

pub trait SessionStore {
    fn start_session(&self, email: &str) -> Result<(), RepoError>;
    fn end_session(&self, email: &str) -> Result<(), RepoError>;
    fn is_active(&self, email: &str) -> Result<bool, RepoError>;
}

pub trait CatalogRepository {
    fn price_for(&self, description: &str) -> Result<Option<f64>, RepoError>;
}

pub trait CartRepository {
    fn add_item(&self, email: &str, item: CartLineItem) -> Result<(), RepoError>;
    fn replace_cart(&self, email: &str, items: Vec<CartLineItem>) -> Result<(), RepoError>;
    fn items_for(&self, email: &str) -> Result<Vec<CartLineItem>, RepoError>;
    fn clear(&self, email: &str) -> Result<(), RepoError>;
}

pub trait QuoteRepository {
    fn save(&self, quote: Quote) -> Result<(), RepoError>;
    fn last(&self) -> Result<Option<Quote>, RepoError>;
}

pub trait InvoiceRepository {
    fn save(&self, invoice: Invoice) -> Result<(), RepoError>;
    fn last(&self) -> Result<Option<Invoice>, RepoError>;
}

pub trait EmailOutbox {
    fn send(&self, email: EmailMessage) -> Result<(), RepoError>;
    fn last(&self) -> Result<Option<EmailMessage>, RepoError>;
}

pub trait InvoiceRenderer {
    fn render_pdf(&self, invoice: &Invoice) -> Result<Vec<u8>, RepoError>;
}

pub trait Clock {
    fn today(&self) -> Result<CalendarDate, RepoError>;
}

pub trait RateProvider {
    fn tax_rate(&self) -> Result<f64, RepoError>;
    fn shipping_rate(&self) -> Result<f64, RepoError>;
}
