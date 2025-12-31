use core_entities::{
    Cart, Employee, EmployeeId, Invoice, InvoiceDraft, InvoiceId, Product, ProductId, QuoteDraft,
    QuoteId, QuoteRecord, Session,
};
use core_ports::{
    BoxFuture, CartRepository, CatalogRepository, EmailError, EmailGateway, EmployeeRepository,
    InvoiceRepository, QuoteRepository, RepoError, SessionRepository,
};

pub struct PostgresStore {
    connection_string: String,
}

impl PostgresStore {
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
        }
    }

    fn not_configured(&self) -> RepoError {
        RepoError::new(format!(
            "postgres adapter not configured (connection_string: {})",
            self.connection_string
        ))
    }
}

impl EmployeeRepository for PostgresStore {
    fn add_employee<'a>(
        &'a self,
        _name: String,
        _email: String,
        _password: String,
    ) -> BoxFuture<'a, Result<Employee, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn find_by_email<'a>(
        &'a self,
        _email: &'a str,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn get_by_id<'a>(
        &'a self,
        _employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Employee>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl CatalogRepository for PostgresStore {
    fn list_products<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Product>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn find_product<'a>(
        &'a self,
        _product_id: ProductId,
    ) -> BoxFuture<'a, Result<Option<Product>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl CartRepository for PostgresStore {
    fn get_cart<'a>(
        &'a self,
        _employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Option<Cart>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn save_cart<'a>(&'a self, _cart: Cart) -> BoxFuture<'a, Result<(), RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn clear_cart<'a>(&'a self, _employee_id: EmployeeId) -> BoxFuture<'a, Result<(), RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl SessionRepository for PostgresStore {
    fn create_session<'a>(
        &'a self,
        _employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Session, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn get_session<'a>(
        &'a self,
        _token: &'a str,
    ) -> BoxFuture<'a, Result<Option<Session>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn remove_session<'a>(&'a self, _token: &'a str) -> BoxFuture<'a, Result<(), RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl InvoiceRepository for PostgresStore {
    fn create_invoice<'a>(
        &'a self,
        _draft: InvoiceDraft,
    ) -> BoxFuture<'a, Result<Invoice, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn get_invoice<'a>(
        &'a self,
        _invoice_id: InvoiceId,
    ) -> BoxFuture<'a, Result<Option<Invoice>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl QuoteRepository for PostgresStore {
    fn create_quote<'a>(
        &'a self,
        _draft: QuoteDraft,
    ) -> BoxFuture<'a, Result<QuoteRecord, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn list_quotes<'a>(
        &'a self,
        _employee_id: EmployeeId,
    ) -> BoxFuture<'a, Result<Vec<QuoteRecord>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }

    fn get_quote<'a>(
        &'a self,
        _employee_id: EmployeeId,
        _quote_id: QuoteId,
    ) -> BoxFuture<'a, Result<Option<QuoteRecord>, RepoError>> {
        let err = self.not_configured();
        Box::pin(async move { Err(err) })
    }
}

impl EmailGateway for PostgresStore {
    fn send_invoice<'a>(
        &'a self,
        _to: &'a str,
        _invoice: &'a Invoice,
    ) -> BoxFuture<'a, Result<(), EmailError>> {
        let message = format!(
            "email gateway not configured (connection_string: {})",
            self.connection_string
        );
        let err = EmailError::new(message);
        Box::pin(async move { Err(err) })
    }
}
