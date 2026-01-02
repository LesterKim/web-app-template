use core_entities::ordering::CartLineItem;
use core_ports::ordering::{CartRepository, CatalogRepository};

use super::errors::CartError;

pub struct AddItemToCartInteractor<'a> {
    carts: &'a dyn CartRepository,
    catalog: &'a dyn CatalogRepository,
}

impl<'a> AddItemToCartInteractor<'a> {
    pub fn new(carts: &'a dyn CartRepository, catalog: &'a dyn CatalogRepository) -> Self {
        Self { carts, catalog }
    }

    pub fn execute(&self, input: AddItemToCartInput) -> Result<(), CartError> {
        let price = self
            .catalog
            .price_for(&input.description)
            .map_err(|err| CartError::new(err.message))?
            .ok_or_else(|| CartError::new("catalog item not found"))?;

        let item = CartLineItem::new(&input.description, input.quantity, price);
        self.carts
            .add_item(&input.email, item)
            .map_err(|err| CartError::new(err.message))?;
        Ok(())
    }
}

pub struct AddItemToCartInput {
    pub email: String,
    pub description: String,
    pub quantity: u32,
}

impl AddItemToCartInput {
    pub fn new(email: &str, description: &str, quantity: u32) -> Self {
        Self {
            email: email.to_string(),
            description: description.to_string(),
            quantity,
        }
    }
}
