use crate::view_models::{
    format_money, CartItemViewModel, CartViewModel, CatalogProductViewModel, CatalogViewModel,
    InvoiceViewModel, QuoteDetailItemViewModel, QuoteDetailViewModel, QuoteHistoryItemViewModel,
    QuoteHistoryViewModel, QuoteViewModel, SignInSessionViewModel,
};
use core_use_cases::outputs::{
    CartOutput, CatalogOutput, ConfirmOrderOutput, QuoteDetailsOutput, QuoteHistoryOutput,
    QuoteOutput, SignInOutput,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub struct CatalogPresenter;

impl CatalogPresenter {
    pub fn present(output: CatalogOutput) -> CatalogViewModel {
        let products = output
            .items
            .into_iter()
            .map(|item| CatalogProductViewModel {
                id: item.product_id.0,
                name: item.name,
                category: item.category.label().to_string(),
                price: format_money(&item.unit_price),
            })
            .collect();

        CatalogViewModel {
            products,
            error: None,
        }
    }
}

pub struct CartPresenter;

impl CartPresenter {
    pub fn present(output: CartOutput) -> CartViewModel {
        let items: Vec<CartItemViewModel> = output
            .items
            .into_iter()
            .map(|item| CartItemViewModel {
                product_id: item.product_id.0,
                name: item.name,
                category: item.category.label().to_string(),
                unit_price: format_money(&item.unit_price),
                quantity: item.quantity,
                line_total: format_money(&item.line_total),
            })
            .collect();

        let is_empty = items.is_empty();
        CartViewModel {
            items,
            subtotal: format_money(&output.subtotal),
            is_empty,
            error: None,
        }
    }
}

pub struct QuotePresenter;

impl QuotePresenter {
    pub fn present(output: QuoteOutput) -> QuoteViewModel {
        let items = output
            .items
            .into_iter()
            .map(|item| CartItemViewModel {
                product_id: item.product_id.0,
                name: item.name,
                category: item.category.label().to_string(),
                unit_price: format_money(&item.unit_price),
                quantity: item.quantity,
                line_total: format_money(&item.line_total),
            })
            .collect();

        QuoteViewModel {
            items,
            subtotal: format_money(&output.subtotal),
            fee: format_money(&output.fee),
            tax: format_money(&output.tax),
            total: format_money(&output.total),
        }
    }
}

pub struct QuoteHistoryPresenter;

impl QuoteHistoryPresenter {
    pub fn present(output: QuoteHistoryOutput) -> QuoteHistoryViewModel {
        let quotes: Vec<QuoteHistoryItemViewModel> = output
            .quotes
            .into_iter()
            .map(|quote| QuoteHistoryItemViewModel {
                quote_id: quote.quote_id.0,
                total: format_money(&quote.total),
                item_count: quote.item_count,
            })
            .collect();

        QuoteHistoryViewModel {
            is_empty: quotes.is_empty(),
            quotes,
        }
    }
}

pub struct QuoteDetailPresenter;

impl QuoteDetailPresenter {
    pub fn present(output: QuoteDetailsOutput) -> QuoteDetailViewModel {
        let items = output
            .items
            .into_iter()
            .map(|item| QuoteDetailItemViewModel {
                name: item.name,
                category: item.category.label().to_string(),
                unit_price: format_money(&item.unit_price),
                quantity: item.quantity,
                line_total: format_money(&item.line_total),
            })
            .collect();

        QuoteDetailViewModel {
            quote_id: output.quote_id.0,
            submitted_at: format_timestamp(output.submitted_at),
            items,
            subtotal: format_money(&output.subtotal),
            fee: format_money(&output.fee),
            tax: format_money(&output.tax),
            total: format_money(&output.total),
        }
    }
}

pub struct ConfirmPresenter;

impl ConfirmPresenter {
    pub fn present(output: ConfirmOrderOutput) -> InvoiceViewModel {
        InvoiceViewModel {
            invoice_id: output.invoice_id.0,
            total: format_money(&output.total),
            email: output.email,
        }
    }
}

pub struct SignInPresenter;

impl SignInPresenter {
    pub fn present(output: SignInOutput) -> SignInSessionViewModel {
        SignInSessionViewModel {
            session_token: output.session_token,
        }
    }
}

fn format_timestamp(timestamp: u64) -> String {
    let timestamp = i64::try_from(timestamp).unwrap_or(0);
    if let Ok(datetime) = OffsetDateTime::from_unix_timestamp(timestamp) {
        if let Ok(formatted) = datetime.format(&Rfc3339) {
            return formatted;
        }
    }
    format!("{} UTC", timestamp)
}
