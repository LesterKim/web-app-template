use core_entities::{EmployeeId, InvoiceId, Money, ProductCategory, ProductId, QuoteId};

#[derive(Clone, Debug)]
pub struct RegisterEmployeeOutput {
    pub employee_id: EmployeeId,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct SignInOutput {
    pub session_token: String,
    pub employee_id: EmployeeId,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct SignOutOutput {
    pub success: bool,
}

#[derive(Clone, Debug)]
pub struct CatalogItem {
    pub product_id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
}

#[derive(Clone, Debug)]
pub struct CatalogOutput {
    pub items: Vec<CatalogItem>,
}

#[derive(Clone, Debug)]
pub struct CartLine {
    pub product_id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
    pub quantity: u32,
    pub line_total: Money,
}

#[derive(Clone, Debug)]
pub struct CartOutput {
    pub items: Vec<CartLine>,
    pub subtotal: Money,
}

#[derive(Clone, Debug)]
pub struct QuoteOutput {
    pub items: Vec<CartLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Clone, Debug)]
pub struct QuoteSummary {
    pub quote_id: QuoteId,
    pub total: Money,
    pub item_count: u32,
}

#[derive(Clone, Debug)]
pub struct QuoteHistoryOutput {
    pub quotes: Vec<QuoteSummary>,
}

#[derive(Clone, Debug)]
pub struct ConfirmOrderOutput {
    pub invoice_id: InvoiceId,
    pub total: Money,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct QuoteDetailsOutput {
    pub quote_id: QuoteId,
    pub submitted_at: u64,
    pub items: Vec<QuoteLineOutput>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Clone, Debug)]
pub struct QuoteLineOutput {
    pub product_id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
    pub quantity: u32,
    pub line_total: Money,
}
