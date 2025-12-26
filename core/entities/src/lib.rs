#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreetingId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Greeting {
    pub id: GreetingId,
    pub message: String,
}

impl Greeting {
    pub fn new(id: u64, message: impl Into<String>) -> Self {
        Self {
            id: GreetingId(id),
            message: message.into(),
        }
    }
}
