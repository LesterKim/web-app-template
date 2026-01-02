use crate::atdd::dsl::{ScenarioLog, TestDsl};
use crate::atdd::types::InvoiceEmailExpectation;
use core_entities::ordering::CartLineItem;

#[test]
fn employee_submits_quote_and_receives_invoice_email() {
    let _log = ScenarioLog::start(
        "Employee submits a quote and receives an invoice email",
        "creates invoice, sends email, and clears cart",
    );
    let mut app = TestDsl::new();
    app.given_signed_in_employee("QWilliams@schools.nyc.gov");
    app.given_school_code("28Q082");
    app.given_today_is("12/22/25");
    app.given_tax_rate(0.08875);
    app.given_shipping_rate(0.01);
    app.given_cart_contains(vec![CartLineItem::new(
        "Poland Spring Water (48 ct/8 oz)",
        8,
        20.00,
    )]);
    app.when_submit_quote();
    app.then_invoice_created_with_quote_number("28Q082x122225");
    app.then_invoice_email_sent_to("QWilliams@schools.nyc.gov");
    app.then_invoice_email_includes(InvoiceEmailExpectation::new(
        "P.S. 082 - The Hammond School",
        "School Hours",
        "Poland Spring Water (48 ct/8 oz)",
        8,
        20.00,
    ));
    app.then_invoice_has_tax_and_shipping();
    app.then_invoice_email_has_pdf_attachment();
    app.then_cart_is_empty();
}
