use core_entities::Greeting;
use core_ports::{BoxFuture, GreetingRepository, RepoError};

pub struct PostgresGreetingRepository {
    connection_string: String,
}

impl PostgresGreetingRepository {
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
        }
    }
}

impl GreetingRepository for PostgresGreetingRepository {
    fn list_greetings<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Greeting>, RepoError>> {
        let message = format!(
            "postgres adapter not configured (connection_string: {})",
            self.connection_string
        );
        Box::pin(async move { Err(RepoError::new(message)) })
    }
}
