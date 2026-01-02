pub mod adapters;
pub mod presenters;
pub mod routes;
pub mod view_models;

use core_entities::ordering::{CalendarDate, School};

use adapters::{
    FixedClock, FixedRateProvider, InMemoryCartRepository, InMemoryCatalogRepository,
    InMemoryEmailOutbox, InMemoryEmployeeRepository, InMemoryInvoiceRepository,
    InMemoryQuoteRepository, InMemorySchoolRepository, InMemorySessionStore, StubInvoiceRenderer,
};

const DEFAULT_SCHOOL_NAME: &str = "P.S. 082 - The Hammond School";
const DEFAULT_SCHOOL_CODE: &str = "28Q082";
const DEFAULT_TAX_RATE: f64 = 0.08875;
const DEFAULT_SHIPPING_RATE: f64 = 0.01;
const DEFAULT_TODAY: &str = "12/22/25";

#[derive(Clone, Debug)]
pub struct CatalogItem {
    pub description: String,
    pub price: f64,
}

#[derive(Clone, Debug)]
pub struct SchoolOption {
    pub name: String,
    pub code: String,
}

#[derive(Clone)]
pub struct OrderingState {
    pub schools: InMemorySchoolRepository,
    pub employees: InMemoryEmployeeRepository,
    pub sessions: InMemorySessionStore,
    pub catalog: InMemoryCatalogRepository,
    pub carts: InMemoryCartRepository,
    pub quotes: InMemoryQuoteRepository,
    pub invoices: InMemoryInvoiceRepository,
    pub email_outbox: InMemoryEmailOutbox,
    pub invoice_renderer: StubInvoiceRenderer,
    pub clock: FixedClock,
    pub rates: FixedRateProvider,
    pub catalog_items: Vec<CatalogItem>,
    pub school_options: Vec<SchoolOption>,
}

impl OrderingState {
    pub fn new() -> Self {
        let schools = InMemorySchoolRepository::new();
        let employees = InMemoryEmployeeRepository::new();
        let sessions = InMemorySessionStore::new();
        let catalog = InMemoryCatalogRepository::new();
        let carts = InMemoryCartRepository::new();
        let quotes = InMemoryQuoteRepository::new();
        let invoices = InMemoryInvoiceRepository::new();
        let email_outbox = InMemoryEmailOutbox::new();
        let invoice_renderer = StubInvoiceRenderer::new();
        let clock = FixedClock::new(CalendarDate::parse_mmddyy(DEFAULT_TODAY));
        let rates = FixedRateProvider::new(DEFAULT_TAX_RATE, DEFAULT_SHIPPING_RATE);

        let school_options = vec![SchoolOption {
            name: DEFAULT_SCHOOL_NAME.to_string(),
            code: DEFAULT_SCHOOL_CODE.to_string(),
        }];
        for school in &school_options {
            schools.seed(School {
                name: school.name.clone(),
                code: Some(school.code.clone()),
            });
        }

        let catalog_items = vec![
            CatalogItem {
                description: "Poland Spring Water (48 ct/8 oz)".to_string(),
                price: 20.00,
            },
            CatalogItem {
                description: "Copy Paper (10 reams)".to_string(),
                price: 42.50,
            },
            CatalogItem {
                description: "Disinfecting Wipes (320 ct)".to_string(),
                price: 15.75,
            },
        ];
        for item in &catalog_items {
            catalog.seed_item(&item.description, item.price);
        }

        Self {
            schools,
            employees,
            sessions,
            catalog,
            carts,
            quotes,
            invoices,
            email_outbox,
            invoice_renderer,
            clock,
            rates,
            catalog_items,
            school_options,
        }
    }
}
