use crate::atdd::dsl::{ScenarioLog, TestDsl};
use crate::atdd::types::QuoteLineExpectation;
use core_entities::ordering::CartLineItem;

#[test]
fn employee_views_quote_for_cart() {
    let _log = ScenarioLog::start(
        "Employee views a quote for their cart",
        "builds a quote with totals and delivery window",
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
    app.when_view_quote();
    app.then_quote_number_is("28Q082x122225");
    app.then_quote_includes(vec![QuoteLineExpectation::new(
        "Poland Spring Water (48 ct/8 oz)",
        8,
        20.00,
        160.00,
    )]);
    app.then_quote_subtotal_is(160.00);
    app.then_quote_tax_is_subtotal_times_rate();
    app.then_quote_shipping_is_subtotal_times_rate();
    app.then_quote_total_is_subtotal_plus_tax_plus_shipping();
    app.then_delivery_window_is("School Hours");
}
