use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Invoice, Quote, School};
use core_ports::ordering::{
    CartRepository, Clock, EmailMessage, EmailOutbox, EmployeeRepository, InvoiceRenderer,
    InvoiceRepository, QuoteRepository, RateProvider, SchoolRepository,
};
use core_ports::RepoError;
use core_use_cases::ordering::{SubmitQuoteInput, SubmitQuoteInteractor};
use std::cell::RefCell;
use std::collections::HashMap;

struct InMemoryCartRepository {
    carts: RefCell<HashMap<String, Vec<CartLineItem>>>,
}

impl InMemoryCartRepository {
    fn new() -> Self {
        Self {
            carts: RefCell::new(HashMap::new()),
        }
    }
}

impl CartRepository for InMemoryCartRepository {
    fn add_item(&self, email: &str, item: CartLineItem) -> Result<(), RepoError> {
        let mut carts = self.carts.borrow_mut();
        carts.entry(email.to_string()).or_default().push(item);
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

struct InMemoryEmployeeRepository {
    employees: RefCell<HashMap<String, Employee>>,
}

impl InMemoryEmployeeRepository {
    fn new() -> Self {
        Self {
            employees: RefCell::new(HashMap::new()),
        }
    }
}

impl EmployeeRepository for InMemoryEmployeeRepository {
    fn insert(&self, employee: Employee) -> Result<(), RepoError> {
        self.employees
            .borrow_mut()
            .insert(employee.email.clone(), employee);
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, RepoError> {
        Ok(self.employees.borrow().get(email).cloned())
    }
}

struct InMemorySchoolRepository {
    schools: RefCell<HashMap<String, School>>,
}

impl InMemorySchoolRepository {
    fn new() -> Self {
        Self {
            schools: RefCell::new(HashMap::new()),
        }
    }
}

impl SchoolRepository for InMemorySchoolRepository {
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

struct InMemoryRateProvider {
    tax_rate: RefCell<f64>,
    shipping_rate: RefCell<f64>,
}

impl InMemoryRateProvider {
    fn new() -> Self {
        Self {
            tax_rate: RefCell::new(0.0),
            shipping_rate: RefCell::new(0.0),
        }
    }

    fn set_tax_rate(&self, rate: f64) {
        *self.tax_rate.borrow_mut() = rate;
    }

    fn set_shipping_rate(&self, rate: f64) {
        *self.shipping_rate.borrow_mut() = rate;
    }
}

impl RateProvider for InMemoryRateProvider {
    fn tax_rate(&self) -> Result<f64, RepoError> {
        Ok(*self.tax_rate.borrow())
    }

    fn shipping_rate(&self) -> Result<f64, RepoError> {
        Ok(*self.shipping_rate.borrow())
    }
}

struct FixedClock {
    date: RefCell<Option<CalendarDate>>,
}

impl FixedClock {
    fn new() -> Self {
        Self {
            date: RefCell::new(None),
        }
    }

    fn set_today(&self, date: CalendarDate) {
        *self.date.borrow_mut() = Some(date);
    }
}

impl Clock for FixedClock {
    fn today(&self) -> Result<CalendarDate, RepoError> {
        self.date
            .borrow()
            .clone()
            .ok_or_else(|| RepoError::new("date not set"))
    }
}

struct InMemoryQuoteRepository {
    last: RefCell<Option<Quote>>,
}

impl InMemoryQuoteRepository {
    fn new() -> Self {
        Self {
            last: RefCell::new(None),
        }
    }
}

impl QuoteRepository for InMemoryQuoteRepository {
    fn save(&self, quote: Quote) -> Result<(), RepoError> {
        *self.last.borrow_mut() = Some(quote);
        Ok(())
    }

    fn last(&self) -> Result<Option<Quote>, RepoError> {
        Ok(self.last.borrow().clone())
    }
}

struct InMemoryInvoiceRepository {
    last: RefCell<Option<Invoice>>,
}

impl InMemoryInvoiceRepository {
    fn new() -> Self {
        Self {
            last: RefCell::new(None),
        }
    }

    fn last(&self) -> Option<Invoice> {
        self.last.borrow().clone()
    }
}

impl InvoiceRepository for InMemoryInvoiceRepository {
    fn save(&self, invoice: Invoice) -> Result<(), RepoError> {
        *self.last.borrow_mut() = Some(invoice);
        Ok(())
    }

    fn last(&self) -> Result<Option<Invoice>, RepoError> {
        Ok(self.last.borrow().clone())
    }
}

struct InMemoryEmailOutbox {
    last: RefCell<Option<EmailMessage>>,
}

impl InMemoryEmailOutbox {
    fn new() -> Self {
        Self {
            last: RefCell::new(None),
        }
    }

    fn last(&self) -> Option<EmailMessage> {
        self.last.borrow().clone()
    }
}

impl EmailOutbox for InMemoryEmailOutbox {
    fn send(&self, email: EmailMessage) -> Result<(), RepoError> {
        *self.last.borrow_mut() = Some(email);
        Ok(())
    }

    fn last(&self) -> Result<Option<EmailMessage>, RepoError> {
        Ok(self.last.borrow().clone())
    }
}

struct StubInvoiceRenderer {
    bytes: Vec<u8>,
}

impl StubInvoiceRenderer {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl InvoiceRenderer for StubInvoiceRenderer {
    fn render_pdf(&self, _invoice: &Invoice) -> Result<Vec<u8>, RepoError> {
        Ok(self.bytes.clone())
    }
}

#[test]
fn submit_quote_creates_invoice_emails_and_clears_cart() {
    let carts = InMemoryCartRepository::new();
    let employees = InMemoryEmployeeRepository::new();
    let schools = InMemorySchoolRepository::new();
    let rates = InMemoryRateProvider::new();
    let clock = FixedClock::new();
    let quotes = InMemoryQuoteRepository::new();
    let invoices = InMemoryInvoiceRepository::new();
    let email_outbox = InMemoryEmailOutbox::new();
    let invoice_renderer = StubInvoiceRenderer::new(vec![1, 2, 3]);

    let employee = Employee::fixture(
        "QWilliams@schools.nyc.gov",
        "P.S. 082 - The Hammond School",
        "School Hours",
    );
    employees.insert(employee).expect("employee should be stored");
    schools
        .upsert(School {
            name: "P.S. 082 - The Hammond School".to_string(),
            code: Some("28Q082".to_string()),
        })
        .expect("school should be stored");
    carts
        .replace_cart(
            "QWilliams@schools.nyc.gov",
            vec![CartLineItem::new(
                "Poland Spring Water (48 ct/8 oz)",
                8,
                20.00,
            )],
        )
        .expect("cart should be stored");
    clock.set_today(CalendarDate::parse_mmddyy("12/22/25"));
    rates.set_tax_rate(0.08875);
    rates.set_shipping_rate(0.01);

    let interactor = SubmitQuoteInteractor::new(
        &carts,
        &employees,
        &schools,
        &rates,
        &clock,
        &quotes,
        &invoices,
        &email_outbox,
        &invoice_renderer,
    );

    let output = interactor
        .execute(SubmitQuoteInput::new("QWilliams@schools.nyc.gov"))
        .expect("submit quote should succeed");

    assert_eq!(output.quote_number.as_str(), "28Q082x122225");
    let invoice = invoices
        .last()
        .expect("invoice should be created");
    assert_eq!(invoice.quote_number.as_str(), "28Q082x122225");
    assert!(invoice.tax.is_some(), "invoice tax should be populated");
    assert!(invoice.shipping.is_some(), "invoice shipping should be populated");

    let email = email_outbox
        .last()
        .expect("invoice email should be sent");
    assert_eq!(email.to.as_str(), "QWilliams@schools.nyc.gov");
    let has_pdf = email
        .attachments
        .iter()
        .any(|attachment| attachment.content_type == "application/pdf");
    assert!(has_pdf, "expected PDF attachment");

    let body = match email.body {
        core_ports::ordering::EmailBody::Structured(body) => body,
    };
    assert_eq!(body.school.as_str(), "P.S. 082 - The Hammond School");
    assert_eq!(body.delivery_window.as_str(), "School Hours");
    assert_eq!(
        body.line_item_description.as_str(),
        "Poland Spring Water (48 ct/8 oz)"
    );
    assert_eq!(body.line_item_quantity, 8);
    assert!((body.line_item_price - 20.00).abs() <= 0.0001);

    let cart_items = carts
        .items_for("QWilliams@schools.nyc.gov")
        .expect("cart should be queryable");
    assert!(cart_items.is_empty(), "expected cart to be cleared");
}
