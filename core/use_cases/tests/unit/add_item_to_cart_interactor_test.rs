use core_entities::ordering::CartLineItem;
use core_ports::ordering::{CartRepository, CatalogRepository};
use core_ports::RepoError;
use core_use_cases::ordering::{AddItemToCartInput, AddItemToCartInteractor};
use std::cell::RefCell;
use std::collections::HashMap;

struct InMemoryCatalogRepository {
    prices: RefCell<HashMap<String, f64>>,
}

impl InMemoryCatalogRepository {
    fn new() -> Self {
        Self {
            prices: RefCell::new(HashMap::new()),
        }
    }

    fn set_price(&self, description: &str, price: f64) {
        self.prices
            .borrow_mut()
            .insert(description.to_string(), price);
    }
}

impl CatalogRepository for InMemoryCatalogRepository {
    fn price_for(&self, description: &str) -> Result<Option<f64>, RepoError> {
        Ok(self.prices.borrow().get(description).copied())
    }
}

struct InMemoryCartRepository {
    carts: RefCell<HashMap<String, Vec<CartLineItem>>>,
}

impl InMemoryCartRepository {
    fn new() -> Self {
        Self {
            carts: RefCell::new(HashMap::new()),
        }
    }
}

impl CartRepository for InMemoryCartRepository {
    fn add_item(&self, email: &str, item: CartLineItem) -> Result<(), RepoError> {
        let mut carts = self.carts.borrow_mut();
        carts.entry(email.to_string()).or_default().push(item);
        Ok(())
    }

    fn replace_cart(&self, email: &str, items: Vec<CartLineItem>) -> Result<(), RepoError> {
        self.carts.borrow_mut().insert(email.to_string(), items);
        Ok(())
    }

    fn items_for(&self, email: &str) -> Result<Vec<CartLineItem>, RepoError> {
        Ok(self
            .carts
            .borrow()
            .get(email)
            .cloned()
            .unwrap_or_default())
    }

    fn clear(&self, email: &str) -> Result<(), RepoError> {
        self.carts.borrow_mut().remove(email);
        Ok(())
    }
}

#[test]
fn add_item_to_cart_stores_quantity_and_price() {
    let catalog = InMemoryCatalogRepository::new();
    catalog.set_price("Poland Spring Water (48 ct/8 oz)", 20.00);
    let carts = InMemoryCartRepository::new();

    let interactor = AddItemToCartInteractor::new(&carts, &catalog);
    interactor
        .execute(AddItemToCartInput::new(
            "QWilliams@schools.nyc.gov",
            "Poland Spring Water (48 ct/8 oz)",
            8,
        ))
        .expect("add item to cart should succeed");

    let items = carts
        .items_for("QWilliams@schools.nyc.gov")
        .expect("items should be retrievable");
    assert_eq!(items.len(), 1, "expected one cart line item");
    assert_eq!(items[0].description.as_str(), "Poland Spring Water (48 ct/8 oz)");
    assert_eq!(items[0].quantity, 8);
    assert!((items[0].price - 20.00).abs() <= 0.0001);
}
