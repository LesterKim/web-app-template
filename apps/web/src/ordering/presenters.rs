use core_entities::ordering::{CartLineItem, Invoice, Quote};
use core_use_cases::ordering::{SignInEmployeeOutput, SignUpEmployeeOutput, SubmitQuoteOutput};

use super::view_models::{
    CartItemViewModel, CatalogItemViewModel, InvoiceLineItemViewModel, InvoiceViewModel,
    QuoteLineItemViewModel, QuotePreviewViewModel, QuoteViewModel, SessionViewModel,
    SignUpSuccessViewModel,
};
use super::CatalogItem;

pub struct OrderingPresenter;

impl OrderingPresenter {
    pub fn sign_up_success(output: &SignUpEmployeeOutput) -> SignUpSuccessViewModel {
        SignUpSuccessViewModel {
            message: format!("Account created for {}. Please sign in.", output.employee_email),
        }
    }

    pub fn sign_in_session(output: &SignInEmployeeOutput) -> SessionViewModel {
        SessionViewModel {
            email: output.email.clone(),
        }
    }

    pub fn submit_quote_confirmation(output: &SubmitQuoteOutput) -> String {
        format!(
            "Quote {} submitted. An invoice email is on its way.",
            output.quote_number
        )
    }

    pub fn catalog_items(items: &[CatalogItem]) -> Vec<CatalogItemViewModel> {
        items
            .iter()
            .map(|item| CatalogItemViewModel {
                description: item.description.clone(),
                price: format_money(item.price),
            })
            .collect()
    }

    pub fn cart_items(items: &[CartLineItem]) -> Vec<CartItemViewModel> {
        items
            .iter()
            .map(|item| CartItemViewModel {
                description: item.description.clone(),
                quantity: item.quantity,
                price: format_money(item.price),
                line_total: format_money(item.price * item.quantity as f64),
            })
            .collect()
    }

    pub fn quote_preview(result: Result<Quote, String>) -> QuotePreviewViewModel {
        match result {
            Ok(quote) => QuotePreviewViewModel {
                quote: Some(Self::quote(quote)),
                quote_error: None,
            },
            Err(message) => QuotePreviewViewModel {
                quote: None,
                quote_error: Some(message),
            },
        }
    }

    pub fn quote(quote: Quote) -> QuoteViewModel {
        let line_items = quote
            .line_items
            .into_iter()
            .map(|item| QuoteLineItemViewModel {
                description: item.description,
                quantity: item.quantity,
                price: format_money(item.price),
                line_total: format_money(item.line_total),
            })
            .collect();

        QuoteViewModel {
            number: quote.number,
            line_items,
            subtotal: format_money(quote.subtotal),
            tax: format_money(quote.tax),
            shipping: format_money(quote.shipping),
            total: format_money(quote.total),
            delivery_window: quote.delivery_window,
        }
    }

    pub fn invoice(invoice: Invoice, recipient: &str) -> InvoiceViewModel {
        let subtotal: f64 = invoice
            .line_items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();
        let tax = invoice.tax.unwrap_or(0.0);
        let shipping = invoice.shipping.unwrap_or(0.0);
        let total = subtotal + tax + shipping;

        let line_items = invoice
            .line_items
            .into_iter()
            .map(|item| InvoiceLineItemViewModel {
                description: item.description,
                quantity: item.quantity,
                price: format_money(item.price),
                line_total: format_money(item.price * item.quantity as f64),
            })
            .collect();

        InvoiceViewModel {
            quote_number: invoice.quote_number,
            recipient: recipient.to_string(),
            school: invoice.school,
            delivery_window: invoice.delivery_window,
            line_items,
            subtotal: format_money(subtotal),
            tax: format_money(tax),
            shipping: format_money(shipping),
            total: format_money(total),
        }
    }
}

fn format_money(value: f64) -> String {
    format!("${:.2}", value)
}
