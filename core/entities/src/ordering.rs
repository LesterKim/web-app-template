#[derive(Clone, Debug, PartialEq)]
pub struct School {
    pub name: String,
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub school_name: String,
    pub phone: String,
    pub delivery_window: String,
}

impl Employee {
    pub fn new(
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
        title: &str,
        school_name: &str,
        phone: &str,
        delivery_window: &str,
    ) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            title: title.to_string(),
            school_name: school_name.to_string(),
            phone: phone.to_string(),
            delivery_window: delivery_window.to_string(),
        }
    }

    pub fn fixture(email: &str, school_name: &str, delivery_window: &str) -> Self {
        Self::new(
            email,
            "CorrectHorseBatteryStaple",
            "Quanisha",
            "Williams",
            "Coordinator",
            school_name,
            "(718) 526-4139 Ext. 2131",
            delivery_window,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CartLineItem {
    pub description: String,
    pub quantity: u32,
    pub price: f64,
}

impl CartLineItem {
    pub fn new(description: &str, quantity: u32, price: f64) -> Self {
        Self {
            description: description.to_string(),
            quantity,
            price,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quote {
    pub number: String,
    pub line_items: Vec<QuoteLineItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub shipping: f64,
    pub total: f64,
    pub delivery_window: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuoteLineItem {
    pub description: String,
    pub quantity: u32,
    pub price: f64,
    pub line_total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Invoice {
    pub quote_number: String,
    pub line_items: Vec<InvoiceLineItem>,
    pub tax: Option<f64>,
    pub shipping: Option<f64>,
    pub delivery_window: String,
    pub school: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvoiceLineItem {
    pub description: String,
    pub quantity: u32,
    pub price: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalendarDate {
    pub month: u32,
    pub day: u32,
    pub year_two_digits: u32,
}

impl CalendarDate {
    pub fn parse_mmddyy(value: &str) -> Self {
        let parts: Vec<&str> = value.split('/').collect();
        assert_eq!(parts.len(), 3, "date should be in MM/DD/YY format");
        let month = parts[0].parse().expect("month should be numeric");
        let day = parts[1].parse().expect("day should be numeric");
        let year_two_digits = parts[2].parse().expect("year should be numeric");
        Self {
            month,
            day,
            year_two_digits,
        }
    }
}
