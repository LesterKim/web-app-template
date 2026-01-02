use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Invoice, Quote, School};
use core_ports::ordering::{
    CatalogRepository, CartRepository, Clock, EmailMessage, EmailOutbox, EmployeeRepository,
    InvoiceRenderer, InvoiceRepository, QuoteRepository, RateProvider, SchoolRepository,
    SessionStore,
};
use core_ports::RepoError;
use core_use_cases::ordering::{
    AddItemToCartInput, AddItemToCartInteractor, AuthError, CartError, QuoteError,
    SignInEmployeeInput, SignInEmployeeInteractor, SignInEmployeeOutput, SignOutEmployeeInput,
    SignOutEmployeeInteractor, SignUpEmployeeInput, SignUpEmployeeInteractor, SignUpEmployeeOutput,
    SignUpError, SubmitQuoteInput, SubmitQuoteInteractor, SubmitQuoteOutput, ViewQuoteInput,
    ViewQuoteInteractor,
};

pub struct ProtocolDrivers {
    pub schools: InMemorySchoolRepo,
    pub users: InMemoryUserRepo,
    pub sessions: InMemorySessionStore,
    pub catalog: InMemoryCatalogRepo,
    pub carts: InMemoryCartRepo,
    pub quotes: InMemoryQuoteRepo,
    pub invoices: InMemoryInvoiceRepo,
    pub email_outbox: InMemoryEmailOutbox,
    pub clock: FixedClock,
    pub rates: FixedRateProvider,
    pub invoice_renderer: StubInvoiceRenderer,
}

impl ProtocolDrivers {
    pub fn new() -> Self {
        Self {
            schools: InMemorySchoolRepo::new(),
            users: InMemoryUserRepo::new(),
            sessions: InMemorySessionStore::new(),
            catalog: InMemoryCatalogRepo::new(),
            carts: InMemoryCartRepo::new(),
            quotes: InMemoryQuoteRepo::new(),
            invoices: InMemoryInvoiceRepo::new(),
            email_outbox: InMemoryEmailOutbox::new(),
            clock: FixedClock::new(),
            rates: FixedRateProvider::new(),
            invoice_renderer: StubInvoiceRenderer,
        }
    }
}

pub struct UseCaseDriver;

impl UseCaseDriver {
    pub fn new() -> Self {
        Self
    }

    pub fn sign_up(
        &self,
        drivers: &ProtocolDrivers,
        request: SignUpEmployeeInput,
    ) -> Result<SignUpEmployeeOutput, SignUpError> {
        let interactor = SignUpEmployeeInteractor::new(&drivers.users, &drivers.schools);
        interactor.execute(request)
    }

    pub fn sign_in(
        &self,
        drivers: &ProtocolDrivers,
        request: SignInEmployeeInput,
    ) -> Result<SignInEmployeeOutput, AuthError> {
        let interactor = SignInEmployeeInteractor::new(&drivers.users, &drivers.sessions);
        interactor.execute(request)
    }

    pub fn sign_out(
        &self,
        drivers: &ProtocolDrivers,
        request: SignOutEmployeeInput,
    ) -> Result<(), AuthError> {
        let interactor = SignOutEmployeeInteractor::new(&drivers.sessions);
        interactor.execute(request)
    }

    pub fn add_item_to_cart(
        &self,
        drivers: &ProtocolDrivers,
        request: AddItemToCartInput,
    ) -> Result<(), CartError> {
        let interactor = AddItemToCartInteractor::new(&drivers.carts, &drivers.catalog);
        interactor.execute(request)
    }

    pub fn view_quote(
        &self,
        drivers: &ProtocolDrivers,
        request: ViewQuoteInput,
    ) -> Result<Quote, QuoteError> {
        let interactor = ViewQuoteInteractor::new(
            &drivers.carts,
            &drivers.users,
            &drivers.schools,
            &drivers.rates,
            &drivers.clock,
            &drivers.quotes,
        );
        interactor.execute(request)
    }

    pub fn submit_quote(
        &self,
        drivers: &ProtocolDrivers,
        request: SubmitQuoteInput,
    ) -> Result<SubmitQuoteOutput, QuoteError> {
        let interactor = SubmitQuoteInteractor::new(
            &drivers.carts,
            &drivers.users,
            &drivers.schools,
            &drivers.rates,
            &drivers.clock,
            &drivers.quotes,
            &drivers.invoices,
            &drivers.email_outbox,
            &drivers.invoice_renderer,
        );
        interactor.execute(request)
    }
}

pub struct InMemorySchoolRepo {
    schools: RefCell<HashMap<String, School>>,
}

impl InMemorySchoolRepo {
    pub fn new() -> Self {
        Self {
            schools: RefCell::new(HashMap::new()),
        }
    }

    pub fn ensure(&self, name: &str) {
        self.schools
            .borrow_mut()
            .entry(name.to_string())
            .or_insert(School {
                name: name.to_string(),
                code: None,
            });
    }

    pub fn upsert(&self, name: &str, code: Option<&str>) {
        let mut schools = self.schools.borrow_mut();
        let entry = schools.entry(name.to_string()).or_insert(School {
            name: name.to_string(),
            code: None,
        });
        entry.code = code.map(str::to_string);
    }
}

impl SchoolRepository for InMemorySchoolRepo {
    fn upsert(&self, school: School) -> Result<(), RepoError> {
        self.schools
            .borrow_mut()
            .insert(school.name.clone(), school);
        Ok(())
    }

    fn find_by_name(&self, name: &str) -> Result<Option<School>, RepoError> {
        Ok(self.schools.borrow().get(name).cloned())
    }
}

pub struct InMemoryUserRepo {
    accounts: RefCell<HashMap<String, Employee>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            accounts: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, account: Employee) {
        self.accounts
            .borrow_mut()
            .insert(account.email.clone(), account);
    }

    pub fn exists(&self, email: &str) -> bool {
        self.accounts.borrow().contains_key(email)
    }

    pub fn get(&self, email: &str) -> Option<Employee> {
        self.accounts.borrow().get(email).cloned()
    }

    pub fn school_name_for(&self, email: &str) -> Option<String> {
        self.accounts
            .borrow()
            .get(email)
            .map(|account| account.school_name.clone())
    }
}

impl EmployeeRepository for InMemoryUserRepo {
    fn insert(&self, employee: Employee) -> Result<(), RepoError> {
        self.accounts
            .borrow_mut()
            .insert(employee.email.clone(), employee);
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, RepoError> {
        Ok(self.accounts.borrow().get(email).cloned())
    }
}

pub struct InMemorySessionStore {
    active_sessions: RefCell<HashSet<String>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            active_sessions: RefCell::new(HashSet::new()),
        }
    }

    pub fn start(&self, email: &str) {
        self.active_sessions.borrow_mut().insert(email.to_string());
    }

    pub fn is_active(&self, email: &str) -> bool {
        self.active_sessions.borrow().contains(email)
    }
}

impl SessionStore for InMemorySessionStore {
    fn start_session(&self, email: &str) -> Result<(), RepoError> {
        self.active_sessions.borrow_mut().insert(email.to_string());
        Ok(())
    }

    fn end_session(&self, email: &str) -> Result<(), RepoError> {
        self.active_sessions.borrow_mut().remove(email);
        Ok(())
    }

    fn is_active(&self, email: &str) -> Result<bool, RepoError> {
        Ok(self.active_sessions.borrow().contains(email))
    }
}

pub struct InMemoryCatalogRepo {
    items: RefCell<HashMap<String, f64>>,
}

impl InMemoryCatalogRepo {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(HashMap::new()),
        }
    }

    pub fn upsert(&self, description: &str, price: f64) {
        self.items
            .borrow_mut()
            .insert(description.to_string(), price);
    }
}

impl CatalogRepository for InMemoryCatalogRepo {
    fn price_for(&self, description: &str) -> Result<Option<f64>, RepoError> {
        Ok(self.items.borrow().get(description).copied())
    }
}

pub struct InMemoryCartRepo {
    carts: RefCell<HashMap<String, Vec<CartLineItem>>>,
}

impl InMemoryCartRepo {
    pub fn new() -> Self {
        Self {
            carts: RefCell::new(HashMap::new()),
        }
    }

    pub fn replace_cart(&self, email: &str, items: Vec<CartLineItem>) {
        self.carts.borrow_mut().insert(email.to_string(), items);
    }

    pub fn items_for(&self, email: &str) -> Vec<CartLineItem> {
        self.carts
            .borrow()
            .get(email)
            .cloned()
            .unwrap_or_default()
    }
}

impl CartRepository for InMemoryCartRepo {
    fn add_item(&self, email: &str, item: CartLineItem) -> Result<(), RepoError> {
        let mut carts = self.carts.borrow_mut();
        let entry = carts.entry(email.to_string()).or_insert(Vec::new());
        entry.push(item);
        Ok(())
    }

    fn replace_cart(&self, email: &str, items: Vec<CartLineItem>) -> Result<(), RepoError> {
        self.carts.borrow_mut().insert(email.to_string(), items);
        Ok(())
    }

    fn items_for(&self, email: &str) -> Result<Vec<CartLineItem>, RepoError> {
        Ok(self
            .carts
            .borrow()
            .get(email)
            .cloned()
            .unwrap_or_default())
    }

    fn clear(&self, email: &str) -> Result<(), RepoError> {
        self.carts.borrow_mut().remove(email);
        Ok(())
    }
}

pub struct InMemoryQuoteRepo {
    quotes: RefCell<Vec<Quote>>,
}

impl InMemoryQuoteRepo {
    pub fn new() -> Self {
        Self {
            quotes: RefCell::new(Vec::new()),
        }
    }
}

impl QuoteRepository for InMemoryQuoteRepo {
    fn save(&self, quote: Quote) -> Result<(), RepoError> {
        self.quotes.borrow_mut().push(quote);
        Ok(())
    }

    fn last(&self) -> Result<Option<Quote>, RepoError> {
        Ok(self.quotes.borrow().last().cloned())
    }
}

pub struct InMemoryInvoiceRepo {
    invoices: RefCell<Vec<Invoice>>,
}

impl InMemoryInvoiceRepo {
    pub fn new() -> Self {
        Self {
            invoices: RefCell::new(Vec::new()),
        }
    }

    pub fn last(&self) -> Option<Invoice> {
        self.invoices.borrow().last().cloned()
    }
}

impl InvoiceRepository for InMemoryInvoiceRepo {
    fn save(&self, invoice: Invoice) -> Result<(), RepoError> {
        self.invoices.borrow_mut().push(invoice);
        Ok(())
    }

    fn last(&self) -> Result<Option<Invoice>, RepoError> {
        Ok(self.invoices.borrow().last().cloned())
    }
}

pub struct InMemoryEmailOutbox {
    sent: RefCell<Vec<EmailMessage>>,
}

impl InMemoryEmailOutbox {
    pub fn new() -> Self {
        Self {
            sent: RefCell::new(Vec::new()),
        }
    }

    pub fn last(&self) -> Option<EmailMessage> {
        self.sent.borrow().last().cloned()
    }
}

impl EmailOutbox for InMemoryEmailOutbox {
    fn send(&self, email: EmailMessage) -> Result<(), RepoError> {
        self.sent.borrow_mut().push(email);
        Ok(())
    }

    fn last(&self) -> Result<Option<EmailMessage>, RepoError> {
        Ok(self.sent.borrow().last().cloned())
    }
}

pub struct FixedClock {
    today: RefCell<Option<CalendarDate>>,
}

impl FixedClock {
    pub fn new() -> Self {
        Self {
            today: RefCell::new(None),
        }
    }

    pub fn set_today(&self, date: CalendarDate) {
        *self.today.borrow_mut() = Some(date);
    }
}

impl Clock for FixedClock {
    fn today(&self) -> Result<CalendarDate, RepoError> {
        self.today
            .borrow()
            .clone()
            .ok_or_else(|| RepoError::new("today not set"))
    }
}

pub struct FixedRateProvider {
    tax_rate: RefCell<Option<f64>>,
    shipping_rate: RefCell<Option<f64>>,
}

impl FixedRateProvider {
    pub fn new() -> Self {
        Self {
            tax_rate: RefCell::new(None),
            shipping_rate: RefCell::new(None),
        }
    }

    pub fn set_tax_rate(&self, rate: f64) {
        *self.tax_rate.borrow_mut() = Some(rate);
    }

    pub fn set_shipping_rate(&self, rate: f64) {
        *self.shipping_rate.borrow_mut() = Some(rate);
    }

    pub fn current_tax_rate(&self) -> Option<f64> {
        *self.tax_rate.borrow()
    }

    pub fn current_shipping_rate(&self) -> Option<f64> {
        *self.shipping_rate.borrow()
    }
}

impl RateProvider for FixedRateProvider {
    fn tax_rate(&self) -> Result<f64, RepoError> {
        (*self.tax_rate.borrow())
            .ok_or_else(|| RepoError::new("tax rate not set"))
    }

    fn shipping_rate(&self) -> Result<f64, RepoError> {
        (*self.shipping_rate.borrow())
            .ok_or_else(|| RepoError::new("shipping rate not set"))
    }
}

pub struct StubInvoiceRenderer;

impl InvoiceRenderer for StubInvoiceRenderer {
    fn render_pdf(&self, _invoice: &Invoice) -> Result<Vec<u8>, RepoError> {
        Ok(vec![37, 80, 68, 70])
    }
}
