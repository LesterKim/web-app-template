use core_entities::Money;

#[derive(Clone, Debug)]
pub struct LayoutViewModel {
    pub employee_name: Option<String>,
    pub cart_count: u32,
}

#[derive(Clone, Debug)]
pub struct CatalogProductViewModel {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub price: String,
}

#[derive(Clone, Debug)]
pub struct CatalogViewModel {
    pub products: Vec<CatalogProductViewModel>,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CartItemViewModel {
    pub product_id: u64,
    pub name: String,
    pub category: String,
    pub unit_price: String,
    pub quantity: u32,
    pub line_total: String,
}

#[derive(Clone, Debug)]
pub struct CartViewModel {
    pub items: Vec<CartItemViewModel>,
    pub subtotal: String,
    pub is_empty: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct QuoteViewModel {
    pub items: Vec<CartItemViewModel>,
    pub subtotal: String,
    pub fee: String,
    pub tax: String,
    pub total: String,
}

#[derive(Clone, Debug)]
pub struct InvoiceViewModel {
    pub invoice_id: u64,
    pub total: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct QuoteHistoryItemViewModel {
    pub quote_id: u64,
    pub total: String,
    pub item_count: u32,
}

#[derive(Clone, Debug)]
pub struct QuoteHistoryViewModel {
    pub quotes: Vec<QuoteHistoryItemViewModel>,
    pub is_empty: bool,
}

#[derive(Clone, Debug)]
pub struct QuoteDetailItemViewModel {
    pub name: String,
    pub category: String,
    pub unit_price: String,
    pub quantity: u32,
    pub line_total: String,
}

#[derive(Clone, Debug)]
pub struct QuoteDetailViewModel {
    pub quote_id: u64,
    pub submitted_at: String,
    pub items: Vec<QuoteDetailItemViewModel>,
    pub subtotal: String,
    pub fee: String,
    pub tax: String,
    pub total: String,
}

#[derive(Clone, Debug)]
pub struct SignInSessionViewModel {
    pub session_token: String,
}

#[derive(Clone, Debug)]
pub struct AuthViewModel {
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ErrorViewModel {
    pub title: String,
    pub message: String,
}

pub fn format_money(money: &Money) -> String {
    let cents = money.cents();
    let dollars = cents / 100;
    let remainder = cents % 100;
    format!("${}.{:02}", dollars, remainder)
}
