#[derive(Clone, Debug)]
pub struct SignUpFormViewModel {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub school: String,
    pub phone: String,
    pub delivery_window: String,
}

impl SignUpFormViewModel {
    pub fn empty() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            title: String::new(),
            school: String::new(),
            phone: String::new(),
            delivery_window: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SignUpViewModel {
    pub title: String,
    pub subtitle: String,
    pub form: SignUpFormViewModel,
    pub schools: Vec<SchoolOptionViewModel>,
    pub error: Option<String>,
    pub success: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SignUpSuccessViewModel {
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct SignInFormViewModel {
    pub email: String,
    pub password: String,
}

impl SignInFormViewModel {
    pub fn empty() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SignInViewModel {
    pub title: String,
    pub subtitle: String,
    pub form: SignInFormViewModel,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SessionViewModel {
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct SchoolOptionViewModel {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct CatalogItemViewModel {
    pub description: String,
    pub price: String,
}

#[derive(Clone, Debug)]
pub struct CartItemViewModel {
    pub description: String,
    pub quantity: u32,
    pub price: String,
    pub line_total: String,
}

#[derive(Clone, Debug)]
pub struct CartViewModel {
    pub title: String,
    pub employee_email: String,
    pub catalog_items: Vec<CatalogItemViewModel>,
    pub cart_items: Vec<CartItemViewModel>,
    pub cart_error: Option<String>,
    pub quote: Option<QuoteViewModel>,
    pub quote_error: Option<String>,
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CartItemsViewModel {
    pub cart_items: Vec<CartItemViewModel>,
    pub cart_error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct QuotePreviewViewModel {
    pub quote: Option<QuoteViewModel>,
    pub quote_error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct QuoteViewModel {
    pub number: String,
    pub line_items: Vec<QuoteLineItemViewModel>,
    pub subtotal: String,
    pub tax: String,
    pub shipping: String,
    pub total: String,
    pub delivery_window: String,
}

#[derive(Clone, Debug)]
pub struct QuoteLineItemViewModel {
    pub description: String,
    pub quantity: u32,
    pub price: String,
    pub line_total: String,
}

#[derive(Clone, Debug)]
pub struct QuotePageViewModel {
    pub title: String,
    pub employee_email: String,
    pub quote: Option<QuoteViewModel>,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct InvoiceViewModel {
    pub quote_number: String,
    pub recipient: String,
    pub school: String,
    pub delivery_window: String,
    pub line_items: Vec<InvoiceLineItemViewModel>,
    pub subtotal: String,
    pub tax: String,
    pub shipping: String,
    pub total: String,
}

#[derive(Clone, Debug)]
pub struct InvoiceLineItemViewModel {
    pub description: String,
    pub quantity: u32,
    pub price: String,
    pub line_total: String,
}

#[derive(Clone, Debug)]
pub struct InvoicePageViewModel {
    pub title: String,
    pub employee_email: String,
    pub invoice: Option<InvoiceViewModel>,
    pub message: Option<String>,
    pub error: Option<String>,
}
