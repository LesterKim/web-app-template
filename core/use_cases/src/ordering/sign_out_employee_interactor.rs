use core_ports::ordering::SessionStore;

use super::errors::AuthError;

pub struct SignOutEmployeeInteractor<'a> {
    sessions: &'a dyn SessionStore,
}

impl<'a> SignOutEmployeeInteractor<'a> {
    pub fn new(sessions: &'a dyn SessionStore) -> Self {
        Self { sessions }
    }

    pub fn execute(&self, input: SignOutEmployeeInput) -> Result<(), AuthError> {
        self.sessions
            .end_session(&input.email)
            .map_err(|err| AuthError::new(err.message))?;
        Ok(())
    }
}

pub struct SignOutEmployeeInput {
    pub email: String,
}

impl SignOutEmployeeInput {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}
