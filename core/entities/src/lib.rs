#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmployeeId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProductId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvoiceId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuoteId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Money {
    cents: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoneyError {
    pub message: String,
}

impl Money {
    pub fn from_cents(cents: i64) -> Result<Self, MoneyError> {
        if cents < 0 {
            return Err(MoneyError {
                message: "money cannot be negative".to_string(),
            });
        }
        Ok(Self { cents })
    }

    pub fn zero() -> Self {
        Self { cents: 0 }
    }

    pub fn cents(&self) -> i64 {
        self.cents
    }

    pub fn add(&self, other: Money) -> Money {
        Money {
            cents: self.cents + other.cents,
        }
    }

    pub fn multiply(&self, quantity: u32) -> Money {
        Money {
            cents: self.cents * quantity as i64,
        }
    }

    pub fn apply_rate_ppm(&self, rate_ppm: i64) -> Money {
        Money {
            cents: (self.cents * rate_ppm) / 100_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Employee {
    pub id: EmployeeId,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Employee {
    pub fn email_is_valid(email: &str) -> bool {
        let trimmed = email.trim();
        let parts: Vec<&str> = trimmed.split('@').collect();
        parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductCategory {
    Food,
    Accessory,
}

impl ProductCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ProductCategory::Food => "Food",
            ProductCategory::Accessory => "Accessory",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartItem {
    pub product_id: ProductId,
    pub quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cart {
    pub employee_id: EmployeeId,
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn empty(employee_id: EmployeeId) -> Self {
        Self {
            employee_id,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, product_id: ProductId, quantity: u32) -> Result<(), CartError> {
        if quantity == 0 {
            return Err(CartError::InvalidQuantity);
        }

        if let Some(item) = self.items.iter_mut().find(|item| item.product_id == product_id) {
            item.quantity = item.quantity.saturating_add(quantity);
        } else {
            self.items.push(CartItem {
                product_id,
                quantity,
            });
        }

        Ok(())
    }

    pub fn set_quantity(
        &mut self,
        product_id: ProductId,
        quantity: u32,
    ) -> Result<(), CartError> {
        if let Some(item) = self.items.iter_mut().find(|item| item.product_id == product_id) {
            if quantity == 0 {
                self.items.retain(|item| item.product_id != product_id);
            } else {
                item.quantity = quantity;
            }
            Ok(())
        } else if quantity > 0 {
            self.items.push(CartItem {
                product_id,
                quantity,
            });
            Ok(())
        } else {
            Err(CartError::MissingItem)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CartError {
    InvalidQuantity,
    MissingItem,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuoteLine {
    pub product_id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
    pub quantity: u32,
    pub line_total: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quote {
    pub items: Vec<QuoteLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuoteDraft {
    pub employee_id: EmployeeId,
    pub items: Vec<QuoteLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
    pub submitted_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuoteRecord {
    pub id: QuoteId,
    pub employee_id: EmployeeId,
    pub items: Vec<QuoteLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
    pub submitted_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvoiceLine {
    pub product_id: ProductId,
    pub name: String,
    pub category: ProductCategory,
    pub unit_price: Money,
    pub quantity: u32,
    pub line_total: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invoice {
    pub id: InvoiceId,
    pub employee_id: EmployeeId,
    pub items: Vec<InvoiceLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvoiceDraft {
    pub employee_id: EmployeeId,
    pub items: Vec<InvoiceLine>,
    pub subtotal: Money,
    pub fee: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub token: String,
    pub employee_id: EmployeeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PricingPolicy {
    pub flat_fee: Money,
    pub tax_rate_ppm: i64,
}

impl PricingPolicy {
    pub fn calculate_tax(&self, taxable_amount: Money) -> Money {
        taxable_amount.apply_rate_ppm(self.tax_rate_ppm)
    }
}

impl Default for PricingPolicy {
    fn default() -> Self {
        Self {
            flat_fee: Money::from_cents(495).unwrap_or_else(|_| Money::zero()),
            tax_rate_ppm: 8_875,
        }
    }
}
