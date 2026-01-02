use core_entities::ordering::{CalendarDate, CartLineItem, Employee, Quote};
use core_use_cases::ordering::{
    AddItemToCartInput, AuthError, CartError, QuoteError, SignInEmployeeInput,
    SignInEmployeeOutput, SignOutEmployeeInput, SignUpEmployeeInput, SignUpEmployeeOutput,
    SignUpError, SubmitQuoteInput, SubmitQuoteOutput, ViewQuoteInput,
};

use super::drivers::{ProtocolDrivers, UseCaseDriver};
use super::types::{CartLineExpectation, InvoiceEmailExpectation, QuoteLineExpectation};

const DEFAULT_SCHOOL_NAME: &str = "P.S. 082 - The Hammond School";
const DEFAULT_DELIVERY_WINDOW: &str = "School Hours";

pub struct ScenarioLog {
    name: &'static str,
    purpose: &'static str,
}

impl ScenarioLog {
    pub fn start(name: &'static str, purpose: &'static str) -> Self {
        println!("SCENARIO: {name} - {purpose}");
        Self { name, purpose }
    }
}

impl Drop for ScenarioLog {
    fn drop(&mut self) {
        if std::thread::panicking() {
            println!("FAIL: {} - {}", self.name, self.purpose);
        } else {
            println!("PASS: {} - {}", self.name, self.purpose);
        }
    }
}

pub struct TestDsl {
    pub drivers: ProtocolDrivers,
    protocol: UseCaseDriver,
    state: TestState,
}

impl TestDsl {
    pub fn new() -> Self {
        Self {
            drivers: ProtocolDrivers::new(),
            protocol: UseCaseDriver::new(),
            state: TestState::default(),
        }
    }

    pub fn given_school(&mut self, name: &str, code: &str) {
        self.drivers.schools.upsert(name, Some(code));
    }

    pub fn given_new_employee(&mut self, fields: SignUpEmployeeInput) {
        self.state.pending_employee = Some(fields);
    }

    pub fn when_sign_up(&mut self) {
        let fields = self
            .state
            .pending_employee
            .take()
            .expect("new employee must be provided before sign up");
        self.state.sign_up_result = Some(self.protocol.sign_up(&self.drivers, fields));
    }

    pub fn then_account_exists(&self, email: &str) {
        assert!(
            self.drivers.users.exists(email),
            "expected account to exist for {email}"
        );
    }

    pub fn then_account_associated_with_school(&self, email: &str, school_name: &str) {
        let account = self
            .drivers
            .users
            .get(email)
            .expect("expected employee account to exist");
        assert_eq!(
            account.school_name.as_str(),
            school_name,
            "account should be associated with the expected school"
        );
    }

    pub fn then_sign_up_rejected(&self) {
        let result = self
            .state
            .sign_up_result
            .as_ref()
            .expect("sign up must be attempted before checking rejection");
        assert!(result.is_err(), "expected sign up to be rejected");
    }

    pub fn then_account_does_not_exist(&self, email: &str) {
        assert!(
            !self.drivers.users.exists(email),
            "expected no account to exist for {email}"
        );
    }

    pub fn given_employee_account(&mut self, email: &str, password: &str) {
        let account = Employee::new(
            email,
            password,
            "Quanisha",
            "Williams",
            "Coordinator",
            DEFAULT_SCHOOL_NAME,
            "(718) 526-4139 Ext. 2131",
            DEFAULT_DELIVERY_WINDOW,
        );
        self.drivers.users.insert(account);
        self.drivers.schools.ensure(DEFAULT_SCHOOL_NAME);
        self.state.current_employee_email = Some(email.to_string());
    }

    pub fn when_sign_in(&mut self, email: &str, password: &str) {
        self.state.current_employee_email = Some(email.to_string());
        let request = SignInEmployeeInput::new(email, password);
        self.state.sign_in_result = Some(self.protocol.sign_in(&self.drivers, request));
    }

    pub fn then_session_active(&self, email: &str) {
        assert!(
            self.drivers.sessions.is_active(email),
            "expected active session for {email}"
        );
    }

    pub fn given_authenticated_session(&mut self, email: &str) {
        self.drivers.sessions.start(email);
        self.state.current_employee_email = Some(email.to_string());
    }

    pub fn when_sign_out(&mut self) {
        let email = self.require_current_email().to_string();
        let request = SignOutEmployeeInput::new(&email);
        self.state.sign_out_result = Some(self.protocol.sign_out(&self.drivers, request));
    }

    pub fn then_session_terminated(&self, email: &str) {
        assert!(
            !self.drivers.sessions.is_active(email),
            "expected session to be terminated for {email}"
        );
    }

    pub fn given_signed_in_employee(&mut self, email: &str) {
        let account = Employee::fixture(email, DEFAULT_SCHOOL_NAME, DEFAULT_DELIVERY_WINDOW);
        self.drivers.users.insert(account);
        self.drivers.schools.ensure(DEFAULT_SCHOOL_NAME);
        self.drivers.sessions.start(email);
        self.state.current_employee_email = Some(email.to_string());
    }

    pub fn given_catalog_item(&mut self, description: &str, price: f64) {
        self.drivers.catalog.upsert(description, price);
    }

    pub fn when_add_to_cart(&mut self, description: &str, quantity: u32) {
        let email = self.require_current_email().to_string();
        let request = AddItemToCartInput::new(&email, description, quantity);
        self.state.add_to_cart_result =
            Some(self.protocol.add_item_to_cart(&self.drivers, request));
    }

    pub fn then_cart_contains(&self, expected: Vec<CartLineExpectation>) {
        let email = self.require_current_email();
        let items = self.drivers.carts.items_for(email);
        assert_eq!(
            items.len(),
            expected.len(),
            "expected cart items count to match"
        );
        for expected_item in expected {
            let actual = items
                .iter()
                .find(|item| item.description.as_str() == expected_item.description.as_str())
                .expect("expected cart line item to exist");
            assert_eq!(
                actual.quantity, expected_item.quantity,
                "expected cart quantity to match for {}",
                expected_item.description
            );
        }
    }

    pub fn given_cart_contains(&mut self, items: Vec<CartLineItem>) {
        let email = self.require_current_email().to_string();
        self.drivers.carts.replace_cart(&email, items);
    }

    pub fn given_school_code(&mut self, code: &str) {
        let email = self.require_current_email();
        let school_name = self
            .drivers
            .users
            .school_name_for(email)
            .expect("employee account must exist to set school code");
        self.drivers.schools.upsert(&school_name, Some(code));
    }

    pub fn given_today_is(&mut self, mmddyy: &str) {
        self.drivers
            .clock
            .set_today(CalendarDate::parse_mmddyy(mmddyy));
    }

    pub fn given_tax_rate(&mut self, rate: f64) {
        self.drivers.rates.set_tax_rate(rate);
    }

    pub fn given_shipping_rate(&mut self, rate: f64) {
        self.drivers.rates.set_shipping_rate(rate);
    }

    pub fn when_view_quote(&mut self) {
        let email = self.require_current_email().to_string();
        let request = ViewQuoteInput::new(&email);
        self.state.quote_result = Some(self.protocol.view_quote(&self.drivers, request));
    }

    pub fn then_quote_number_is(&self, expected: &str) {
        let quote = self.require_quote();
        assert_eq!(quote.number.as_str(), expected, "quote number should match");
    }

    pub fn then_quote_includes(&self, expected: Vec<QuoteLineExpectation>) {
        let quote = self.require_quote();
        assert_eq!(
            quote.line_items.len(),
            expected.len(),
            "expected quote line item count to match"
        );
        for expected_item in expected {
            let actual = quote
                .line_items
                .iter()
                .find(|item| item.description.as_str() == expected_item.description.as_str())
                .expect("expected quote line item to exist");
            assert_eq!(
                actual.quantity, expected_item.quantity,
                "expected quote quantity to match for {}",
                expected_item.description
            );
            assert_f64_close(actual.price, expected_item.price, "quote line item price");
            assert_f64_close(
                actual.line_total,
                expected_item.line_total,
                "quote line item total",
            );
        }
    }

    pub fn then_quote_subtotal_is(&self, expected: f64) {
        let quote = self.require_quote();
        assert_f64_close(quote.subtotal, expected, "quote subtotal");
    }

    pub fn then_quote_tax_is_subtotal_times_rate(&self) {
        let quote = self.require_quote();
        let rate = self
            .drivers
            .rates
            .current_tax_rate()
            .expect("tax rate must be set");
        let expected = quote.subtotal * rate;
        assert_f64_close(quote.tax, expected, "quote tax");
    }

    pub fn then_quote_shipping_is_subtotal_times_rate(&self) {
        let quote = self.require_quote();
        let rate = self
            .drivers
            .rates
            .current_shipping_rate()
            .expect("shipping rate must be set");
        let expected = quote.subtotal * rate;
        assert_f64_close(quote.shipping, expected, "quote shipping");
    }

    pub fn then_quote_total_is_subtotal_plus_tax_plus_shipping(&self) {
        let quote = self.require_quote();
        let expected = quote.subtotal + quote.tax + quote.shipping;
        assert_f64_close(quote.total, expected, "quote total");
    }

    pub fn then_delivery_window_is(&self, expected: &str) {
        let quote = self.require_quote();
        assert_eq!(
            quote.delivery_window.as_str(),
            expected,
            "delivery window should match"
        );
    }

    pub fn when_submit_quote(&mut self) {
        let email = self.require_current_email().to_string();
        let request = SubmitQuoteInput::new(&email);
        self.state.submit_result = Some(self.protocol.submit_quote(&self.drivers, request));
    }

    pub fn then_invoice_created_with_quote_number(&self, expected: &str) {
        let invoice = self
            .drivers
            .invoices
            .last()
            .expect("expected invoice to be created");
        assert_eq!(
            invoice.quote_number.as_str(),
            expected,
            "invoice quote number should match"
        );
    }

    pub fn then_invoice_email_sent_to(&self, expected: &str) {
        let email = self
            .drivers
            .email_outbox
            .last()
            .expect("expected invoice email to be sent");
        assert_eq!(
            email.to.as_str(),
            expected,
            "invoice email recipient should match"
        );
    }

    pub fn then_invoice_email_includes(&self, expected: InvoiceEmailExpectation) {
        let email = self
            .drivers
            .email_outbox
            .last()
            .expect("expected invoice email to be sent");
        match &email.body {
            core_ports::ordering::EmailBody::Structured(body) => {
                assert_eq!(
                    body.school.as_str(),
                    expected.school.as_str(),
                    "invoice email school"
                );
                assert_eq!(
                    body.delivery_window.as_str(),
                    expected.delivery_window.as_str(),
                    "invoice email delivery window"
                );
                assert_eq!(
                    body.line_item_description.as_str(),
                    expected.line_item_description.as_str(),
                    "invoice email line item description"
                );
                assert_eq!(
                    body.line_item_quantity, expected.line_item_quantity,
                    "invoice email line item quantity"
                );
                assert_f64_close(
                    body.line_item_price,
                    expected.line_item_price,
                    "invoice email line item price",
                );
            }
        }
    }

    pub fn then_invoice_has_tax_and_shipping(&self) {
        let invoice = self
            .drivers
            .invoices
            .last()
            .expect("expected invoice to be created");
        assert!(invoice.tax.is_some(), "invoice should include tax");
        assert!(invoice.shipping.is_some(), "invoice should include shipping");
    }

    pub fn then_invoice_email_has_pdf_attachment(&self) {
        let email = self
            .drivers
            .email_outbox
            .last()
            .expect("expected invoice email to be sent");
        let has_pdf = email
            .attachments
            .iter()
            .any(|attachment| attachment.content_type == "application/pdf");
        assert!(has_pdf, "expected invoice email to include a PDF attachment");
    }

    pub fn then_cart_is_empty(&self) {
        let email = self.require_current_email();
        let items = self.drivers.carts.items_for(email);
        assert!(items.is_empty(), "expected cart to be empty");
    }

    fn require_current_email(&self) -> &str {
        self.state
            .current_employee_email
            .as_deref()
            .expect("current employee email must be set")
    }

    fn require_quote(&self) -> &Quote {
        self.state
            .quote_result
            .as_ref()
            .expect("quote must be requested before assertions")
            .as_ref()
            .expect("quote request should succeed")
    }
}

#[derive(Default)]
struct TestState {
    pending_employee: Option<SignUpEmployeeInput>,
    current_employee_email: Option<String>,
    sign_up_result: Option<Result<SignUpEmployeeOutput, SignUpError>>,
    sign_in_result: Option<Result<SignInEmployeeOutput, AuthError>>,
    sign_out_result: Option<Result<(), AuthError>>,
    add_to_cart_result: Option<Result<(), CartError>>,
    quote_result: Option<Result<Quote, QuoteError>>,
    submit_result: Option<Result<SubmitQuoteOutput, QuoteError>>,
}

fn assert_f64_close(actual: f64, expected: f64, context: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= 0.0001,
        "{context} expected {expected:.5}, got {actual:.5}"
    );
}
