use core_entities::{
    Cart, CartItem, Employee, EmployeeId, Invoice, InvoiceDraft, InvoiceId, Money, PricingPolicy,
    Product, ProductCategory, ProductId, QuoteDraft, QuoteId, QuoteLine, QuoteRecord, Session,
};
use core_ports::{
    BoxFuture, CartRepository, CatalogRepository, EmailError, EmailGateway, EmployeeRepository,
    InvoiceRepository, QuoteRepository, RepoError, SessionRepository,
};
use core_use_cases::{
    AddItemInput, AddItemToCartInteractor, ConfirmOrderInteractor, GetQuoteInteractor,
    GetQuoteDetailsInteractor, ListCatalogInteractor, ListQuotesInteractor, RegisterEmployeeInput,
    RegisterEmployeeInteractor, SignInInput, SignInInteractor, SignOutInput, SignOutInteractor,
    UpdateCartInput, UpdateCartInteractor, UseCaseError, ViewCartInteractor,
};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

struct FakeEmployeeRepo {
    employees: Mutex<Vec<Employee>>,
    next_id: Mutex<u64>,
}

impl FakeEmployeeRepo {
    fn new(seed: Vec<Employee>) -> Self {
        let next_id = seed
            .iter()
            .map(|employee| employee.id.0)
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            employees: Mutex::new(seed),
            next_id: Mutex::new(next_id),
        }
    }
}

impl EmployeeRepository for FakeEmployeeRepo {
    fn add_employee<'a>(
        &'a self,
        name: String,
        email: String,
        password: String,
    ) -> BoxFuture<'a, Result<Employee, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let employee = Employee {
                id: EmployeeId(*next_id),
                name,
                email,
                password,
            };
            *next_id += 1;
            self.employees.lock().unwrap().push(employee.clone());
            Ok(employee)
        })
    }

    fn find_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        Box::pin(async move {
            let employee = self
                .employees
                .lock()
                .unwrap()
                .iter()
                .find(|employee| employee.email == email)
                .cloned();
            Ok(employee)
        })
    }

    fn get_by_id<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        Box::pin(async move {
            let employee = self
                .employees
                .lock()
                .unwrap()
                .iter()
                .find(|employee| employee.id == employee_id)
                .cloned();
            Ok(employee)
        })
    }
}

struct FakeCatalogRepo {
    products: Vec<Product>,
}

impl FakeCatalogRepo {
    fn new(products: Vec<Product>) -> Self {
        Self { products }
    }
}

impl CatalogRepository for FakeCatalogRepo {
    fn list_products<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Product>, RepoError>> {
        let products = self.products.clone();
        Box::pin(async move { Ok(products) })
    }

    fn find_product<'a>(
        &'a self,
        product_id: ProductId,
    ) -> BoxFuture<'a, Result<Option<Product>, RepoError>> {
        let product = self
            .products
            .iter()
            .find(|product| product.id == product_id)
            .cloned();
        Box::pin(async move { Ok(product) })
    }
}

struct FakeCartRepo {
    carts: Mutex<HashMap<EmployeeId, Cart>>,
}

impl FakeCartRepo {
    fn new(seed: HashMap<EmployeeId, Cart>) -> Self {
        Self {
            carts: Mutex::new(seed),
        }
    }
}

impl CartRepository for FakeCartRepo {
    fn get_cart<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Cart>, RepoError>> {
        Box::pin(async move {
            let cart = self.carts.lock().unwrap().get(&employee_id).cloned();
            Ok(cart)
        })
    }

    fn save_cart<'a>(&'a self, cart: Cart) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.carts
                .lock()
                .unwrap()
                .insert(cart.employee_id.clone(), cart);
            Ok(())
        })
    }

    fn clear_cart<'a>(&'a self, employee_id: EmployeeId) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.carts.lock().unwrap().remove(&employee_id);
            Ok(())
        })
    }
}

struct FakeSessionRepo {
    sessions: Mutex<HashMap<String, Session>>,
    next_id: Mutex<u64>,
}

impl FakeSessionRepo {
    fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }
}

impl SessionRepository for FakeSessionRepo {
    fn create_session<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Session, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let token = format!("session-{}", *next_id);
            *next_id += 1;
            let session = Session {
                token: token.clone(),
                employee_id,
            };
            self.sessions
                .lock()
                .unwrap()
                .insert(token.clone(), session.clone());
            Ok(session)
        })
    }

    fn get_session<'a>(
        &'a self,
        token: &'a str,
    ) -> BoxFuture<'a, Result<Option<Session>, RepoError>> {
        let session = self.sessions.lock().unwrap().get(token).cloned();
        Box::pin(async move { Ok(session) })
    }

    fn remove_session<'a>(&'a self, token: &'a str) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.sessions.lock().unwrap().remove(token);
            Ok(())
        })
    }
}

struct FakeInvoiceRepo {
    invoices: Mutex<Vec<Invoice>>,
    next_id: Mutex<u64>,
}

impl FakeInvoiceRepo {
    fn new() -> Self {
        Self {
            invoices: Mutex::new(Vec::new()),
            next_id: Mutex::new(1),
        }
    }
}

impl InvoiceRepository for FakeInvoiceRepo {
    fn create_invoice<'a>(
        &'a self,
        draft: InvoiceDraft,
    ) -> BoxFuture<'a, Result<Invoice, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let invoice = Invoice {
                id: InvoiceId(*next_id),
                employee_id: draft.employee_id,
                items: draft.items,
                subtotal: draft.subtotal,
                fee: draft.fee,
                tax: draft.tax,
                total: draft.total,
            };
            *next_id += 1;
            self.invoices.lock().unwrap().push(invoice.clone());
            Ok(invoice)
        })
    }

    fn get_invoice<'a>(
        &'a self,
        invoice_id: InvoiceId,
    ) -> BoxFuture<'a, Result<Option<Invoice>, RepoError>> {
        let invoice = self
            .invoices
            .lock()
            .unwrap()
            .iter()
            .find(|invoice| invoice.id == invoice_id)
            .cloned();
        Box::pin(async move { Ok(invoice) })
    }
}

struct FakeQuoteRepo {
    quotes: Mutex<Vec<QuoteRecord>>,
    next_id: Mutex<u64>,
}

impl FakeQuoteRepo {
    fn new(seed: Vec<QuoteRecord>) -> Self {
        let next_id = seed.iter().map(|quote| quote.id.0).max().unwrap_or(0) + 1;
        Self {
            quotes: Mutex::new(seed),
            next_id: Mutex::new(next_id),
        }
    }

    fn all(&self) -> Vec<QuoteRecord> {
        self.quotes.lock().unwrap().clone()
    }
}

impl QuoteRepository for FakeQuoteRepo {
    fn create_quote<'a>(
        &'a self,
        draft: QuoteDraft,
    ) -> BoxFuture<'a, Result<QuoteRecord, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let record = QuoteRecord {
                id: QuoteId(*next_id),
                employee_id: draft.employee_id,
                items: draft.items,
                subtotal: draft.subtotal,
                fee: draft.fee,
                tax: draft.tax,
                total: draft.total,
                submitted_at: draft.submitted_at,
            };
            *next_id += 1;
            self.quotes.lock().unwrap().push(record.clone());
            Ok(record)
        })
    }

    fn list_quotes<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Vec<QuoteRecord>, RepoError>> {
        Box::pin(async move {
            let quotes = self
                .quotes
                .lock()
                .unwrap()
                .iter()
                .filter(|quote| quote.employee_id == employee_id)
                .cloned()
                .collect();
            Ok(quotes)
        })
    }

    fn get_quote<'a>(
        &'a self,
        employee_id: EmployeeId,
        quote_id: QuoteId,
    ) -> BoxFuture<'a, Result<Option<QuoteRecord>, RepoError>> {
        Box::pin(async move {
            let quote = self
                .quotes
                .lock()
                .unwrap()
                .iter()
                .find(|quote| quote.id == quote_id && quote.employee_id == employee_id)
                .cloned();
            Ok(quote)
        })
    }
}

#[derive(Clone, Debug)]
struct SentEmail {
    to: String,
    invoice_id: InvoiceId,
}

struct FakeEmailGateway {
    sent: Mutex<Vec<SentEmail>>,
}

impl FakeEmailGateway {
    fn new() -> Self {
        Self {
            sent: Mutex::new(Vec::new()),
        }
    }
}

impl EmailGateway for FakeEmailGateway {
    fn send_invoice<'a>(
        &'a self,
        to: &'a str,
        invoice: &'a Invoice,
    ) -> BoxFuture<'a, Result<(), EmailError>> {
        Box::pin(async move {
            self.sent.lock().unwrap().push(SentEmail {
                to: to.to_string(),
                invoice_id: invoice.id.clone(),
            });
            Ok(())
        })
    }
}


fn block_on<F: Future>(mut future: F) -> F::Output {
    fn raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut context = Context::from_waker(&waker);
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => {}
        }
    }
}

#[test]
fn register_employee_rejects_duplicate_email() {
    let existing = Employee {
        id: EmployeeId(1),
        name: "Ava Educator".to_string(),
        email: "ava@schools.nyc.gov".to_string(),
        password: "secret".to_string(),
    };
    let repo = FakeEmployeeRepo::new(vec![existing]);
    let interactor = RegisterEmployeeInteractor::new(&repo);

    let result = block_on(interactor.execute(RegisterEmployeeInput {
        name: "Ava Educator".to_string(),
        email: "ava@schools.nyc.gov".to_string(),
        password: "secret".to_string(),
    }));

    assert!(matches!(result, Err(UseCaseError::Validation(_))));
}

#[test]
fn register_employee_creates_employee() {
    let repo = FakeEmployeeRepo::new(Vec::new());
    let interactor = RegisterEmployeeInteractor::new(&repo);

    let output = block_on(interactor.execute(RegisterEmployeeInput {
        name: "Jamie Rivera".to_string(),
        email: "jamie@schools.nyc.gov".to_string(),
        password: "secret".to_string(),
    }))
    .expect("registration should succeed");
    assert_eq!(output.employee_id.0, 1);
    assert_eq!(output.email, "jamie@schools.nyc.gov");
}

#[test]
fn sign_in_creates_session() {
    let employee = Employee {
        id: EmployeeId(3),
        name: "Jordan Lee".to_string(),
        email: "jordan@schools.nyc.gov".to_string(),
        password: "passcode".to_string(),
    };
    let employees = FakeEmployeeRepo::new(vec![employee]);
    let sessions = FakeSessionRepo::new();
    let interactor = SignInInteractor::new(&employees, &sessions);

    let output = block_on(interactor.execute(SignInInput {
        email: "jordan@schools.nyc.gov".to_string(),
        password: "passcode".to_string(),
    }))
    .expect("sign in should succeed");

    assert_eq!(output.session_token, "session-1");
    assert_eq!(output.employee_id.0, 3);
}

#[test]
fn sign_out_clears_session() {
    let sessions = FakeSessionRepo::new();
    let session = block_on(sessions.create_session(EmployeeId(99))).unwrap();
    let interactor = SignOutInteractor::new(&sessions);

    let output = block_on(interactor.execute(SignOutInput {
        token: session.token.clone(),
    }))
    .expect("sign out should succeed");

    assert!(output.success);
    let remaining = block_on(sessions.get_session(&session.token)).unwrap();
    assert!(remaining.is_none());
}

#[test]
fn list_catalog_returns_products() {
    let products = vec![
        Product {
            id: ProductId(1),
            name: "Harvest Salad Box".to_string(),
            category: ProductCategory::food(),
            unit_price: Money::from_cents(1000).unwrap(),
        },
        Product {
            id: ProductId(2),
            name: "Reusable Lunch Tote".to_string(),
            category: ProductCategory::accessory(),
            unit_price: Money::from_cents(1800).unwrap(),
        },
    ];
    let catalog = FakeCatalogRepo::new(products);
    let interactor = ListCatalogInteractor::new(&catalog);

    let output = block_on(interactor.execute()).expect("catalog should succeed");

    assert_eq!(output.items.len(), 2);
    assert_eq!(output.items[0].name, "Harvest Salad Box");
    assert_eq!(output.items[1].category, ProductCategory::accessory());
}

#[test]
fn add_item_to_cart_returns_cart_output() {
    let product = Product {
        id: ProductId(11),
        name: "Harvest Salad Box".to_string(),
        category: ProductCategory::food(),
        unit_price: Money::from_cents(500).unwrap(),
    };
    let catalog = FakeCatalogRepo::new(vec![product.clone()]);
    let cart_repo = FakeCartRepo::new(HashMap::new());
    let interactor = AddItemToCartInteractor::new(&catalog, &cart_repo);

    let output = block_on(
        interactor.execute(AddItemInput {
            employee_id: EmployeeId(1),
            product_id: product.id,
            quantity: 2,
        }),
    )
    .expect("add to cart should succeed");

    assert_eq!(output.items.len(), 1);
    assert_eq!(output.items[0].quantity, 2);
    assert_eq!(output.subtotal.cents(), 1000);
}

#[test]
fn update_cart_returns_updated_output() {
    let product = Product {
        id: ProductId(21),
        name: "Brooklyn Bento Set".to_string(),
        category: ProductCategory::food(),
        unit_price: Money::from_cents(400).unwrap(),
    };
    let catalog = FakeCatalogRepo::new(vec![product.clone()]);
    let mut carts = HashMap::new();
    carts.insert(
        EmployeeId(2),
        Cart {
            employee_id: EmployeeId(2),
            items: vec![CartItem {
                product_id: product.id.clone(),
                quantity: 1,
            }],
        },
    );
    let cart_repo = FakeCartRepo::new(carts);
    let interactor = UpdateCartInteractor::new(&catalog, &cart_repo);

    let output = block_on(
        interactor.execute(UpdateCartInput {
            employee_id: EmployeeId(2),
            product_id: product.id,
            quantity: 3,
        }),
    )
    .expect("update cart should succeed");

    assert_eq!(output.items.len(), 1);
    assert_eq!(output.items[0].quantity, 3);
    assert_eq!(output.subtotal.cents(), 1200);
}

#[test]
fn view_cart_returns_output() {
    let product = Product {
        id: ProductId(31),
        name: "Teacher Travel Mug".to_string(),
        category: ProductCategory::accessory(),
        unit_price: Money::from_cents(900).unwrap(),
    };
    let catalog = FakeCatalogRepo::new(vec![product.clone()]);
    let mut carts = HashMap::new();
    carts.insert(
        EmployeeId(3),
        Cart {
            employee_id: EmployeeId(3),
            items: vec![CartItem {
                product_id: product.id.clone(),
                quantity: 2,
            }],
        },
    );
    let cart_repo = FakeCartRepo::new(carts);
    let interactor = ViewCartInteractor::new(&catalog, &cart_repo);

    let output = block_on(interactor.execute(EmployeeId(3))).expect("view cart should succeed");

    assert_eq!(output.items.len(), 1);
    assert_eq!(output.items[0].name, "Teacher Travel Mug");
    assert_eq!(output.subtotal.cents(), 1800);
}

#[test]
fn get_quote_details_returns_items_and_totals() {
    let quote = QuoteRecord {
        id: QuoteId(10),
        employee_id: EmployeeId(7),
        items: vec![
            QuoteLine {
                product_id: ProductId(101),
                name: "Harvest Salad Box".to_string(),
                category: ProductCategory::food(),
                unit_price: Money::from_cents(1000).unwrap(),
                quantity: 2,
                line_total: Money::from_cents(2000).unwrap(),
            },
            QuoteLine {
                product_id: ProductId(102),
                name: "Teacher Travel Mug".to_string(),
                category: ProductCategory::accessory(),
                unit_price: Money::from_cents(1500).unwrap(),
                quantity: 1,
                line_total: Money::from_cents(1500).unwrap(),
            },
        ],
        subtotal: Money::from_cents(3500).unwrap(),
        fee: Money::from_cents(495).unwrap(),
        tax: Money::from_cents(355).unwrap(),
        total: Money::from_cents(4350).unwrap(),
        submitted_at: 1_700_000_000,
    };
    let quote_repo = FakeQuoteRepo::new(vec![quote]);
    let interactor = GetQuoteDetailsInteractor::new(&quote_repo);

    let output = block_on(interactor.execute(EmployeeId(7), QuoteId(10)))
        .expect("quote details should succeed");

    assert_eq!(output.items.len(), 2);
    assert_eq!(output.subtotal.cents(), 3500);
    assert_eq!(output.fee.cents(), 495);
    assert_eq!(output.tax.cents(), 355);
    assert_eq!(output.total.cents(), 4350);
    assert_eq!(output.submitted_at, 1_700_000_000);
}

#[test]
fn quote_includes_fee_and_tax() {
    let pricing = PricingPolicy::default();
    let product = Product {
        id: ProductId(1),
        name: "Harvest Salad Box".to_string(),
        category: ProductCategory::food(),
        unit_price: Money::from_cents(1000).unwrap(),
    };
    let catalog = FakeCatalogRepo::new(vec![product.clone()]);
    let mut carts = HashMap::new();
    carts.insert(
        EmployeeId(1),
        Cart {
            employee_id: EmployeeId(1),
            items: vec![CartItem {
                product_id: product.id,
                quantity: 2,
            }],
        },
    );
    let cart_repo = FakeCartRepo::new(carts);
    let quote_repo = FakeQuoteRepo::new(Vec::new());
    let interactor = GetQuoteInteractor::new(&catalog, &cart_repo, &quote_repo, pricing);

    let output = block_on(interactor.execute(EmployeeId(1))).expect("quote should succeed");
    assert_eq!(output.subtotal.cents(), 2000);
    assert_eq!(output.fee.cents(), 495);
    assert_eq!(output.tax.cents(), 221);
    assert_eq!(output.total.cents(), 2716);

    let stored = quote_repo.all();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].total.cents(), 2716);
}

#[test]
fn list_quotes_returns_employee_history() {
    let quote_a = QuoteRecord {
        id: QuoteId(1),
        employee_id: EmployeeId(9),
        items: vec![
            QuoteLine {
                product_id: ProductId(11),
                name: "Harvest Salad Box".to_string(),
                category: ProductCategory::food(),
                unit_price: Money::from_cents(1000).unwrap(),
                quantity: 2,
                line_total: Money::from_cents(2000).unwrap(),
            },
            QuoteLine {
                product_id: ProductId(12),
                name: "Teacher Travel Mug".to_string(),
                category: ProductCategory::accessory(),
                unit_price: Money::from_cents(1500).unwrap(),
                quantity: 1,
                line_total: Money::from_cents(1500).unwrap(),
            },
        ],
        subtotal: Money::from_cents(3500).unwrap(),
        fee: Money::from_cents(495).unwrap(),
        tax: Money::from_cents(355).unwrap(),
        total: Money::from_cents(4350).unwrap(),
        submitted_at: 1_700_000_111,
    };
    let quote_b = QuoteRecord {
        id: QuoteId(2),
        employee_id: EmployeeId(9),
        items: vec![QuoteLine {
            product_id: ProductId(13),
            name: "Warm Grain Bowl".to_string(),
            category: ProductCategory::food(),
            unit_price: Money::from_cents(1200).unwrap(),
            quantity: 3,
            line_total: Money::from_cents(3600).unwrap(),
        }],
        subtotal: Money::from_cents(3600).unwrap(),
        fee: Money::from_cents(495).unwrap(),
        tax: Money::from_cents(363).unwrap(),
        total: Money::from_cents(4458).unwrap(),
        submitted_at: 1_700_000_222,
    };
    let other_quote = QuoteRecord {
        id: QuoteId(3),
        employee_id: EmployeeId(10),
        items: vec![QuoteLine {
            product_id: ProductId(14),
            name: "Reusable Lunch Tote".to_string(),
            category: ProductCategory::accessory(),
            unit_price: Money::from_cents(1800).unwrap(),
            quantity: 1,
            line_total: Money::from_cents(1800).unwrap(),
        }],
        subtotal: Money::from_cents(1800).unwrap(),
        fee: Money::from_cents(495).unwrap(),
        tax: Money::from_cents(203).unwrap(),
        total: Money::from_cents(2498).unwrap(),
        submitted_at: 1_700_000_333,
    };
    let quote_repo = FakeQuoteRepo::new(vec![quote_a.clone(), quote_b.clone(), other_quote]);
    let interactor = ListQuotesInteractor::new(&quote_repo);

    let output =
        block_on(interactor.execute(EmployeeId(9))).expect("list quotes should succeed");
    assert_eq!(output.quotes.len(), 2);
    assert_eq!(output.quotes[0].quote_id.0, 1);
    assert_eq!(output.quotes[0].total.cents(), 4350);
    assert_eq!(output.quotes[0].item_count, 3);
    assert_eq!(output.quotes[1].quote_id.0, 2);
    assert_eq!(output.quotes[1].total.cents(), 4458);
    assert_eq!(output.quotes[1].item_count, 3);
}

#[test]
fn confirm_order_sends_email_and_clears_cart() {
    let employee = Employee {
        id: EmployeeId(4),
        name: "Taylor Cruz".to_string(),
        email: "taylor@schools.nyc.gov".to_string(),
        password: "secret".to_string(),
    };
    let employees = FakeEmployeeRepo::new(vec![employee.clone()]);
    let product = Product {
        id: ProductId(2),
        name: "Snack Pack".to_string(),
        category: ProductCategory::food(),
        unit_price: Money::from_cents(750).unwrap(),
    };
    let catalog = FakeCatalogRepo::new(vec![product.clone()]);
    let mut carts = HashMap::new();
    carts.insert(
        employee.id.clone(),
        Cart {
            employee_id: employee.id.clone(),
            items: vec![CartItem {
                product_id: product.id,
                quantity: 1,
            }],
        },
    );
    let cart_repo = FakeCartRepo::new(carts);
    let invoices = FakeInvoiceRepo::new();
    let email = FakeEmailGateway::new();
    let interactor = ConfirmOrderInteractor::new(
        &catalog,
        &cart_repo,
        &invoices,
        &employees,
        &email,
        PricingPolicy::default(),
    );

    let output =
        block_on(interactor.execute(employee.id.clone())).expect("confirm should succeed");
    assert_eq!(output.invoice_id.0, 1);
    assert_eq!(output.email, "taylor@schools.nyc.gov");

    let sent = email.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].invoice_id.0, 1);
    assert_eq!(sent[0].to, "taylor@schools.nyc.gov");

    let cart = block_on(cart_repo.get_cart(employee.id)).unwrap();
    assert!(cart.is_none());
}
