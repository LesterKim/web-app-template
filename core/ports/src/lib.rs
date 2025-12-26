use core_entities::Greeting;
use std::future::Future;
use std::pin::Pin;

pub mod output_boundary;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug)]
pub struct RepoError {
    pub message: String,
}

impl RepoError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait GreetingRepository: Send + Sync {
    fn list_greetings<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Greeting>, RepoError>>;
}
