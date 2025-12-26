pub mod postgres;

use core_entities::Greeting;
use core_ports::{BoxFuture, GreetingRepository, RepoError};
use std::sync::{Arc, Mutex};

pub struct MemoryGreetingRepository {
    greetings: Arc<Mutex<Vec<Greeting>>>,
}

impl MemoryGreetingRepository {
    pub fn new(initial: Vec<Greeting>) -> Self {
        Self {
            greetings: Arc::new(Mutex::new(initial)),
        }
    }
}

impl GreetingRepository for MemoryGreetingRepository {
    fn list_greetings<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Greeting>, RepoError>> {
        let greetings = self.greetings.clone();
        Box::pin(async move {
            let data = greetings.lock().unwrap().clone();
            Ok(data)
        })
    }
}
