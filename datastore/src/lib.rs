pub mod postgres;

use core_entities::{
    Cart, Employee, EmployeeId, Invoice, InvoiceDraft, InvoiceId, Money, Product, ProductCategory,
    ProductId, QuoteDraft, QuoteId, QuoteRecord, Session,
};
use core_ports::{
    BoxFuture, CartRepository, CatalogRepository, EmailError, EmailGateway, EmployeeRepository,
    InvoiceRepository, QuoteRepository, RepoError, SessionRepository,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryEmployeeRepository {
    employees: Arc<Mutex<Vec<Employee>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryEmployeeRepository {
    pub fn new(seed: Vec<Employee>) -> Self {
        let next_id = seed
            .iter()
            .map(|employee| employee.id.0)
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            employees: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(next_id)),
        }
    }
}

impl EmployeeRepository for InMemoryEmployeeRepository {
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

pub struct InMemoryCatalogRepository {
    products: Arc<Vec<Product>>,
}

impl InMemoryCatalogRepository {
    pub fn new(products: Vec<Product>) -> Self {
        Self {
            products: Arc::new(products),
        }
    }

    pub fn seeded() -> Self {
        Self::new(vec![
            Product {
                id: ProductId(1),
                name: "Harvest Salad Box".to_string(),
                category: ProductCategory::Food,
                unit_price: Money::from_cents(1095).unwrap_or_else(|_| Money::zero()),
            },
            Product {
                id: ProductId(2),
                name: "Brooklyn Bento Set".to_string(),
                category: ProductCategory::Food,
                unit_price: Money::from_cents(1395).unwrap_or_else(|_| Money::zero()),
            },
            Product {
                id: ProductId(3),
                name: "Warm Grain Bowl".to_string(),
                category: ProductCategory::Food,
                unit_price: Money::from_cents(1295).unwrap_or_else(|_| Money::zero()),
            },
            Product {
                id: ProductId(4),
                name: "Reusable Lunch Tote".to_string(),
                category: ProductCategory::Accessory,
                unit_price: Money::from_cents(1895).unwrap_or_else(|_| Money::zero()),
            },
            Product {
                id: ProductId(5),
                name: "Stainless Snack Canister".to_string(),
                category: ProductCategory::Accessory,
                unit_price: Money::from_cents(995).unwrap_or_else(|_| Money::zero()),
            },
            Product {
                id: ProductId(6),
                name: "Teacher Travel Mug".to_string(),
                category: ProductCategory::Accessory,
                unit_price: Money::from_cents(1495).unwrap_or_else(|_| Money::zero()),
            },
        ])
    }
}

impl CatalogRepository for InMemoryCatalogRepository {
    fn list_products<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Product>, RepoError>> {
        let products = self.products.as_ref().clone();
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

pub struct InMemoryCartRepository {
    carts: Arc<Mutex<HashMap<EmployeeId, Cart>>>,
}

impl InMemoryCartRepository {
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CartRepository for InMemoryCartRepository {
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

pub struct InMemorySessionRepository {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl SessionRepository for InMemorySessionRepository {
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

pub struct InMemoryInvoiceRepository {
    invoices: Arc<Mutex<Vec<Invoice>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryInvoiceRepository {
    pub fn new() -> Self {
        Self {
            invoices: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl InvoiceRepository for InMemoryInvoiceRepository {
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

pub struct InMemoryQuoteRepository {
    quotes: Arc<Mutex<Vec<QuoteRecord>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryQuoteRepository {
    pub fn new() -> Self {
        Self {
            quotes: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl QuoteRepository for InMemoryQuoteRepository {
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
pub struct SentEmail {
    pub to: String,
    pub invoice_id: InvoiceId,
}

pub struct InMemoryEmailGateway {
    sent: Arc<Mutex<Vec<SentEmail>>>,
}

impl InMemoryEmailGateway {
    pub fn new() -> Self {
        Self {
            sent: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn sent_emails(&self) -> Vec<SentEmail> {
        self.sent.lock().unwrap().clone()
    }
}

impl EmailGateway for InMemoryEmailGateway {
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
