use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Quote, School};
use core_ports::ordering::{
    CartRepository, Clock, EmployeeRepository, QuoteRepository, RateProvider, SchoolRepository,
};
use core_ports::RepoError;
use core_use_cases::ordering::{ViewQuoteInput, ViewQuoteInteractor};
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

#[test]
fn view_quote_builds_totals_and_quote_number() {
    let carts = InMemoryCartRepository::new();
    let employees = InMemoryEmployeeRepository::new();
    let schools = InMemorySchoolRepository::new();
    let rates = InMemoryRateProvider::new();
    let clock = FixedClock::new();
    let quotes = InMemoryQuoteRepository::new();

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

    let interactor = ViewQuoteInteractor::new(
        &carts, &employees, &schools, &rates, &clock, &quotes,
    );

    let quote = interactor
        .execute(ViewQuoteInput::new("QWilliams@schools.nyc.gov"))
        .expect("view quote should succeed");

    assert_eq!(quote.number.as_str(), "28Q082x122225");
    assert_eq!(quote.line_items.len(), 1, "expected one quote line item");
    assert_eq!(quote.line_items[0].quantity, 8);
    assert_f64_close(quote.line_items[0].line_total, 160.00, "line total");
    assert_f64_close(quote.subtotal, 160.00, "subtotal");
    assert_f64_close(quote.tax, 160.00 * 0.08875, "tax");
    assert_f64_close(quote.shipping, 160.00 * 0.01, "shipping");
    assert_f64_close(quote.total, 160.00 + (160.00 * 0.08875) + (160.00 * 0.01), "total");
    assert_eq!(quote.delivery_window.as_str(), "School Hours");
}

fn assert_f64_close(actual: f64, expected: f64, label: &str) {
    let diff = (actual - expected).abs();
    assert!(diff <= 0.0001, "{label} expected {expected:.5}, got {actual:.5}");
}
