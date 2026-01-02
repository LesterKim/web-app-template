use core_entities::ordering::Quote;
use core_ports::ordering::{
    CartRepository, Clock, EmployeeRepository, QuoteRepository, RateProvider, SchoolRepository,
};

use super::errors::QuoteError;
use super::quote_builder::build_quote;

pub struct ViewQuoteInteractor<'a> {
    carts: &'a dyn CartRepository,
    employees: &'a dyn EmployeeRepository,
    schools: &'a dyn SchoolRepository,
    rates: &'a dyn RateProvider,
    clock: &'a dyn Clock,
    quotes: &'a dyn QuoteRepository,
}

impl<'a> ViewQuoteInteractor<'a> {
    pub fn new(
        carts: &'a dyn CartRepository,
        employees: &'a dyn EmployeeRepository,
        schools: &'a dyn SchoolRepository,
        rates: &'a dyn RateProvider,
        clock: &'a dyn Clock,
        quotes: &'a dyn QuoteRepository,
    ) -> Self {
        Self {
            carts,
            employees,
            schools,
            rates,
            clock,
            quotes,
        }
    }

    pub fn execute(&self, input: ViewQuoteInput) -> Result<Quote, QuoteError> {
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

        Ok(quote)
    }
}

pub struct ViewQuoteInput {
    pub email: String,
}

impl ViewQuoteInput {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}
