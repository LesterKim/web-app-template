use core_ports::ordering::{EmployeeRepository, SessionStore};

use super::errors::AuthError;

pub struct SignInEmployeeInteractor<'a> {
    employees: &'a dyn EmployeeRepository,
    sessions: &'a dyn SessionStore,
}

impl<'a> SignInEmployeeInteractor<'a> {
    pub fn new(
        employees: &'a dyn EmployeeRepository,
        sessions: &'a dyn SessionStore,
    ) -> Self {
        Self { employees, sessions }
    }

    pub fn execute(&self, input: SignInEmployeeInput) -> Result<SignInEmployeeOutput, AuthError> {
        let employee = self
            .employees
            .find_by_email(&input.email)
            .map_err(|err| AuthError::new(err.message))?
            .ok_or_else(|| AuthError::new("employee account not found"))?;

        if employee.password != input.password {
            return Err(AuthError::new("invalid credentials"));
        }

        self.sessions
            .start_session(&input.email)
            .map_err(|err| AuthError::new(err.message))?;

        Ok(SignInEmployeeOutput {
            email: input.email,
        })
    }
}

pub struct SignInEmployeeInput {
    pub email: String,
    pub password: String,
}

impl SignInEmployeeInput {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct SignInEmployeeOutput {
    pub email: String,
}
