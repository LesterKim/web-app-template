#[derive(Clone, Debug)]
pub struct CartLineExpectation {
    pub description: String,
    pub quantity: u32,
}

impl CartLineExpectation {
    pub fn new(description: &str, quantity: u32) -> Self {
        Self {
            description: description.to_string(),
            quantity,
        }
    }
}

pub struct QuoteLineExpectation {
    pub description: String,
    pub quantity: u32,
    pub price: f64,
    pub line_total: f64,
}

impl QuoteLineExpectation {
    pub fn new(description: &str, quantity: u32, price: f64, line_total: f64) -> Self {
        Self {
            description: description.to_string(),
            quantity,
            price,
            line_total,
        }
    }
}

pub struct InvoiceEmailExpectation {
    pub school: String,
    pub delivery_window: String,
    pub line_item_description: String,
    pub line_item_quantity: u32,
    pub line_item_price: f64,
}

impl InvoiceEmailExpectation {
    pub fn new(
        school: &str,
        delivery_window: &str,
        line_item_description: &str,
        line_item_quantity: u32,
        line_item_price: f64,
    ) -> Self {
        Self {
            school: school.to_string(),
            delivery_window: delivery_window.to_string(),
            line_item_description: line_item_description.to_string(),
            line_item_quantity,
            line_item_price,
        }
    }
}
