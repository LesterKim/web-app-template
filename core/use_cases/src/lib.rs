pub mod outputs;

use crate::outputs::{
    CartLine, CartOutput, CatalogItem, CatalogOutput, ConfirmOrderOutput, QuoteDetailsOutput,
    QuoteHistoryOutput, QuoteLineOutput, QuoteOutput, QuoteSummary, RegisterEmployeeOutput,
    SignInOutput, SignOutOutput,
};
use core_entities::{
    Cart, Employee, EmployeeId, InvoiceDraft, InvoiceLine, Money, PricingPolicy, Product,
    ProductId, QuoteDraft, QuoteId, QuoteLine,
};
use core_ports::{
    CartRepository, CatalogRepository, EmailGateway, EmailError, EmployeeRepository,
    InvoiceRepository, QuoteRepository, RepoError, SessionRepository,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum UseCaseError {
    Repo(RepoError),
    Email(EmailError),
    Validation(String),
    NotFound(String),
    Unauthorized,
    EmptyCart,
}

impl std::fmt::Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseCaseError::Repo(err) => write!(f, "repository error: {}", err.message),
            UseCaseError::Email(err) => write!(f, "email error: {}", err.message),
            UseCaseError::Validation(message) => write!(f, "validation error: {}", message),
            UseCaseError::NotFound(message) => write!(f, "not found: {}", message),
            UseCaseError::Unauthorized => write!(f, "unauthorized"),
            UseCaseError::EmptyCart => write!(f, "cart is empty"),
        }
    }
}

impl std::error::Error for UseCaseError {}

impl From<RepoError> for UseCaseError {
    fn from(err: RepoError) -> Self {
        UseCaseError::Repo(err)
    }
}

impl From<EmailError> for UseCaseError {
    fn from(err: EmailError) -> Self {
        UseCaseError::Email(err)
    }
}

pub struct RegisterEmployeeInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterEmployeeInteractor<'a> {
    repo: &'a dyn EmployeeRepository,
}

impl<'a> RegisterEmployeeInteractor<'a> {
    pub fn new(repo: &'a dyn EmployeeRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        input: RegisterEmployeeInput,
    ) -> Result<RegisterEmployeeOutput, UseCaseError> {
        let name = input.name.trim().to_string();
        if name.is_empty() {
            return Err(UseCaseError::Validation(
                "name cannot be empty".to_string(),
            ));
        }
        if !Employee::email_is_valid(&input.email) {
            return Err(UseCaseError::Validation(
                "email must look like an email address".to_string(),
            ));
        }
        if input.password.trim().is_empty() {
            return Err(UseCaseError::Validation(
                "password cannot be empty".to_string(),
            ));
        }

        if self.repo.find_by_email(&input.email).await?.is_some() {
            return Err(UseCaseError::Validation(
                "email already registered".to_string(),
            ));
        }

        let employee = self
            .repo
            .add_employee(name, input.email, input.password)
            .await?;

        let output = RegisterEmployeeOutput {
            employee_id: employee.id,
            name: employee.name,
            email: employee.email,
        };
        Ok(output)
    }
}

pub struct SignInInput {
    pub email: String,
    pub password: String,
}

pub struct SignInInteractor<'a> {
    employees: &'a dyn EmployeeRepository,
    sessions: &'a dyn SessionRepository,
}

impl<'a> SignInInteractor<'a> {
    pub fn new(
        employees: &'a dyn EmployeeRepository,
        sessions: &'a dyn SessionRepository,
    ) -> Self {
        Self { employees, sessions }
    }

    pub async fn execute(&self, input: SignInInput) -> Result<SignInOutput, UseCaseError> {
        let employee = self
            .employees
            .find_by_email(&input.email)
            .await?
            .ok_or(UseCaseError::Unauthorized)?;

        if employee.password != input.password {
            return Err(UseCaseError::Unauthorized);
        }

        let session = self.sessions.create_session(employee.id.clone()).await?;
        let output = SignInOutput {
            session_token: session.token,
            employee_id: employee.id,
            name: employee.name,
        };
        Ok(output)
    }
}

pub struct SignOutInput {
    pub token: String,
}

pub struct SignOutInteractor<'a> {
    sessions: &'a dyn SessionRepository,
}

impl<'a> SignOutInteractor<'a> {
    pub fn new(sessions: &'a dyn SessionRepository) -> Self {
        Self { sessions }
    }

    pub async fn execute(&self, input: SignOutInput) -> Result<SignOutOutput, UseCaseError> {
        self.sessions.remove_session(&input.token).await?;
        Ok(SignOutOutput { success: true })
    }
}

pub struct ListCatalogInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
}

impl<'a> ListCatalogInteractor<'a> {
    pub fn new(catalog: &'a dyn CatalogRepository) -> Self {
        Self { catalog }
    }

    pub async fn execute(&self) -> Result<CatalogOutput, UseCaseError> {
        let products = self.catalog.list_products().await?;
        let items = products
            .into_iter()
            .map(|product| CatalogItem {
                product_id: product.id,
                name: product.name,
                category: product.category,
                unit_price: product.unit_price,
            })
            .collect();
        Ok(CatalogOutput { items })
    }
}

pub struct AddItemInput {
    pub employee_id: EmployeeId,
    pub product_id: ProductId,
    pub quantity: u32,
}

pub struct AddItemToCartInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
    carts: &'a dyn CartRepository,
}

impl<'a> AddItemToCartInteractor<'a> {
    pub fn new(
        catalog: &'a dyn CatalogRepository,
        carts: &'a dyn CartRepository,
    ) -> Self {
        Self { catalog, carts }
    }

    pub async fn execute(&self, input: AddItemInput) -> Result<CartOutput, UseCaseError> {
        if input.quantity == 0 {
            return Err(UseCaseError::Validation(
                "quantity must be at least 1".to_string(),
            ));
        }

        let product = self
            .catalog
            .find_product(input.product_id.clone())
            .await?
            .ok_or_else(|| UseCaseError::NotFound("product not found".to_string()))?;

        let cart = self
            .carts
            .get_cart(input.employee_id.clone())
            .await?
            .unwrap_or_else(|| Cart::empty(input.employee_id.clone()));

        let mut updated = cart;
        updated
            .add_item(product.id.clone(), input.quantity)
            .map_err(|_| UseCaseError::Validation("quantity must be at least 1".to_string()))?;
        self.carts.save_cart(updated.clone()).await?;

        let (items, subtotal) = build_cart_lines(&updated, self.catalog).await?;
        Ok(CartOutput { items, subtotal })
    }
}

pub struct UpdateCartInput {
    pub employee_id: EmployeeId,
    pub product_id: ProductId,
    pub quantity: u32,
}

pub struct UpdateCartInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
    carts: &'a dyn CartRepository,
}

impl<'a> UpdateCartInteractor<'a> {
    pub fn new(
        catalog: &'a dyn CatalogRepository,
        carts: &'a dyn CartRepository,
    ) -> Self {
        Self { catalog, carts }
    }

    pub async fn execute(&self, input: UpdateCartInput) -> Result<CartOutput, UseCaseError> {
        let cart = self
            .carts
            .get_cart(input.employee_id.clone())
            .await?
            .unwrap_or_else(|| Cart::empty(input.employee_id.clone()));

        let mut updated = cart;
        updated
            .set_quantity(input.product_id, input.quantity)
            .map_err(|_| UseCaseError::Validation("invalid cart update".to_string()))?;
        self.carts.save_cart(updated.clone()).await?;

        let (items, subtotal) = build_cart_lines(&updated, self.catalog).await?;
        Ok(CartOutput { items, subtotal })
    }
}

pub struct ViewCartInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
    carts: &'a dyn CartRepository,
}

impl<'a> ViewCartInteractor<'a> {
    pub fn new(
        catalog: &'a dyn CatalogRepository,
        carts: &'a dyn CartRepository,
    ) -> Self {
        Self { catalog, carts }
    }

    pub async fn execute(&self, employee_id: EmployeeId) -> Result<CartOutput, UseCaseError> {
        let cart = self
            .carts
            .get_cart(employee_id.clone())
            .await?
            .unwrap_or_else(|| Cart::empty(employee_id));

        let (items, subtotal) = build_cart_lines(&cart, self.catalog).await?;
        Ok(CartOutput { items, subtotal })
    }
}

pub struct GetQuoteInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
    carts: &'a dyn CartRepository,
    quotes: &'a dyn QuoteRepository,
    pricing: PricingPolicy,
}

impl<'a> GetQuoteInteractor<'a> {
    pub fn new(
        catalog: &'a dyn CatalogRepository,
        carts: &'a dyn CartRepository,
        quotes: &'a dyn QuoteRepository,
        pricing: PricingPolicy,
    ) -> Self {
        Self {
            catalog,
            carts,
            quotes,
            pricing,
        }
    }

    pub async fn execute(&self, employee_id: EmployeeId) -> Result<QuoteOutput, UseCaseError> {
        let cart = self
            .carts
            .get_cart(employee_id.clone())
            .await?
            .unwrap_or_else(|| Cart::empty(employee_id));

        if cart.is_empty() {
            return Err(UseCaseError::EmptyCart);
        }

        let (items, subtotal) = build_cart_lines(&cart, self.catalog).await?;
        let fee = self.pricing.flat_fee.clone();
        let taxable = subtotal.add(fee.clone());
        let tax = self.pricing.calculate_tax(taxable.clone());
        let total = taxable.add(tax.clone());

        let quote_items: Vec<QuoteLine> = items
            .iter()
            .map(|item| QuoteLine {
                product_id: item.product_id.clone(),
                name: item.name.clone(),
                category: item.category.clone(),
                unit_price: item.unit_price.clone(),
                quantity: item.quantity,
                line_total: item.line_total.clone(),
            })
            .collect();

        let draft = QuoteDraft {
            employee_id: cart.employee_id.clone(),
            items: quote_items,
            subtotal: subtotal.clone(),
            fee: fee.clone(),
            tax: tax.clone(),
            total: total.clone(),
            submitted_at: current_timestamp_seconds(),
        };

        let _ = self.quotes.create_quote(draft).await?;

        Ok(QuoteOutput {
            items,
            subtotal,
            fee,
            tax,
            total,
        })
    }
}

pub struct ListQuotesInteractor<'a> {
    quotes: &'a dyn QuoteRepository,
}

impl<'a> ListQuotesInteractor<'a> {
    pub fn new(quotes: &'a dyn QuoteRepository) -> Self {
        Self { quotes }
    }

    pub async fn execute(
        &self,
        employee_id: EmployeeId,
    ) -> Result<QuoteHistoryOutput, UseCaseError> {
        let quotes = self.quotes.list_quotes(employee_id).await?;
        let summaries = quotes
            .into_iter()
            .map(|quote| {
                let item_count = quote.items.iter().map(|item| item.quantity).sum();
                QuoteSummary {
                    quote_id: quote.id,
                    total: quote.total,
                    item_count,
                }
            })
            .collect();

        Ok(QuoteHistoryOutput { quotes: summaries })
    }
}

pub struct ConfirmOrderInteractor<'a> {
    catalog: &'a dyn CatalogRepository,
    carts: &'a dyn CartRepository,
    invoices: &'a dyn InvoiceRepository,
    employees: &'a dyn EmployeeRepository,
    email: &'a dyn EmailGateway,
    pricing: PricingPolicy,
}

pub struct GetQuoteDetailsInteractor<'a> {
    quotes: &'a dyn QuoteRepository,
}

impl<'a> GetQuoteDetailsInteractor<'a> {
    pub fn new(quotes: &'a dyn QuoteRepository) -> Self {
        Self { quotes }
    }

    pub async fn execute(
        &self,
        employee_id: EmployeeId,
        quote_id: QuoteId,
    ) -> Result<QuoteDetailsOutput, UseCaseError> {
        let record = self
            .quotes
            .get_quote(employee_id, quote_id)
            .await?
            .ok_or_else(|| UseCaseError::NotFound("quote not found".to_string()))?;

        let items = record
            .items
            .into_iter()
            .map(|item| QuoteLineOutput {
                product_id: item.product_id,
                name: item.name,
                category: item.category,
                unit_price: item.unit_price,
                quantity: item.quantity,
                line_total: item.line_total,
            })
            .collect();

        Ok(QuoteDetailsOutput {
            quote_id: record.id,
            submitted_at: record.submitted_at,
            items,
            subtotal: record.subtotal,
            fee: record.fee,
            tax: record.tax,
            total: record.total,
        })
    }
}

fn current_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

impl<'a> ConfirmOrderInteractor<'a> {
    pub fn new(
        catalog: &'a dyn CatalogRepository,
        carts: &'a dyn CartRepository,
        invoices: &'a dyn InvoiceRepository,
        employees: &'a dyn EmployeeRepository,
        email: &'a dyn EmailGateway,
        pricing: PricingPolicy,
    ) -> Self {
        Self {
            catalog,
            carts,
            invoices,
            employees,
            email,
            pricing,
        }
    }

    pub async fn execute(
        &self,
        employee_id: EmployeeId,
    ) -> Result<ConfirmOrderOutput, UseCaseError> {
        let employee = self
            .employees
            .get_by_id(employee_id.clone())
            .await?
            .ok_or_else(|| UseCaseError::NotFound("employee not found".to_string()))?;

        let cart = self
            .carts
            .get_cart(employee_id.clone())
            .await?
            .unwrap_or_else(|| Cart::empty(employee_id));

        if cart.is_empty() {
            return Err(UseCaseError::EmptyCart);
        }

        let (items, subtotal) = build_cart_lines(&cart, self.catalog).await?;
        let fee = self.pricing.flat_fee.clone();
        let taxable = subtotal.add(fee.clone());
        let tax = self.pricing.calculate_tax(taxable.clone());
        let total = taxable.add(tax.clone());

        let invoice_items: Vec<InvoiceLine> = items
            .iter()
            .map(|item| InvoiceLine {
                product_id: item.product_id.clone(),
                name: item.name.clone(),
                category: item.category.clone(),
                unit_price: item.unit_price.clone(),
                quantity: item.quantity,
                line_total: item.line_total.clone(),
            })
            .collect();

        let draft = InvoiceDraft {
            employee_id: employee.id.clone(),
            items: invoice_items,
            subtotal: subtotal.clone(),
            fee: fee.clone(),
            tax: tax.clone(),
            total: total.clone(),
        };

        let invoice = self.invoices.create_invoice(draft).await?;
        self.email
            .send_invoice(employee.email.as_str(), &invoice)
            .await?;
        self.carts.clear_cart(employee.id.clone()).await?;

        Ok(ConfirmOrderOutput {
            invoice_id: invoice.id,
            total,
            email: employee.email,
        })
    }
}

async fn build_cart_lines(
    cart: &Cart,
    catalog: &dyn CatalogRepository,
) -> Result<(Vec<CartLine>, Money), UseCaseError> {
    let mut items = Vec::new();
    let mut subtotal = Money::zero();

    for item in &cart.items {
        let product = catalog
            .find_product(item.product_id.clone())
            .await?
            .ok_or_else(|| UseCaseError::NotFound("product not found".to_string()))?;
        let line_total = product.unit_price.multiply(item.quantity);
        subtotal = subtotal.add(line_total.clone());
        items.push(build_cart_line(product, item.quantity, line_total));
    }

    Ok((items, subtotal))
}

fn build_cart_line(product: Product, quantity: u32, line_total: Money) -> CartLine {
    CartLine {
        product_id: product.id,
        name: product.name,
        category: product.category,
        unit_price: product.unit_price,
        quantity,
        line_total,
    }
}
