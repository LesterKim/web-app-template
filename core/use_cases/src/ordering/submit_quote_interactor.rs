use core_entities::ordering::{Invoice, InvoiceLineItem, Quote};
use core_ports::ordering::{
    CartRepository, Clock, EmailAttachment, EmailBody, EmailMessage, EmailOutbox, EmployeeRepository,
    InvoiceRenderer, InvoiceRepository, QuoteRepository, RateProvider, SchoolRepository,
    StructuredEmailBody,
};

use super::errors::QuoteError;
use super::quote_builder::build_quote;

pub struct SubmitQuoteInteractor<'a> {
    carts: &'a dyn CartRepository,
    employees: &'a dyn EmployeeRepository,
    schools: &'a dyn SchoolRepository,
    rates: &'a dyn RateProvider,
    clock: &'a dyn Clock,
    quotes: &'a dyn QuoteRepository,
    invoices: &'a dyn InvoiceRepository,
    email_outbox: &'a dyn EmailOutbox,
    invoice_renderer: &'a dyn InvoiceRenderer,
}

impl<'a> SubmitQuoteInteractor<'a> {
    pub fn new(
        carts: &'a dyn CartRepository,
        employees: &'a dyn EmployeeRepository,
        schools: &'a dyn SchoolRepository,
        rates: &'a dyn RateProvider,
        clock: &'a dyn Clock,
        quotes: &'a dyn QuoteRepository,
        invoices: &'a dyn InvoiceRepository,
        email_outbox: &'a dyn EmailOutbox,
        invoice_renderer: &'a dyn InvoiceRenderer,
    ) -> Self {
        Self {
            carts,
            employees,
            schools,
            rates,
            clock,
            quotes,
            invoices,
            email_outbox,
            invoice_renderer,
        }
    }

    pub fn execute(&self, input: SubmitQuoteInput) -> Result<SubmitQuoteOutput, QuoteError> {
        let employee = self
            .employees
            .find_by_email(&input.email)
            .map_err(|err| QuoteError::new(err.message))?
            .ok_or_else(|| QuoteError::new("employee account not found"))?;

        let school = self
            .schools
            .find_by_name(&employee.school_name)
            .map_err(|err| QuoteError::new(err.message))?
            .ok_or_else(|| QuoteError::new("school not found"))?;
        let school_code = school
            .code
            .ok_or_else(|| QuoteError::new("school code not found"))?;

        let date = self
            .clock
            .today()
            .map_err(|err| QuoteError::new(err.message))?;

        let cart_items = self
            .carts
            .items_for(&input.email)
            .map_err(|err| QuoteError::new(err.message))?;

        let tax_rate = self
            .rates
            .tax_rate()
            .map_err(|err| QuoteError::new(err.message))?;
        let shipping_rate = self
            .rates
            .shipping_rate()
            .map_err(|err| QuoteError::new(err.message))?;

        let quote = build_quote(
            &school_code,
            date,
            &employee.delivery_window,
            &cart_items,
            tax_rate,
            shipping_rate,
        );

        self.quotes
            .save(quote.clone())
            .map_err(|err| QuoteError::new(err.message))?;

        let invoice = build_invoice(&quote, &employee.school_name);
        self.invoices
            .save(invoice.clone())
            .map_err(|err| QuoteError::new(err.message))?;

        let pdf_bytes = self
            .invoice_renderer
            .render_pdf(&invoice)
            .map_err(|err| QuoteError::new(err.message))?;

        let email = EmailMessage {
            to: employee.email.clone(),
            body: build_email_body(&invoice),
            attachments: vec![EmailAttachment {
                content_type: "application/pdf".to_string(),
                file_name: Some("invoice.pdf".to_string()),
                bytes: pdf_bytes,
            }],
        };
        self.email_outbox
            .send(email)
            .map_err(|err| QuoteError::new(err.message))?;

        self.carts
            .clear(&input.email)
            .map_err(|err| QuoteError::new(err.message))?;

        Ok(SubmitQuoteOutput {
            quote_number: quote.number,
        })
    }
}

pub struct SubmitQuoteInput {
    pub email: String,
}

impl SubmitQuoteInput {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

pub struct SubmitQuoteOutput {
    pub quote_number: String,
}

fn build_invoice(quote: &Quote, school_name: &str) -> Invoice {
    let line_items = quote
        .line_items
        .iter()
        .map(|item| InvoiceLineItem {
            description: item.description.clone(),
            quantity: item.quantity,
            price: item.price,
        })
        .collect();

    Invoice {
        quote_number: quote.number.clone(),
        line_items,
        tax: Some(quote.tax),
        shipping: Some(quote.shipping),
        delivery_window: quote.delivery_window.clone(),
        school: school_name.to_string(),
    }
}

fn build_email_body(invoice: &Invoice) -> EmailBody {
    let (description, quantity, price) = invoice
        .line_items
        .first()
        .map(|item| (item.description.clone(), item.quantity, item.price))
        .unwrap_or_else(|| ("".to_string(), 0, 0.0));

    EmailBody::Structured(StructuredEmailBody {
        school: invoice.school.clone(),
        delivery_window: invoice.delivery_window.clone(),
        line_item_description: description,
        line_item_quantity: quantity,
        line_item_price: price,
    })
}
