use core_entities::Greeting;

#[derive(Clone, Debug)]
pub struct GreetingOutput {
    pub greetings: Vec<Greeting>,
}

pub trait GreetingOutputBoundary: Send + Sync {
    fn present(&self, output: GreetingOutput);
}
