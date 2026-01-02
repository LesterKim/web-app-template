use crate::atdd::dsl::{ScenarioLog, TestDsl};
use crate::atdd::types::CartLineExpectation;

#[test]
fn employee_adds_items_to_cart() {
    let _log = ScenarioLog::start(
        "Employee adds items to their cart",
        "stores quantities for catalog items",
    );
    let mut app = TestDsl::new();
    app.given_signed_in_employee("QWilliams@schools.nyc.gov");
    app.given_catalog_item("Poland Spring Water (48 ct/8 oz)", 20.00);
    app.when_add_to_cart("Poland Spring Water (48 ct/8 oz)", 8);
    app.then_cart_contains(vec![CartLineExpectation::new(
        "Poland Spring Water (48 ct/8 oz)",
        8,
    )]);
}
