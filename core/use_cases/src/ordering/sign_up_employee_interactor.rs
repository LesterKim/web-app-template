use core_entities::ordering::Employee;
use core_ports::ordering::{EmployeeRepository, SchoolRepository};

use super::errors::SignUpError;

pub struct SignUpEmployeeInteractor<'a> {
    employees: &'a dyn EmployeeRepository,
    schools: &'a dyn SchoolRepository,
}

impl<'a> SignUpEmployeeInteractor<'a> {
    pub fn new(
        employees: &'a dyn EmployeeRepository,
        schools: &'a dyn SchoolRepository,
    ) -> Self {
        Self { employees, schools }
    }

    pub fn execute(&self, input: SignUpEmployeeInput) -> Result<SignUpEmployeeOutput, SignUpError> {
        let email = input.email.trim();
        if email.is_empty() || !is_school_email(email) {
            return Err(SignUpError::new("email must be a schools.nyc.gov address"));
        }

        if input.password.len() <= 16 {
            return Err(SignUpError::new("password must be longer than 16 characters"));
        }

        if is_blank(&input.first_name)
            || is_blank(&input.last_name)
            || is_blank(&input.title)
            || is_blank(&input.school)
            || is_blank(&input.phone)
        {
            return Err(SignUpError::new("all required fields must be provided"));
        }

        let delivery_window = match input.delivery_window.as_deref() {
            Some(value) if !is_blank(value) => value.to_string(),
            _ => {
                return Err(SignUpError::new("delivery window is required"));
            }
        };

        self.schools
            .find_by_name(&input.school)
            .map_err(|err| SignUpError::new(err.message))?;
        let employee = Employee::new(
            email,
            &input.password,
            &input.first_name,
            &input.last_name,
            &input.title,
            &input.school,
            &input.phone,
            &delivery_window,
        );
        self.employees
            .insert(employee)
            .map_err(|err| SignUpError::new(err.message))?;

        Ok(SignUpEmployeeOutput {
            employee_email: email.to_string(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct SignUpEmployeeInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub school: String,
    pub phone: String,
    pub delivery_window: Option<String>,
}

impl SignUpEmployeeInput {
    pub fn new(
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
        title: &str,
        school: &str,
        phone: &str,
        delivery_window: Option<&str>,
    ) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            title: title.to_string(),
            school: school.to_string(),
            phone: phone.to_string(),
            delivery_window: delivery_window.map(str::to_string),
        }
    }
}

pub struct SignUpEmployeeOutput {
    pub employee_email: String,
}

fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

fn is_school_email(email: &str) -> bool {
    let domain = email
        .split('@')
        .nth(1)
        .unwrap_or_default()
        .to_lowercase();
    domain == "schools.nyc.gov"
}
