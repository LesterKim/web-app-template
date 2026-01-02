use core_entities::ordering::{CalendarDate, CartLineItem, Quote, QuoteLineItem};

pub fn build_quote(
    school_code: &str,
    date: CalendarDate,
    delivery_window: &str,
    cart_items: &[CartLineItem],
    tax_rate: f64,
    shipping_rate: f64,
) -> Quote {
    let line_items: Vec<QuoteLineItem> = cart_items
        .iter()
        .map(|item| QuoteLineItem {
            description: item.description.clone(),
            quantity: item.quantity,
            price: item.price,
            line_total: item.price * item.quantity as f64,
        })
        .collect();

    let subtotal: f64 = line_items.iter().map(|item| item.line_total).sum();
    let tax = subtotal * tax_rate;
    let shipping = subtotal * shipping_rate;
    let total = subtotal + tax + shipping;

    Quote {
        number: format!(
            "{school_code}x{:02}{:02}{:02}",
            date.month, date.day, date.year_two_digits
        ),
        line_items,
        subtotal,
        tax,
        shipping,
        total,
        delivery_window: delivery_window.to_string(),
    }
}
