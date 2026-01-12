pub mod postgres;

use core_entities::{
    Cart, Employee, EmployeeId, Invoice, InvoiceDraft, InvoiceId, Money, Product, ProductCategory,
    ProductId, QuoteDraft, QuoteId, QuoteRecord, Session,
};
use core_ports::{
    BoxFuture, CartRepository, CatalogRepository, EmailError, EmailGateway, EmployeeRepository,
    InvoiceRepository, QuoteRepository, RepoError, SessionRepository,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryEmployeeRepository {
    employees: Arc<Mutex<Vec<Employee>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryEmployeeRepository {
    pub fn new(seed: Vec<Employee>) -> Self {
        let next_id = seed
            .iter()
            .map(|employee| employee.id.0)
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            employees: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(next_id)),
        }
    }
}

impl EmployeeRepository for InMemoryEmployeeRepository {
    fn add_employee<'a>(
        &'a self,
        name: String,
        email: String,
        password: String,
    ) -> BoxFuture<'a, Result<Employee, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let employee = Employee {
                id: EmployeeId(*next_id),
                name,
                email,
                password,
            };
            *next_id += 1;
            self.employees.lock().unwrap().push(employee.clone());
            Ok(employee)
        })
    }

    fn find_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        Box::pin(async move {
            let employee = self
                .employees
                .lock()
                .unwrap()
                .iter()
                .find(|employee| employee.email == email)
                .cloned();
            Ok(employee)
        })
    }

    fn get_by_id<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        Box::pin(async move {
            let employee = self
                .employees
                .lock()
                .unwrap()
                .iter()
                .find(|employee| employee.id == employee_id)
                .cloned();
            Ok(employee)
        })
    }
}

pub struct InMemoryCatalogRepository {
    products: Arc<Vec<Product>>,
}

impl InMemoryCatalogRepository {
    pub fn new(products: Vec<Product>) -> Self {
        Self {
            products: Arc::new(products),
        }
    }

    pub fn seeded() -> Self {
        Self::new(seeded_products())
    }
}

impl CatalogRepository for InMemoryCatalogRepository {
    fn list_products<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Product>, RepoError>> {
        let products = self.products.as_ref().clone();
        Box::pin(async move { Ok(products) })
    }

    fn find_product<'a>(
        &'a self,
        product_id: ProductId,
    ) -> BoxFuture<'a, Result<Option<Product>, RepoError>> {
        let product = self
            .products
            .iter()
            .find(|product| product.id == product_id)
            .cloned();
        Box::pin(async move { Ok(product) })
    }
}

// Seed data sourced from references/new_order_form.csv.
fn seeded_products() -> Vec<Product> {
    vec![
        Product {
            id: ProductId(1),
            name: "Shelf Stable Pantry Kit".to_string(),
            category: ProductCategory::new("Kit"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(2),
            name: "Shelf Stable Pantry Kit in Drawstring Bag".to_string(),
            category: ProductCategory::new("Kit"),
            unit_price: Money::from_cents(2700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(3),
            name: "Female Hygiene Kits *NEW CONTENTS !".to_string(),
            category: ProductCategory::new("Kit"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(4),
            name: "Male Hygiene Kits * NEW CONTENTS !".to_string(),
            category: ProductCategory::new("Kit"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(5),
            name: "Kommissary Laundry Kit".to_string(),
            category: ProductCategory::new("Kit"),
            unit_price: Money::from_cents(7000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(6),
            name: "Kommissary Christmas Kit (A Set of a Box and a Bag)".to_string(),
            category: ProductCategory::new("Christmas: Kit"),
            unit_price: Money::from_cents(6000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(7),
            name: "The Dorm Room Chef".to_string(),
            category: ProductCategory::new("Bundle"),
            unit_price: Money::from_cents(50000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(8),
            name: "The Snack Stash".to_string(),
            category: ProductCategory::new("Bundle"),
            unit_price: Money::from_cents(50000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(9),
            name: "The Guilt-free Goodies".to_string(),
            category: ProductCategory::new("Bundle"),
            unit_price: Money::from_cents(50000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(10),
            name: "The Glow Up Pack".to_string(),
            category: ProductCategory::new("Bundle"),
            unit_price: Money::from_cents(50000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(11),
            name: "Blue Bunny Chocolate Ice Cream Cup (48 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(12),
            name: "Blue Bunny Vanilla Ice Cream Cup (48 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(13),
            name: "Blue Bunny Strawberry Ice Cream Cup (48 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(14),
            name: "Luigi's Italian Ice Cherry Intermezzo Cup (72 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(8100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(15),
            name: "Luigi's Italian Ice Lemon Intermezzo Cup (72 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(8100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(16),
            name: "Luigi's Italian Ice Orange Intermezzo Cup (72 pk/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(8100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(17),
            name: "Haagen-Dazs Strawberry Ice Cream Cup (12 pk/3.6 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(18),
            name: "Haagen-Dazs Dulce de Leche Ice Cream Cup (12 pk/3.6 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(19),
            name: "Haagen-Dazs Vanilla Ice Cream Cup (12 pk/3.6 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(20),
            name: "Good Humor Cookie & Cream Ice Cream Oreo Bar (24 ct/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(21),
            name: "Good Humor Strawberry Shortcake Ice Cream Bar (24 ct/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(22),
            name: "Good Humor Chocolate Eclair Ice Cream Bar (24 ct/4 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(23),
            name: "Haagen-Dazs Chocolate Coated Vanilla Bar (12 ct/3 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(5400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(24),
            name: "Haagen-Dazs Chocolate & Almond Coated Vanilla Bar (12 ct/3 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(5400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(25),
            name: "Bomb Pop Popsicle Original: Mixed Flavor of Cherry, Lime, Raspberry (24 ct/3.75 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(26),
            name: "Bomb Pop Popsicle Nerds: Mixed Flavor of Strawberry, Watermelon, Grape (24 ct/3.75 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(27),
            name: "Teenage Ninja Mutant Turtles Ice Cream Bar (18 ct/3.5 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(28),
            name: "Blue Bunny Vanilla Ice Cream Cone (24 ct/4.6 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(29),
            name: "Good Humor Reese's Peanut Butter Ice Cream Bar (24 ct/3.3 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(30),
            name: "Klondike Vanilla Original Giant Bar (24 ct/5.5 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(31),
            name: "Klondike Oreo Ice Cream Sandwich (24 ct/4.5 oz)".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(32),
            name: "Nestle Toll House Chocolate Chip Ice Cream Sandwich (12 ct/6 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(6000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(33),
            name: "Blue Bunny Blue Ribbon Vanilla with Chocolate Cookie Ice Cream Sandwich (48 ct/3.5 oz) *Kosher".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(34),
            name: "Nightingale Chocolate Ice Cream Sandwich (24 ct/2.6 oz)".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(7200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(35),
            name: "Nightingale Strawberry Ice Cream Sandwich (24 ct/2.6 oz)".to_string(),
            category: ProductCategory::new("ICE CREAM"),
            unit_price: Money::from_cents(7200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(36),
            name: "Rainbow Sprinkles (2.2 lbs)".to_string(),
            category: ProductCategory::new("Sprinkles"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(37),
            name: "Chocolate Sprinkles (2.2 lbs)".to_string(),
            category: ProductCategory::new("Sprinkles"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(38),
            name: "Hershey Dessert Syrup 3 Flavor Variety Pack: Chocolate (24 oz) & Caramel, Strawberry (22 oz)".to_string(),
            category: ProductCategory::new("Syrup"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(39),
            name: "Individually Wrapped Wooden Ice Cream Spoon (100 ct)".to_string(),
            category: ProductCategory::new("Spoon"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(40),
            name: "Igloo Glide Cooler (110 Quart)".to_string(),
            category: ProductCategory::new("Cooler"),
            unit_price: Money::from_cents(23000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(41),
            name: "Capri Sun Flavored Juice Drink Blend Variety Pack (40 ct/6 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(42),
            name: "Juicy Juice Variety Pack (32 ct/6.75 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(43),
            name: "Dole 100% Pineapple Juice Cans (24 pk/8 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(44),
            name: "Mott's 100% Original Apple Juice (24 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(45),
            name: "Apple & Eve Fruit Juice Variety Pack (Fruit Punch, Apple, Very Berry) (36 ct/6.7 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(46),
            name: "Hawaiian Punch Variety Pack Juice Drink (24 pk/10 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(47),
            name: "Ocean Spray Tropical Variety Pack (24 ct/10 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(48),
            name: "Welch's Juice Drink Variety Pack (24 pk/10 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(49),
            name: "Tropicana 100% Juice Orange Blend (24 pk/10 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(50),
            name: "Snapple Juice Drink Variety Pack (24 pk/20 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(51),
            name: "Honest Kids Juice Boxes Variety Pack (40 ct/6 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(52),
            name: "Gatorade Thirst Quencher Variety (28 pk/12 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(53),
            name: "Gatorade Thirst Quencher Zero Sugar 3-Flavor Pack (28 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(54),
            name: "Pedialyte Advanced Care+ Liter Pack - Berry Frost (3 ct/1 L)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(55),
            name: "Vita Coco Coconut Water (18 ct/11 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(56),
            name: "Yoohoo Chocolate Drink (40 pk/6.5 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(57),
            name: "Horizon Organic Shelf-Stable 1% Low Fat Chocolate Milk (18 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(58),
            name: "Califia Oat Milk Barista Blend (12 pk/32 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(59),
            name: "Almond Breeze Unsweetened Original Almond Milk (6 pk/32 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(60),
            name: "Almond Breeze Unsweetened Vanilla Almond Milk (6 pk/32 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(61),
            name: "Silk Soy Milk Original (12 pk/1 qt)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(62),
            name: "Farmland Shelf Stable 1% Milk (27 ct/8 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(63),
            name: "Shelf Stable Whole Aseptic Milk (12 ct/32 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3930).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(64),
            name: "Poland Spring Water (48 ct/8 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(65),
            name: "Poland Spring Water (40 ct/16.9 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(66),
            name: "Saratoga Spring Water - STILL Water (24 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(67),
            name: "Saratoga Spring Water - STILL Water (12 pk/28 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(68),
            name: "Saratoga Spring Water - SPARKLING Water (24 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(69),
            name: "Saratoga Spring Water - SPARKLING Water (12 pk/28 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(70),
            name: "S.Pellegrino Sparkling Natural Mineral Water Plastic Bottles (24 ct/16.9 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(71),
            name: "LaCroix Sparkling Variety Pack (24 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(72),
            name: "Polar Seltzer Cans Variety Pack (30 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(73),
            name: "FIJI Water (24 pk/16.9 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3550).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(74),
            name: "Arizona Green Tea with Ginseng and Honey (24 pk/16 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(75),
            name: "AriZona Lemon Iced Tea (12 pk/16 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1680).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(76),
            name: "Arizona Arnold Palmer Half & Half Ice Tea Lemonade (30 pk/11.5 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(77),
            name: "Lipton Brisk Lemon Iced Tea Cans (36 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(78),
            name: "Snapple Lemon Tea Naturally Flavored (24 pk/16 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(79),
            name: "Snapple Peach Tea Naturally Flavored (24 pk/16 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(80),
            name: "Coca-Cola Original Cans (35 pk/12 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(81),
            name: "Coca-Cola Zero Sugar Cans (35 pk/12 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(82),
            name: "Diet Coke Soda Cans (35 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(83),
            name: "Coca-Cola Mini Cans Variety Pack (30 pk/7.5 oz; 10 pk of Coca-cola Cherry, Sprite, Fanta Orange)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(84),
            name: "Pepsi Cola Cans (36 ct/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(85),
            name: "Sprite Original Soda Cans (35 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(86),
            name: "Canada Dry Ginger Ale (24 ct/12 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(87),
            name: "Dr. Pepper Cans (35 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(88),
            name: "Sunkist Orange Soda Can (24 ct/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(89),
            name: "OLIPOP Prebiotic Soda Pop Variety Pack (12 pk/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(90),
            name: "Poppi Prebiotic Soda Variety Pack (15 pk/12 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(91),
            name: "Celsius Vibe Variety Pack (18 ct/12 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(92),
            name: "PediaSure Grow & Gain Kids Nutritional Shake Strawberry (24 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(6750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(93),
            name: "PediaSure Grow & Gain Kids' Nutritional Shake Vanilla (24 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(6750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(94),
            name: "Ensure Original Vanilla Nutrition Shake 9g Protein (24 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(6200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(95),
            name: "Ensure Original Strawberry Nutrition Shake 9g Protein (24 pk/8 fl. oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(6200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(96),
            name: "Premier Protein Vanilla Ready to Drink Shake (15 ct/11 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(5100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(97),
            name: "Premier Protein Chocolate Ready to Drink Shake (15 ct/11 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(5100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(98),
            name: "OWYN 30g Protein Shake Zero Sugar Vanilla (15 ct/11.15 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(99),
            name: "OWYN 30g Protein Shake Zero Sugar Chocolate (15 ct/11.15 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(100),
            name: "Carnation Breakfast Essentials Nutritional Chocolate Drink Mix (30 packets)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(101),
            name: "Boost High 20g Protein Chocolate Nutritional Shake (28 ct/8 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(6200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(102),
            name: "Lipton Lemon Iced Tea Mix (38 qt)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(103),
            name: "Crystal Light Peach Tea Sticks (16 ct)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(104),
            name: "Crystal Light Lemon Iced Tea Sticks (16 ct)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(105),
            name: "Nesquik Chocolate Flavored Powder (44.9 oz)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(106),
            name: "Propel Powder Variety Pack (40 packets)".to_string(),
            category: ProductCategory::new("Beverages"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(107),
            name: "Mr. Coffee Coffee Maker, Rapid Brew (12 cups)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(108),
            name: "45 Cups Double Wall Stainless Steel Coffee Percolator".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(109),
            name: "Melitta #4 Cone Coffee Filters (300 ct)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(110),
            name: "La Colombe Coffee Draft Latte Variety Pack (12 pk/9 fl. oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(111),
            name: "Starbucks Mocha Frappuccino (15 ct/9.5 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(112),
            name: "Starbucks Vanilla Frappuccino (15 ct/9.5 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(113),
            name: "Cafe Bustelo Espresso Ground Coffee (4 pk/10 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(114),
            name: "Cafe Bustelo Espresso Ground Coffee Canister (46 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(115),
            name: "Starbucks Caffe Verona Dark Roast Ground Coffee (40 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(116),
            name: "Starbucks Pike Place Roast Medium Roast Ground Coffee (40 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(117),
            name: "Dunkin' Donuts Original Blend Medium Roast Ground Coffee (40 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(4500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(118),
            name: "Folgers Classic Roast Ground Coffee (43.5 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(119),
            name: "Maxwell House Medium Roast Original Ground Coffee (43.1 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(120),
            name: "Stumptown Organic Medium Roast Ground Holler Mtn. Pack of 3 (3 pk/12 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(7100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(121),
            name: "Illy Ground Drip Coffee Classico Medium Roast (6 pk/8.8 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(12800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(122),
            name: "Sugar In The Raw Brown Sugar (400 pk/case)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(123),
            name: "Equal 0 Calorie Sweetener (1,000 ct/3 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(124),
            name: "Splenda (1,000 ct)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(125),
            name: "Truvia Calorie-Free Sweetener (400 ct)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(3150).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(126),
            name: "Domino Sugar Packets (2,000 ct)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(127),
            name: "Coffee Mate Original Powdered Coffee Creamer (56.015 oz)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(1550).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(128),
            name: "Nestle CoffeeMate Creamer 3 Flavor Pack (50ct of Original, Hazelnut, French Vanilla)".to_string(),
            category: ProductCategory::new("Coffee"),
            unit_price: Money::from_cents(4600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(129),
            name: "Keurig Single Serve K-Cup Pod Coffee Maker (6 to 12 oz)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(14000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(130),
            name: "Keurig Single Serve K-Cup Pod Coffee Maker, 4 Brew Sizes (66 oz)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(23800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(131),
            name: "Maxwell House, Medium Roast K-Cup (100 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(7200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(132),
            name: "Cafe Bustelo Espresso K-Cup (80 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(6300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(133),
            name: "Dunkin' Donuts Original Blend K-Cup Pods (72 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(7650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(134),
            name: "Dunkin' Donuts Original Blend Decaf K-Cup Pods (54 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(6750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(135),
            name: "Starbucks Pike Place Roast Medium Roast K-Cup Pods (72 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(7800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(136),
            name: "Starbucks French Dark Roast K-Cup (72 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(7800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(137),
            name: "La Colombe Coffee Nizza Medium Roast K-Cup (20 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(138),
            name: "The Original Donut Shop Medium Roast K-Cup (96 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(7200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(139),
            name: "Tim Hortons Original Blend Medium Roast K-Cup (72 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(5900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(140),
            name: "Illy Coffee Classico Roast K-Cup (32 ct)".to_string(),
            category: ProductCategory::new("Coffee Pods"),
            unit_price: Money::from_cents(4600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(141),
            name: "Electric Kettle for Hot Water and Tea (1.7 Liter)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(4000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(142),
            name: "Maud's Blend Organic Tea Pods 9 Flavor Variety Pack (48 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(4500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(143),
            name: "Bigelow Caffeine Free Chamomile Herbal Tea (100 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(144),
            name: "Bigelow Organic Green Tea (150 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(1900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(145),
            name: "Lipton Tea Bags (312 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(146),
            name: "Twinings Earl Grey Black Tea (100 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(147),
            name: "Swiss Miss Hot Cocoa Mix (50 packets/case)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(148),
            name: "Nestle Hot Chocolate Powder (50 packets)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(149),
            name: "Keurig Cleaner Pods (12 ct)".to_string(),
            category: ProductCategory::new("Hot Beverage"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(150),
            name: "Rockland Bakery 10\" Apple Pie (5 Pie Set)".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(13500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(151),
            name: "Rockland Bakery 10\" Pumpkin Pie (5 Pie Set)".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(13500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(152),
            name: "Large Plain Butter Croissant Individually Wrapped (5 Dozen Set)".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(13500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(153),
            name: "Chocolate Croissant Wrapped, Large (5 dozen set)".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(154),
            name: "Rockland Bakery, White Bread, Fresh 1.5LB (1 Loaf). Increment order of 10x loaves".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(5000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(155),
            name: "Rockland Bakery, Bread Whole Wheat, Fresh 1.5LB (1 Loaf). Increment order of 10x loaves".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(5000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(156),
            name: "Mannys Jumbo Loaf Slice Wrapped (5 dozen set) Choose from: Banana Nut, Carrot, Chocolate Chip, Iced Lemon, Marble".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(157),
            name: "Premier Pastries Donuts, Wrapped (5 dozen set). Choose from Boston Cream, Jelly, Pink Frosted, Vanilla Frosting, Chocolate Frosting, Double Glazed".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(158),
            name: "Large Yogurt Muffn Individually Wrapped 5 Dozen Set *Choose Flavor from: Banana Nut, Blueberry, Corn, Orange, Chocolate Chip".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(16500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(159),
            name: "Muffins 'n Stuff Black & White Cookie Wrapped, Large (5 dozen set)".to_string(),
            category: ProductCategory::new("Bakery"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(160),
            name: "Kellogg's Cold Breakfast Cereal Single Serve Variety Pack (25 pk)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(161),
            name: "Kellogg's Breakfast Cereal Cups Variety Pack (96 ct/0.75-1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(10000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(162),
            name: "Kellogg's Frosted Flakes Bowl (96 ct/1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(163),
            name: "Honey Nut Cheerios Whole Grain (96 ct/1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(164),
            name: "Apple Cinnamon Cheerios Whole Grain (96 ct/1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(165),
            name: "CoCo Puffs Reduced Sugar Whole Grain (96 ct/1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(166),
            name: "Cinnamon Toast Crunch Reduced Sugar Whole Grain (96 ct/1 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(167),
            name: "belVita Blueberry Breakfast Biscuits (25 pk/4 ea)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(168),
            name: "belVita Cinnamon Brown Sugar Breakfast Biscuits (25 pk/4 ea)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(169),
            name: "Smucker's Uncrustables: Thaw & Eat! (Frozen) (48 ct/2.6 oz; Peanut Butter + Grape Jelly)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(6000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(170),
            name: "Smucker's Uncrustables: Thaw & Eat! (Frozen) Peanut Butter & Strawberry Jam (48 ct/2.6 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(6000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(171),
            name: "Quaker Oatmeal Pouches 3 Flavor Variety Pack (52 ct/1.46 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(172),
            name: "Quaker Oatmeal Bowl Apple Cinnamon (24 ct/1.51 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(173),
            name: "Quaker Oatmeal Bowl Maple Brown Sugar (24 ct/1.69 oz)".to_string(),
            category: ProductCategory::new("Breakfast"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(174),
            name: "Gerber Natural for Toddler 12+ Months (12 pk/3.5 oz)".to_string(),
            category: ProductCategory::new("Baby Food"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(175),
            name: "Gerber Baby Snacks For Baby Puffs (8 pk/1.48 oz)".to_string(),
            category: ProductCategory::new("Baby Food"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(176),
            name: "Gerber Baby Snacks Lil Crunchies (8 pk/1.48 oz)".to_string(),
            category: ProductCategory::new("Baby Food"),
            unit_price: Money::from_cents(2700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(177),
            name: "Happy Baby Organic Fruit and Oat Variety Pack (12 ct/4 oz)".to_string(),
            category: ProductCategory::new("Baby Food"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(178),
            name: "Beech-Nut Veggies and Fruities Stage 2 (18 pk/3.5 oz)".to_string(),
            category: ProductCategory::new("Baby Food"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(179),
            name: "Little Potato Co. Little Duos (3 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(180),
            name: "Idaho Potatoes (5 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(181),
            name: "Sweet Potatoes (3 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(182),
            name: "Organic Whole Garlic (5 ct)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(183),
            name: "Yellow Onions (3 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(184),
            name: "Red Onions (3 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(185),
            name: "Whole Carrot (5 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(186),
            name: "Lemons (2 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(187),
            name: "Mandarins (5 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(1200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(188),
            name: "Bananas (3 lbs)".to_string(),
            category: ProductCategory::new("Fresh Produce"),
            unit_price: Money::from_cents(500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(189),
            name: "Del Monte Sweet Peas (8 pk/15 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(190),
            name: "Del Monte Canned Cut Green Beans (12 pk/15.25 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(191),
            name: "Del Monte Canned Cut Corn (12 pk/15.25 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(192),
            name: "Pearls Large Pitted Ripe Black Olives (16 pk/1.2 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2050).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(193),
            name: "Goya Pinto Beans (8 pk/15.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(194),
            name: "Goya Red Kidney Beans (8 pk/15.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(195),
            name: "Goya Black Beans (8 pk/15.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(196),
            name: "Goya Chick Peas (8 pk/15.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(197),
            name: "Bush's Original Baked Beans (8 pk/16.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(198),
            name: "Hormel Mary's Kitchen Corned Beef Hash (6 pk/14 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(199),
            name: "Hormel Chili with Beans Cans (6 ct/15 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(200),
            name: "Libby's Vienna Sausage (18 ct/4.6 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(201),
            name: "Spam, 25% Less Sodium (6 pk/12 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(202),
            name: "Starkist Light Tuna Pouches (10 ct/2.6 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(203),
            name: "Genova Yellowfin Tuna in Olive Oil (8 pk/5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(204),
            name: "Bumble Bee Solid White Albacore Tuna in Water (8 pk/5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(205),
            name: "Bumble Bee Skinless and Boneless Wild Pink Salmon (5 pk/5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(206),
            name: "Bumble Bee Snack Chicken Salad Kit with Crackers (12 pk)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(3080).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(207),
            name: "Bumble Bee Snack Tuna Salad Kit with Crackers (12 pk)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(3080).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(208),
            name: "Ben's Original Ready Rice (6 ct/8.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(209),
            name: "Prego Traditional Pasta Sauce (3 pk/45 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(210),
            name: "Ragu Chunky Tomato, Garlic and Onion Pasta Sauce (3 pk/45 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(211),
            name: "Barilla Penne, Spaghetti & Elbows, Non-GMO Kosher (6 pk/1 lb)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(212),
            name: "Barilla Rotini and Farfalle Pasta (6 pk/1 lb)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(213),
            name: "Hunt's Tomato Sauce (12 pk/15 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(214),
            name: "Hunt's Diced Tomatoes (8 pk/14.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(215),
            name: "Amy's Organic Lentil Soups (8 pk/14.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(216),
            name: "Campbell's Tomato Soup Cans (12 ct/10.75 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(217),
            name: "Campbell's Chicken Noodle Soup Cans (12 ct/10.75 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(218),
            name: "Swanson Chicken Broth Lower Sodium (6 pk/32 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(219),
            name: "Swanson 100% Natural Beef Broth (3 pk/32 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(220),
            name: "Buttery Homestyle Mashed Potatoes in Pouches (8 pk)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(221),
            name: "Chef Boyardee Variety Cups Microwavable (12 ct/7.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(222),
            name: "Kraft Mac and Cheese Cups Microwavable (12 ct/2.05 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(223),
            name: "Annie's Homegrown Organic Mac and Cheese Variety Pack (12 ct/6 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(224),
            name: "Velveeta Original Microwavable Pasta (12 pk/2.39 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(225),
            name: "Stove Top Stuffing Mix for Chicken (6 pk/6 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(226),
            name: "Carolina White Rice - Extra Long Grain (12 ct/2 lb)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(227),
            name: "ParExcellence Yellow Rice (3.5 lbs)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(1150).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(228),
            name: "Skippy Peanut Butter Creamy Regular Jar (12 ct/16 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(5800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(229),
            name: "Smucker's Jelly Grape Squeeze Bottle (12 ct/20 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(5700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(230),
            name: "Hellmann's Mayonnaise Squeeze Bottle (12 ct/11.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(9000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(231),
            name: "Heinz Tomato Ketchup (16 ct/14 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(5900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(232),
            name: "Nutella Hazelnut Spread (2 pk/26.5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(2300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(233),
            name: "Cholula Hot Sauce Glass Bottles (24 ct/5 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(10100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(234),
            name: "Sriracha Chili Sauce Shelf Stable Squeeze Bottle (12 ct/28 oz)".to_string(),
            category: ProductCategory::new("Pantry"),
            unit_price: Money::from_cents(9200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(235),
            name: "Nong Shim SHIN Ramyun Noodle Soup in Bag (16 pk/4.2 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(236),
            name: "Nong Shim Hot & Spicy Bowl Noodle Soup Microwavable Bowl (12 pk/3.03 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(237),
            name: "Nong Shim Spicy Chicken Bowl Noodle Soup Microwavable Bowl (12 pk/3.03 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(238),
            name: "Nong Shim Soon Vegan Kimchi Noodle Soup Microwavable Bowl (6 ct/2.64 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(1300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(239),
            name: "Samyang Buldak Carbonara Spicy Chicken Ramen Bowl - Non-microwavable (6 pk/3.7 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(240),
            name: "Nissin Cup Noodles Chicken Flavor Microwavable (24 ct/2.25 oz)".to_string(),
            category: ProductCategory::new("Noodles"),
            unit_price: Money::from_cents(1900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(241),
            name: "Pam Canola Grill & Pan Coating Spray (6 pk/17 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(4900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(242),
            name: "La Spagnola Canola Oil Plastic Bottle (12 ct/32 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(5600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(243),
            name: "Musselman's White Vinegar (12 ct/16 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(244),
            name: "Gold Medal All-Purpose Flour (8 ct/5 lbs)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(3300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(245),
            name: "Progresso Panko Crispy Bread Crumbs (6 pk/8 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(246),
            name: "Arm & Hammer Baking Soda (12 ct/1 lb)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(247),
            name: "Monarch Iodized Table Salt (24 ct/26 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(4000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(248),
            name: "McCormick Pure Ground Black Pepper (5 ct/16 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(11500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(249),
            name: "McCormick Italian Seasoning (5 ct/6.25 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(5000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(250),
            name: "Goya Sazon with Culantro and Achiote (36 pk/6.3 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(251),
            name: "Goya Adobo All Purpose Seasoning (5 ct/28 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(5500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(252),
            name: "Old Bay Seasoning (5 ct/24 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(7500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(253),
            name: "Old El Paso Taco Seasoning Mix (10 ct/1 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(1200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(254),
            name: "Badia Granulated Onion Seasoning (5 ct/20 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(5250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(255),
            name: "Badia Granulated Garlic Seasoning (5 ct/24 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(5750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(256),
            name: "Badia Sazon Complete Seasoning (5 ct/1.75 lbs)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(6750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(257),
            name: "Badia Paprika (5 ct/16 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(7750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(258),
            name: "Knorr Granulated Chicken Bouillon (3 ct/32 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(259),
            name: "Kikkoman Shelf Stable Soy Sauce Less Sodium (6 pk/0.5 GA)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(7800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(260),
            name: "Goya Coconut Milk (6 pk/13.5 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(261),
            name: "Carnation Evaporated Milk (8 ct/12 oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(262),
            name: "Nestle La Lechera Sweetened Condensed Milk Cans (6 pk/14 fl. oz)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(263),
            name: "Domino Pure Cane Granulated Sugar (24 ct/1 lb)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(4900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(264),
            name: "Domino Dark Brown Granulated Sugar (24 ct/1 lb)".to_string(),
            category: ProductCategory::new("Cooking"),
            unit_price: Money::from_cents(4900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(265),
            name: "Mott's Applesauce Cups No Sugar Added Variety Pack (36 ct/3.9 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(1900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(266),
            name: "GoGo SqueeZ Organic Applesauce Variety Pack (24 pk/3.2 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(267),
            name: "Mott's Fruit Flavored Snacks (90 ct/0.8 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(268),
            name: "Welch's Mixed Fruit Snacks (90 ct/0.8 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(269),
            name: "Dole Fruit Cups (36 ct/4 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(270),
            name: "Skittles Original Real Fruit Juice Fruit Snacks (42 ct)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(271),
            name: "Gushers Strawberry Splash and Tropical Flavor Fruit Snacks (42 ct/0.8 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(272),
            name: "Fruit by the Foot Variety Pack (48 ct)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(2700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(273),
            name: "Fruit Roll-Ups (72 ct)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(2700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(274),
            name: "Sun Maid Raisin Seedless Bulk Pack (24 pk/12 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(275),
            name: "Fla-Vor-Ice Popsicle Freezer Bars Variety Pack (100 ct/1.5 oz)".to_string(),
            category: ProductCategory::new("Fruit Snacks"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(276),
            name: "Ritz Original Crackers (18 Sleeves)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(277),
            name: "Ritz Bits Cheese Crackers (30 ct/1.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(278),
            name: "Premium Original Saltine Crackers (12 Sleeves)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(279),
            name: "Keebler Club Cracker Snack Stacks (24 Sleeves)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(280),
            name: "Cheez-It White Cheddar Baked Snack Pack (45 ct/1.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(281),
            name: "Cheez-It Whole Grain Bundle (175 ct/0.75 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(7500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(282),
            name: "Goldfish Crackers Sweet & Savory Variety Snack Packs (45 pk/1.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(283),
            name: "Goldfish Whole Grain Pretzel Snacks (300 ct/0.75 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(9600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(284),
            name: "Lance Sandwich Crackers Variety Pack (36 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(285),
            name: "Lance Sandwich Crackers Toasty Peanut Butter (40 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(286),
            name: "Honey Maid Honey Graham Crackers (4 pk/14.4 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(287),
            name: "Snack Pack Chocolate and Vanilla Pudding Variety Pack (36 ct/3.25 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(288),
            name: "Snyder's Hanover Mini Pretzels (36 ct/0.92 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(289),
            name: "Snack Factory Pretzel Crisps Original (26 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(290),
            name: "Dot's Pretzels Original Flavor Seasoned Pretzels (35 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(291),
            name: "Garden Veggie Straws Variety Snack Pack (30 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(292),
            name: "Takis Fuego Spicy Rolled Tortilla Chips Multipack (46 ct/1 oz) **DISCLAIMER: 2-3 MONTH EXPIRY".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(293),
            name: "Pringles Snack Stacks Variety Pack (48 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(294),
            name: "Wise Chips Grab & Snack Variety Pack (50 ct) **DISCLAIMER: 2-3 MONTH EXPIRY".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(295),
            name: "Cape Cod Kettle Cooked Original Reduced Fat (24 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(296),
            name: "Pirate's Booty (40 ct/0.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(297),
            name: "SkinnyPop Variety Pack (36 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(298),
            name: "PopCorners Variety Pack (28 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(299),
            name: "Act II Butter Lovers Microwave Popcorn (32 ct/2.75 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(300),
            name: "Pepperidge Farm Dark Chocolate Milano Cookies (20 pk/2 ea)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(301),
            name: "Goya Maria Cookies (10 Sleeves)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(302),
            name: "Tate's Bake Shop Chocolate Chip Cookies (4 pk/3.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(303),
            name: "Nabisco Cookies Variety Pack (Oreos, Chips Ahoy, Golden Oreo) (60 ct/2 ea)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(304),
            name: "Oreo Chocolate Sandwich Cookies (30 pk/2.4 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(305),
            name: "Chips Ahoy Cookies Chocolate Chip (40 ct/1.55 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(306),
            name: "Famous Amos Chocolate Chip Cookies (42 ct/2 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(307),
            name: "Newtons Soft & Chewy Fig Cookies Snack Packs (24 ct/2 ea)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(308),
            name: "Grandma's Chocolate Chip Cookie (60 ct/2.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(5000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(309),
            name: "Grandma's Oatmeal Raisin Cookie (60 ct/2.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(5000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(310),
            name: "Grandma's Mini Vanilla Creme Cookie (60 ct/2.12 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(311),
            name: "Lorna Doone Shortbread Cookie (120 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(5300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(312),
            name: "Darlington Soft Baked Short Bread Cookie (216 ct/.75 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(6700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(313),
            name: "Nilla Wafers Cookies, Vanilla Wafer (2 pk/15 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(314),
            name: "Nutella & Go with Breadsticks (16 ct/1.8 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(315),
            name: "Nonnis Cioccolati Biscotti (25 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(316),
            name: "Keebler Whole Wheat Animal Crackers (150 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(7300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(317),
            name: "Pop-Tarts Frosted Variety Pack (48 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(318),
            name: "Kellogg's Special K Cereal Bars (60 ct/2 ea)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(1750).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(319),
            name: "Rice Krispies Treats Original (60 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(320),
            name: "Nutri-Grain Bars Variety Pack (48 ct/1.3 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(321),
            name: "Kind Snacks Minis Nut Bars Variety Pack (32 ct/0.7 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(322),
            name: "Planters Roasted Salted Peanuts (144 ct/1 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(6000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(323),
            name: "Clif Kid ZBar Variety Pack with 2g Protein (36 ct/1.27 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(324),
            name: "Jack Links Beef Jerky Variety Pack (9 ct/1.25 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(325),
            name: "Chomps Grass-Fed and Finished Original Beef Jerky Snack Sticks (10 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(326),
            name: "Nature Valley Oats 'n Honey Granola Bars (49 ct/2 ea)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(327),
            name: "Nature Valley Fruit & Nut Trail Mix Chewy Granola Bars (48 ct/1.2 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(328),
            name: "Nature Valley Peanut Butter Chocolate Protein Bars (26 ct)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(329),
            name: "Nature Valley Chocolate Chunk and Oatmeal Raisin Assortment Granola Bars (120 ct/0.89 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(7700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(330),
            name: "Cracker Jack Popcorn (9 ct/8.5 oz)".to_string(),
            category: ProductCategory::new("Snacks"),
            unit_price: Money::from_cents(2700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(331),
            name: "Ghirardelli Chocolate Assorted Squares Bag (18.8 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(332),
            name: "Lindt Lindor Assorted Gold Bag (19 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(333),
            name: "Ferrero Rocher Fine Hazelnut Chocolates (48 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(334),
            name: "Toblerone Milk Chocolate Bars (6 ct/3.52 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(335),
            name: "York Snack Size Dark Chocolate Peppermint Patties (175 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(336),
            name: "Reese's Miniature Peanut Butter Cups (56 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(337),
            name: "Snickers, Twix & More Minis Chocolate Candy Bars Variety Pack (200 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(338),
            name: "Hershey's, Kit Kat & Reese's Full Size Chocolate Candy Bars Variety Pack (30 pk/case)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(339),
            name: "Snickers, Twix & More Chocolate Candy Bars, Full-Size Variety Pack (30 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(340),
            name: "Twix Full Size Caramel Cookie Chocolate Candy Bars (36 ct/1.79 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(6200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(341),
            name: "Hershey's Full Size Milk Chocolate Candy Bars (36 pk/1.55 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(5900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(342),
            name: "Kit Kat Full Size Milk Chocolate Wafer Candy Bars (36 pk/1.5 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(5900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(343),
            name: "Skittles and Starburst Chewy Candy Full Size Assorted (30 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(344),
            name: "Swedish Fish Mini Soft & Chewy Candy Snack Packs (24 pk/2 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(345),
            name: "Skittles, Starburst & Life Savers Fruity Candy Mini Variety Pack (150 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2550).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(346),
            name: "Sour Patch Kids and Swedish Fish Mini Variety Pack (200 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(347),
            name: "Twizzler & Jolly Rancher Variety Pack (260 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(348),
            name: "Starburst Original Fruity Chewy Candy Bulk Jar (54 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(349),
            name: "Airheads Candy Bars Variety Pack (60 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(350),
            name: "Jolly Rancher Assorted Hard Candy (5 lbs)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(351),
            name: "Werther's Original Caramel Hard Candies (30 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(352),
            name: "Spangler Dum Dums Original Pops (360 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(353),
            name: "Charms Blow Pop Assorted (100 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(354),
            name: "Tootsie Roll Favorites Variety Pack (4.75 lbs)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(355),
            name: "Life Savers Wint O Blue Breath Mints Bulk Hard Candy (53.95 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(356),
            name: "Life Savers Wint O Green Breath Mints Bulk Hard Candy (53.95 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(357),
            name: "Ricola Sugar-free Lemon Mint Herb Throat Drops (130 ct)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(358),
            name: "Ice Breakers Sugar-Free Cool Mints (8 pk/1.5 oz)".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(359),
            name: "Brachs Softmints Peppermint Carton".to_string(),
            category: ProductCategory::new("Candy"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(360),
            name: "Colgate 360 Whole Mouth Clean Toothbrush (8 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(361),
            name: "Travel Toothbrush Case Multicolor, Plastic (4 pk)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1150).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(362),
            name: "Gum Crayola Neon Clean Children's Soft Toothbrush (8 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(363),
            name: "Oral-B Glide Dental Floss All-in-One (6 pk)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(364),
            name: "Crest Pro-Health Advanced Multi-Benefit Toothpaste (5 ct/5.8 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(365),
            name: "Sensodyne Advanced Whitening Toothpaste for Sensitive Teeth and Cavity Protection (4 pk/6.5 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(4650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(366),
            name: "Colgate MaxFresh Cool Mint Toothpaste (5 pk/7.3 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2350).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(367),
            name: "Tom's of Maine Anticavity Kids Toothpaste Variety (3 pk/5.1 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(368),
            name: "ACT Kids Bubblegum Blowout Anti-Cavity Rinse (3 ct/16.9 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(369),
            name: "Crest Pro-Health Multi-Protection Mouthwash (3 pk/1L)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(370),
            name: "Neutrogena Daily Makeup Remover Cleansing Wipes (107 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(371),
            name: "Cetaphil Gentle Skin Facial Cleanser (2 ct/20 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(372),
            name: "Neutrogena Oil-Free Salicylic Acid Acne Face Wash (2 pk/9.1 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(373),
            name: "Cetaphil Moisturizing Cream (2 pk/16 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(374),
            name: "Lubriderm Daily Moisture Advanced Therapy Lotion (3 pk)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2550).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(375),
            name: "Aveeno Daily Moisturizing Body Lotion For Dry Skin (2 pk/18 fl. oz + 8oz Tube)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(376),
            name: "Eucerin Intensive Repair Body Lotion (2 pk/16.9 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(377),
            name: "Eucerin Hand Lotion (3 ct/2.7 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(378),
            name: "Dove Damage Therapy Shampoo (4 pk/12 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(379),
            name: "Dove Damage Therapy Conditioner (4 pk/12 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(380),
            name: "Dove Men+ Care Fortifying 2-in-1 Shampoo and Conditioner (4 pk/12 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(381),
            name: "Dove Beauty Bar White (16 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(382),
            name: "Dove Men + Care Extra Fresh Bar Soap (14 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(383),
            name: "Dove Deep Moisture Renewing Body Wash (3 pk/23 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(384),
            name: "Dove Men+ Care Body Wash (4 ct/18 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(4600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(385),
            name: "Dove Men+ Care Antiperspirant Deodorant Roll-on (4 pk/2.7 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(386),
            name: "Dove Men+ Care Antiperspirant Deodorant Dry Spray (3 pk/3.8 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(387),
            name: "Secret Invisible Solid Antiperspirant and Deodorant (5 pk)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(388),
            name: "Dove Clear Finish Antiperspirant Spray (3 pk/4.8 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(389),
            name: "Always Ultra Thin Daytime Pads with Wings, Size 1 Regular (3 pk/42 count)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(390),
            name: "Always Pocket Pads with FlexFoam & Wings, Size 1 Regular (22 count)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(391),
            name: "Tampax Pearl Unscented Tampons (96 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(392),
            name: "Dial Foaming Hand Wash Antibacterial & Moisturizing (4 pk/7.5 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(393),
            name: "Purell Prime Advanced Hand Sanitizer (4 pk/8 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(394),
            name: "Wet Ones Antibacterial Wipes Travel Pack (7 pk/20 Wipes)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2250).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(395),
            name: "Vaseline Lip with Aloe Chapstick".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(396),
            name: "Blistex Lip Care Variety Pack (11 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(397),
            name: "Vaseline Original Petroleum Jelly (2 pk/13 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(398),
            name: "Dove Moisturizing Hand Cream (4 ct/3 oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(399),
            name: "Coppertone Sport Sunscreen Lotion SPF 50 (6 ct/3 fl. oz)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(400),
            name: "Puffs Plus Lotion Facial Tissue (12 pk/56 sheets)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(401),
            name: "Kleenex Trusted Care 2-Ply Facial Tissues (12 pk/160 sheets)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(402),
            name: "Cottonelle Fresh Care Flushable Wet Wipes (12 pk)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(403),
            name: "Disposable Facial Mask - adult size (100 ct, Black)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(404),
            name: "Pampers Swaddlers Ultra-Absorbent Baby Diapers - Size 5 (140 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(7400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(405),
            name: "Pampers Fresh Scent Baby Clean Wipes (15 pk/80 wipes)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(406),
            name: "Huggies Snug & Dry Baby Diapers - Size 5 (186 ct)".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(7900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(407),
            name: "Nix Lice Treatment & Prevention Kit".to_string(),
            category: ProductCategory::new("Hygiene"),
            unit_price: Money::from_cents(4000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(408),
            name: "Pendaflex Letter-Size Hanging File Folders (25 pk)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(409),
            name: "Energizer AA Alkaline Batteries (32 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(410),
            name: "Energizer AAA Alkaline Batteries (32 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(411),
            name: "Energizer D Alkaline Batteries (12 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(412),
            name: "Sharpie Assorted Tip Permanent Markers (21 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(413),
            name: "EXPO Marker & Dry Erase Set".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(414),
            name: "Scotch Heavy Duty Shipping Packaging Tape with Dispenser (4 rolls)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(415),
            name: "Post-it Super Sticky Notes 3\" x 3\" (24 ct/70 sheets)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(416),
            name: "Glad ForceFlex Tall Kitchen Trash Bags (120 ct/13 gal)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3950).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(417),
            name: "Swiffer Sweeper Dry and Wet Sweeping Kit".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(418),
            name: "O-Cedar EasyWring Microfiber Spin Mop".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(419),
            name: "Mr. Clean Extra Durable Scrub Magic Eraser Sponges (15 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(420),
            name: "Lysol Disinfectant Spray, Crisp Linen (3 pk/19 oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(421),
            name: "Lysol All-Purpose Cleaner (4 ct/32 oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(422),
            name: "Lysol Advanced Disinfecting Wipes Variety Pack (5 pk/85 sheets)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(423),
            name: "Clorox Disinfecting Wipes Value Pack (5 pk/85 sheets)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3150).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(424),
            name: "Clorox Performance Bleach (3 pk/121 oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(425),
            name: "Clorox Ultra Clean Bleach Toilet Tablets (6 ct/3.5 oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(426),
            name: "Dawn Powerwash Fresh Dish Spray (64.5 fl. oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(427),
            name: "Scotch-Brite Lint Rollers (4 pk/420 sheets)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(428),
            name: "Bounty Select-A-Size Paper Towels (12 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(4650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(429),
            name: "Gain Flings Detergent Pacs - 3in1 Original Scent (4 pk/38 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(4650).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(430),
            name: "Tide POD Laundry Detergent Pacs (4 pk/42 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(5200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(431),
            name: "Gain Laundry Detergent Aroma Boost (2 pk/65 fl. oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(432),
            name: "Gain Original Scent Dryer Sheets (2pk / 160 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(433),
            name: "Snuggle Fabric Softener Dryer Sheet (320 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1850).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(434),
            name: "Downy Ultra Soft Fabric Softener (2 ct/56 oz)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(435),
            name: "Febreze Fabric Extra Strength Spray with Downy Calm (2 pk)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(436),
            name: "Tide Oxi Pens Stain Remover (6 count)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(437),
            name: "Laundry Bag 28\" x 36\", Heavy Duty (2 bags; color may vary)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(438),
            name: "USB-C Phone Charger Block (3 ct/6 feet)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(439),
            name: "Power Strip Surge Protector (2 ct/5 feet)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(440),
            name: "Digital LED Alarm Clock: Basic Degit & USB Charging Port (Outlet & Battery Operated)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(441),
            name: "Revlon Turbo Hair Dryer".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2150).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(442),
            name: "17\" Diamond Patch Backpacks Assorted 8 Colors (24 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(19600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(443),
            name: "17\" Basic Backpack in Black Color (24 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(16000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(444),
            name: "42\" Umbrellas, Windproof and Compact for Teenagers (Black)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(445),
            name: "Fleece Throw Blanket (50\" x 60\")".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(446),
            name: "Drawstring Bag Bulk 10 Colors (100 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(447),
            name: "Multicolor Reusable Grocery Bags (24 pk/13.4\" x 10.2\" x 15\")".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(448),
            name: "Ziploc Storage Bag Variety Pack (347 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2450).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(449),
            name: "Foldable Utility Cart with Swivel Wheels and Double Basket".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(11500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(450),
            name: "Cell Phone Locker (24 Slots, Aluminum)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(9900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(451),
            name: "Waterproof Pencil Pouches - 12 Colors (24 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(452),
            name: "First Aid Kit for 100 person (326 pieces)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(6500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(453),
            name: "Portable Emergency First Aid Kit (80 pieces)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(454),
            name: "Insulated Stainless 17oz Water Bottle - Color May Vary".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(455),
            name: "Portable Handheld Turbo Fan Type-C (Green)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(456),
            name: "Methanol Gel Chafing Dish Fuel 2.5 Hours (72 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(7200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(457),
            name: "BIC Multi-Purpose Lighter Set (4 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(458),
            name: "Heavy Duty Stainless Steel Can Opener".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(459),
            name: "Hot Hand Warmers Heat Pack".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(460),
            name: "Knit Scarf - Assorted Colors (25 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(28000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(461),
            name: "Cuffed Beanies - Assorted Colors (25 ct)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(15000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(462),
            name: "48x Winter Beanies and Gloves Combo (24x beanies + 24x pairs gloves)".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(10000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(463),
            name: "12x Pairs Winter Gloves for Kids 6-12 Years - Assorted Colors".to_string(),
            category: ProductCategory::new("Supplies"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(464),
            name: "Individually Wrapped Sporks (1,000 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(2900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(465),
            name: "4 Piece Cutlery Packet (420 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(466),
            name: "9\" Compostable ECO Plates (500 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(467),
            name: "28oz Rectangular Microwavable Container and Lid (150 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(468),
            name: "Foil Pan Deep Full Size - LIDS SEPARATE (50 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(8400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(469),
            name: "Foil Pan Full Size Lids (50 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(6400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(470),
            name: "Powder Free Vinyl Gloves (1,000 ct) - Small".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(471),
            name: "Powder Free Vinyl Gloves (1,000 ct) - Medium".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(472),
            name: "Powder Free Vinyl Gloves (1,000 ct) - Large".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(473),
            name: "Powder Free Vinyl Gloves (1,000 ct) - Extra Large".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(474),
            name: "1/8 Small Size White Unprinted T-shirt Bag (1,000 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(4200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(475),
            name: "10 oz Hot Cup (1,000 ct)".to_string(),
            category: ProductCategory::new("Packaging"),
            unit_price: Money::from_cents(5500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(476),
            name: "Chess (Wooden, Foldable, Magentic)".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(4800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(477),
            name: "Connect 4".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(478),
            name: "Mancala".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(479),
            name: "Battleship".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(480),
            name: "Catan".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(7000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(481),
            name: "Ticket to Ride".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(7700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(482),
            name: "Sequence".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(2600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(483),
            name: "Checkers".to_string(),
            category: ProductCategory::new("Board Games - Strategy"),
            unit_price: Money::from_cents(2300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(484),
            name: "Candy Land Kingdom of Sweet Adventures".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(485),
            name: "Trouble Game".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(486),
            name: "The Game of Life".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(487),
            name: "Clue".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(488),
            name: "Guess Who?".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(489),
            name: "Guess in 10 - Animal Planet".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(490),
            name: "Sorry!".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(491),
            name: "Monopoly Classic".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(492),
            name: "Dominoes".to_string(),
            category: ProductCategory::new("Board Games - Family"),
            unit_price: Money::from_cents(1200).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(493),
            name: "Upwords".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(2500).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(494),
            name: "Five Crowns - GIANT".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(495),
            name: "Exploding Kittens Original".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(496),
            name: "Taco Cat Goat Cheese Pizza".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(1400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(497),
            name: "Clumsy Thief Math Game".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(3000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(498),
            name: "Scrabble Classic".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(499),
            name: "UNO".to_string(),
            category: ProductCategory::new("Board Games - Word, Number, Cards"),
            unit_price: Money::from_cents(1700).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(500),
            name: "Jenga".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(2300).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(501),
            name: "Don't Break The Ice".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(2400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(502),
            name: "The Sneaky, Snacky Squirrel Game".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(3100).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(503),
            name: "Feed the Woozle".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(504),
            name: "Zingo! Bingo with a Zing".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(3600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(505),
            name: "Spot It! Classic".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(1600).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(506),
            name: "Outfoxed!".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(3400).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(507),
            name: "Twister".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(508),
            name: "Operation".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(509),
            name: "Hungry Hungry Hippos".to_string(),
            category: ProductCategory::new("Board Games - Skill & Action"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(510),
            name: "United States Puzzles for Kids (70 pieces)".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(511),
            name: "World Shaped Jigsaw Puzzles (68 pieces)".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(3800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(512),
            name: "Rotating World Globe (8 inch)".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(4000).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(513),
            name: "Wooden Mosaic Puzzle (370 pieces)".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(2800).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(514),
            name: "Tangram Wooden Puzzle for 60 Challenges".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(1900).unwrap_or_else(|_| Money::zero()),
        },
        Product {
            id: ProductId(515),
            name: "Tangram Jigsaw Wooden Multicolor Puzzle (40 pieces)".to_string(),
            category: ProductCategory::new("Puzzles"),
            unit_price: Money::from_cents(1500).unwrap_or_else(|_| Money::zero()),
        },
    ]
}

pub struct InMemoryCartRepository {
    carts: Arc<Mutex<HashMap<EmployeeId, Cart>>>,
}

impl InMemoryCartRepository {
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CartRepository for InMemoryCartRepository {
    fn get_cart<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Cart>, RepoError>> {
        Box::pin(async move {
            let cart = self.carts.lock().unwrap().get(&employee_id).cloned();
            Ok(cart)
        })
    }

    fn save_cart<'a>(&'a self, cart: Cart) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.carts
                .lock()
                .unwrap()
                .insert(cart.employee_id.clone(), cart);
            Ok(())
        })
    }

    fn clear_cart<'a>(&'a self, employee_id: EmployeeId) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.carts.lock().unwrap().remove(&employee_id);
            Ok(())
        })
    }
}

pub struct InMemorySessionRepository {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl SessionRepository for InMemorySessionRepository {
    fn create_session<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Session, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let token = format!("session-{}", *next_id);
            *next_id += 1;
            let session = Session {
                token: token.clone(),
                employee_id,
            };
            self.sessions
                .lock()
                .unwrap()
                .insert(token.clone(), session.clone());
            Ok(session)
        })
    }

    fn get_session<'a>(
        &'a self,
        token: &'a str,
    ) -> BoxFuture<'a, Result<Option<Session>, RepoError>> {
        let session = self.sessions.lock().unwrap().get(token).cloned();
        Box::pin(async move { Ok(session) })
    }

    fn remove_session<'a>(&'a self, token: &'a str) -> BoxFuture<'a, Result<(), RepoError>> {
        Box::pin(async move {
            self.sessions.lock().unwrap().remove(token);
            Ok(())
        })
    }
}

pub struct InMemoryInvoiceRepository {
    invoices: Arc<Mutex<Vec<Invoice>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryInvoiceRepository {
    pub fn new() -> Self {
        Self {
            invoices: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl InvoiceRepository for InMemoryInvoiceRepository {
    fn create_invoice<'a>(
        &'a self,
        draft: InvoiceDraft,
    ) -> BoxFuture<'a, Result<Invoice, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let invoice = Invoice {
                id: InvoiceId(*next_id),
                employee_id: draft.employee_id,
                items: draft.items,
                subtotal: draft.subtotal,
                fee: draft.fee,
                tax: draft.tax,
                total: draft.total,
            };
            *next_id += 1;
            self.invoices.lock().unwrap().push(invoice.clone());
            Ok(invoice)
        })
    }

    fn get_invoice<'a>(
        &'a self,
        invoice_id: InvoiceId,
    ) -> BoxFuture<'a, Result<Option<Invoice>, RepoError>> {
        let invoice = self
            .invoices
            .lock()
            .unwrap()
            .iter()
            .find(|invoice| invoice.id == invoice_id)
            .cloned();
        Box::pin(async move { Ok(invoice) })
    }
}

pub struct InMemoryQuoteRepository {
    quotes: Arc<Mutex<Vec<QuoteRecord>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryQuoteRepository {
    pub fn new() -> Self {
        Self {
            quotes: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl QuoteRepository for InMemoryQuoteRepository {
    fn create_quote<'a>(
        &'a self,
        draft: QuoteDraft,
    ) -> BoxFuture<'a, Result<QuoteRecord, RepoError>> {
        Box::pin(async move {
            let mut next_id = self.next_id.lock().unwrap();
            let record = QuoteRecord {
                id: QuoteId(*next_id),
                employee_id: draft.employee_id,
                items: draft.items,
                subtotal: draft.subtotal,
                fee: draft.fee,
                tax: draft.tax,
                total: draft.total,
                submitted_at: draft.submitted_at,
            };
            *next_id += 1;
            self.quotes.lock().unwrap().push(record.clone());
            Ok(record)
        })
    }

    fn list_quotes<'a>(
        &'a self,
        employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Vec<QuoteRecord>, RepoError>> {
        Box::pin(async move {
            let quotes = self
                .quotes
                .lock()
                .unwrap()
                .iter()
                .filter(|quote| quote.employee_id == employee_id)
                .cloned()
                .collect();
            Ok(quotes)
        })
    }

    fn get_quote<'a>(
        &'a self,
        employee_id: EmployeeId,
        quote_id: QuoteId,
    ) -> BoxFuture<'a, Result<Option<QuoteRecord>, RepoError>> {
        Box::pin(async move {
            let quote = self
                .quotes
                .lock()
                .unwrap()
                .iter()
                .find(|quote| quote.id == quote_id && quote.employee_id == employee_id)
                .cloned();
            Ok(quote)
        })
    }
}

#[derive(Clone, Debug)]
pub struct SentEmail {
    pub to: String,
    pub invoice_id: InvoiceId,
}

pub struct InMemoryEmailGateway {
    sent: Arc<Mutex<Vec<SentEmail>>>,
}

impl InMemoryEmailGateway {
    pub fn new() -> Self {
        Self {
            sent: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn sent_emails(&self) -> Vec<SentEmail> {
        self.sent.lock().unwrap().clone()
    }
}

impl EmailGateway for InMemoryEmailGateway {
    fn send_invoice<'a>(
        &'a self,
        to: &'a str,
        invoice: &'a Invoice,
    ) -> BoxFuture<'a, Result<(), EmailError>> {
        Box::pin(async move {
            self.sent.lock().unwrap().push(SentEmail {
                to: to.to_string(),
                invoice_id: invoice.id.clone(),
            });
            Ok(())
        })
    }
}
