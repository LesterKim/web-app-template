use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Invoice, Quote, School};
use core_ports::ordering::{
    CatalogRepository, CartRepository, Clock, EmailMessage, EmailOutbox, EmployeeRepository,
    InvoiceRenderer, InvoiceRepository, QuoteRepository, RateProvider, SchoolRepository,
    SessionStore,
};
use core_ports::RepoError;

#[derive(Clone)]
pub struct InMemorySchoolRepository {
    schools: Arc<Mutex<HashMap<String, School>>>,
}

impl InMemorySchoolRepository {
    pub fn new() -> Self {
        Self {
            schools: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn seed(&self, school: School) {
        self.schools
            .lock()
            .expect("school lock poisoned")
            .insert(school.name.clone(), school);
    }
}

impl SchoolRepository for InMemorySchoolRepository {
    fn upsert(&self, school: School) -> Result<(), RepoError> {
        self.schools
            .lock()
            .expect("school lock poisoned")
            .insert(school.name.clone(), school);
        Ok(())
    }

    fn find_by_name(&self, name: &str) -> Result<Option<School>, RepoError> {
        Ok(self
            .schools
            .lock()
            .expect("school lock poisoned")
            .get(name)
            .cloned())
    }
}

#[derive(Clone)]
pub struct InMemoryEmployeeRepository {
    employees: Arc<Mutex<HashMap<String, Employee>>>,
}

impl InMemoryEmployeeRepository {
    pub fn new() -> Self {
        Self {
            employees: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl EmployeeRepository for InMemoryEmployeeRepository {
    fn insert(&self, employee: Employee) -> Result<(), RepoError> {
        self.employees
            .lock()
            .expect("employee lock poisoned")
            .insert(employee.email.clone(), employee);
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, RepoError> {
        Ok(self
            .employees
            .lock()
            .expect("employee lock poisoned")
            .get(email)
            .cloned())
    }
}

#[derive(Clone)]
pub struct InMemorySessionStore {
    sessions: Arc<Mutex<HashSet<String>>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}

impl SessionStore for InMemorySessionStore {
    fn start_session(&self, email: &str) -> Result<(), RepoError> {
        self.sessions
            .lock()
            .expect("session lock poisoned")
            .insert(email.to_string());
        Ok(())
    }

    fn end_session(&self, email: &str) -> Result<(), RepoError> {
        self.sessions
            .lock()
            .expect("session lock poisoned")
            .remove(email);
        Ok(())
    }

    fn is_active(&self, email: &str) -> Result<bool, RepoError> {
        Ok(self
            .sessions
            .lock()
            .expect("session lock poisoned")
            .contains(email))
    }
}

#[derive(Clone)]
pub struct InMemoryCatalogRepository {
    items: Arc<Mutex<HashMap<String, f64>>>,
}

impl InMemoryCatalogRepository {
    pub fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn seed_item(&self, description: &str, price: f64) {
        self.items
            .lock()
            .expect("catalog lock poisoned")
            .insert(description.to_string(), price);
    }
}

impl CatalogRepository for InMemoryCatalogRepository {
    fn price_for(&self, description: &str) -> Result<Option<f64>, RepoError> {
        Ok(self
            .items
            .lock()
            .expect("catalog lock poisoned")
            .get(description)
            .copied())
    }
}

#[derive(Clone)]
pub struct InMemoryCartRepository {
    carts: Arc<Mutex<HashMap<String, Vec<CartLineItem>>>>,
}

impl InMemoryCartRepository {
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CartRepository for InMemoryCartRepository {
    fn add_item(&self, email: &str, item: CartLineItem) -> Result<(), RepoError> {
        let mut carts = self.carts.lock().expect("cart lock poisoned");
        carts.entry(email.to_string()).or_default().push(item);
        Ok(())
    }

    fn replace_cart(&self, email: &str, items: Vec<CartLineItem>) -> Result<(), RepoError> {
        self.carts
            .lock()
            .expect("cart lock poisoned")
            .insert(email.to_string(), items);
        Ok(())
    }

    fn items_for(&self, email: &str) -> Result<Vec<CartLineItem>, RepoError> {
        Ok(self
            .carts
            .lock()
            .expect("cart lock poisoned")
            .get(email)
            .cloned()
            .unwrap_or_default())
    }

    fn clear(&self, email: &str) -> Result<(), RepoError> {
        self.carts
            .lock()
            .expect("cart lock poisoned")
            .remove(email);
        Ok(())
    }
}

#[derive(Clone)]
pub struct InMemoryQuoteRepository {
    quotes: Arc<Mutex<Vec<Quote>>>,
}

impl InMemoryQuoteRepository {
    pub fn new() -> Self {
        Self {
            quotes: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl QuoteRepository for InMemoryQuoteRepository {
    fn save(&self, quote: Quote) -> Result<(), RepoError> {
        self.quotes
            .lock()
            .expect("quote lock poisoned")
            .push(quote);
        Ok(())
    }

    fn last(&self) -> Result<Option<Quote>, RepoError> {
        Ok(self
            .quotes
            .lock()
            .expect("quote lock poisoned")
            .last()
            .cloned())
    }
}

#[derive(Clone)]
pub struct InMemoryInvoiceRepository {
    invoices: Arc<Mutex<Vec<Invoice>>>,
}

impl InMemoryInvoiceRepository {
    pub fn new() -> Self {
        Self {
            invoices: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl InvoiceRepository for InMemoryInvoiceRepository {
    fn save(&self, invoice: Invoice) -> Result<(), RepoError> {
        self.invoices
            .lock()
            .expect("invoice lock poisoned")
            .push(invoice);
        Ok(())
    }

    fn last(&self) -> Result<Option<Invoice>, RepoError> {
        Ok(self
            .invoices
            .lock()
            .expect("invoice lock poisoned")
            .last()
            .cloned())
    }
}

#[derive(Clone)]
pub struct InMemoryEmailOutbox {
    sent: Arc<Mutex<Vec<EmailMessage>>>,
}

impl InMemoryEmailOutbox {
    pub fn new() -> Self {
        Self {
            sent: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EmailOutbox for InMemoryEmailOutbox {
    fn send(&self, email: EmailMessage) -> Result<(), RepoError> {
        self.sent
            .lock()
            .expect("email lock poisoned")
            .push(email);
        Ok(())
    }

    fn last(&self) -> Result<Option<EmailMessage>, RepoError> {
        Ok(self
            .sent
            .lock()
            .expect("email lock poisoned")
            .last()
            .cloned())
    }
}

#[derive(Clone)]
pub struct StubInvoiceRenderer {
    pdf_bytes: Arc<Vec<u8>>,
}

impl StubInvoiceRenderer {
    pub fn new() -> Self {
        Self {
            pdf_bytes: Arc::new(b"%PDF-1.4\n%NYC DOE Ordering\n".to_vec()),
        }
    }
}

impl InvoiceRenderer for StubInvoiceRenderer {
    fn render_pdf(&self, _invoice: &Invoice) -> Result<Vec<u8>, RepoError> {
        Ok((*self.pdf_bytes).clone())
    }
}

#[derive(Clone)]
pub struct FixedClock {
    today: CalendarDate,
}

impl FixedClock {
    pub fn new(today: CalendarDate) -> Self {
        Self { today }
    }
}

impl Clock for FixedClock {
    fn today(&self) -> Result<CalendarDate, RepoError> {
        Ok(self.today.clone())
    }
}

#[derive(Clone)]
pub struct FixedRateProvider {
    tax_rate: f64,
    shipping_rate: f64,
}

impl FixedRateProvider {
    pub fn new(tax_rate: f64, shipping_rate: f64) -> Self {
        Self {
            tax_rate,
            shipping_rate,
        }
    }
}

impl RateProvider for FixedRateProvider {
    fn tax_rate(&self) -> Result<f64, RepoError> {
        Ok(self.tax_rate)
    }

    fn shipping_rate(&self) -> Result<f64, RepoError> {
        Ok(self.shipping_rate)
    }
}
