use core_ports::output_boundary::GreetingOutput;
use core_ports::output_boundary::GreetingOutputBoundary;
use core_ports::{GreetingRepository, RepoError};

#[derive(Debug)]
pub enum UseCaseError {
    Repo(RepoError),
}

impl std::fmt::Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseCaseError::Repo(err) => write!(f, "repository error: {}", err.message),
        }
    }
}

impl std::error::Error for UseCaseError {}

pub struct ListGreetingsInteractor<'a> {
    repo: &'a dyn GreetingRepository,
    presenter: &'a dyn GreetingOutputBoundary,
}

pub mod ordering;

impl<'a> ListGreetingsInteractor<'a> {
    pub fn new(
        repo: &'a dyn GreetingRepository,
        presenter: &'a dyn GreetingOutputBoundary,
    ) -> Self {
        Self { repo, presenter }
    }

    pub async fn execute(&self) -> Result<(), UseCaseError> {
        let greetings = self
            .repo
            .list_greetings()
            .await
            .map_err(UseCaseError::Repo)?;
        let output = GreetingOutput { greetings };
        self.presenter.present(output);
        Ok(())
    }
}
