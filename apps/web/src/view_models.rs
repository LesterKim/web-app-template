#[derive(Clone, Debug)]
pub struct GreetingViewModel {
    pub title: String,
    pub greetings: Vec<GreetingListItem>,
    pub empty_message: String,
}

impl GreetingViewModel {
    pub fn empty() -> Self {
        Self {
            title: "Greetings".to_string(),
            greetings: Vec::new(),
            empty_message: "No greetings yet.".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GreetingListItem {
    pub id: u64,
    pub message: String,
}
