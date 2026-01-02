use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use core_entities::ordering::Employee;
use core_ports::ordering::{EmployeeRepository, SessionStore};
use core_ports::RepoError;
use core_use_cases::ordering::{SignInEmployeeInput, SignInEmployeeInteractor};

struct InMemoryEmployeeRepo {
    employees: RefCell<HashMap<String, Employee>>,
}

impl InMemoryEmployeeRepo {
    fn new() -> Self {
        Self {
            employees: RefCell::new(HashMap::new()),
        }
    }

    fn insert(&self, employee: Employee) {
        self.employees
            .borrow_mut()
            .insert(employee.email.clone(), employee);
    }
}

impl EmployeeRepository for InMemoryEmployeeRepo {
    fn insert(&self, employee: Employee) -> Result<(), RepoError> {
        self.employees
            .borrow_mut()
            .insert(employee.email.clone(), employee);
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<Employee>, RepoError> {
        Ok(self.employees.borrow().get(email).cloned())
    }
}

struct InMemorySessionStore {
    active: RefCell<HashSet<String>>,
}

impl InMemorySessionStore {
    fn new() -> Self {
        Self {
            active: RefCell::new(HashSet::new()),
        }
    }

    fn is_active(&self, email: &str) -> bool {
        self.active.borrow().contains(email)
    }
}

impl SessionStore for InMemorySessionStore {
    fn start_session(&self, email: &str) -> Result<(), RepoError> {
        self.active.borrow_mut().insert(email.to_string());
        Ok(())
    }

    fn end_session(&self, email: &str) -> Result<(), RepoError> {
        self.active.borrow_mut().remove(email);
        Ok(())
    }

    fn is_active(&self, email: &str) -> Result<bool, RepoError> {
        Ok(self.active.borrow().contains(email))
    }
}

#[test]
fn sign_in_establishes_session() {
    let employees = InMemoryEmployeeRepo::new();
    let sessions = InMemorySessionStore::new();
    let employee = Employee::fixture(
        "QWilliams@schools.nyc.gov",
        "P.S. 082 - The Hammond School",
        "School Hours",
    );
    employees.insert(employee);

    let interactor = SignInEmployeeInteractor::new(&employees, &sessions);
    let result = interactor
        .execute(SignInEmployeeInput::new(
            "QWilliams@schools.nyc.gov",
            "CorrectHorseBatteryStaple",
        ))
        .expect("sign in should succeed");

    assert_eq!(result.email.as_str(), "QWilliams@schools.nyc.gov");
    assert!(sessions.is_active("QWilliams@schools.nyc.gov"));
}
