use crate::view_models::{GreetingListItem, GreetingViewModel};
use core_ports::output_boundary::{GreetingOutput, GreetingOutputBoundary};
use std::sync::Mutex;

pub struct GreetingPresenter {
    view_model: Mutex<Option<GreetingViewModel>>,
}

impl GreetingPresenter {
    pub fn new() -> Self {
        Self {
            view_model: Mutex::new(None),
        }
    }

    pub fn take_view_model(&self) -> GreetingViewModel {
        self.view_model
            .lock()
            .unwrap()
            .take()
            .unwrap_or_else(GreetingViewModel::empty)
    }
}

impl GreetingOutputBoundary for GreetingPresenter {
    fn present(&self, output: GreetingOutput) {
        let greetings = output
            .greetings
            .into_iter()
            .map(|greeting| GreetingListItem {
                id: greeting.id.0,
                message: greeting.message,
            })
            .collect();

        let view_model = GreetingViewModel {
            title: "Greetings".to_string(),
            greetings,
            empty_message: "No greetings yet.".to_string(),
        };

        *self.view_model.lock().unwrap() = Some(view_model);
    }
}
