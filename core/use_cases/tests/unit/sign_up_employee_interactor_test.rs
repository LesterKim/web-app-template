use std::cell::RefCell;
use std::collections::HashMap;

use core_entities::ordering::{Employee, School};
use core_ports::ordering::{EmployeeRepository, SchoolRepository};
use core_ports::RepoError;
use core_use_cases::ordering::{SignUpEmployeeInput, SignUpEmployeeInteractor};

struct InMemoryEmployeeRepo {
    employees: RefCell<HashMap<String, Employee>>,
}

impl InMemoryEmployeeRepo {
    fn new() -> Self {
        Self {
            employees: RefCell::new(HashMap::new()),
        }
    }

    fn exists(&self, email: &str) -> bool {
        self.employees.borrow().contains_key(email)
    }

    fn get(&self, email: &str) -> Option<Employee> {
        self.employees.borrow().get(email).cloned()
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

struct InMemorySchoolRepo {
    schools: RefCell<HashMap<String, School>>,
}

impl InMemorySchoolRepo {
    fn new() -> Self {
        Self {
            schools: RefCell::new(HashMap::new()),
        }
    }

    fn insert(&self, school: School) {
        self.schools
            .borrow_mut()
            .insert(school.name.clone(), school);
    }
}

impl SchoolRepository for InMemorySchoolRepo {
    fn upsert(&self, school: School) -> Result<(), RepoError> {
        self.schools
            .borrow_mut()
            .insert(school.name.clone(), school);
        Ok(())
    }

    fn find_by_name(&self, name: &str) -> Result<Option<School>, RepoError> {
        Ok(self.schools.borrow().get(name).cloned())
    }
}

#[test]
fn sign_up_accepts_school_email_and_persists_employee() {
    let employees = InMemoryEmployeeRepo::new();
    let schools = InMemorySchoolRepo::new();
    schools.insert(School {
        name: "P.S. 082 - The Hammond School".to_string(),
        code: Some("28Q082".to_string()),
    });
    let interactor = SignUpEmployeeInteractor::new(&employees, &schools);

    let result = interactor
        .execute(SignUpEmployeeInput::new(
            "QWilliams@schools.nyc.gov",
            "CorrectHorseBatteryStaple",
            "Quanisha",
            "Williams",
            "Coordinator",
            "P.S. 082 - The Hammond School",
            "(718) 526-4139 Ext. 2131",
            Some("School Hours"),
        ))
        .expect("sign up should succeed");

    assert_eq!(result.employee_email.as_str(), "QWilliams@schools.nyc.gov");
    assert!(employees.exists("QWilliams@schools.nyc.gov"));
    let employee = employees
        .get("QWilliams@schools.nyc.gov")
        .expect("employee should exist");
    assert_eq!(
        employee.school_name.as_str(),
        "P.S. 082 - The Hammond School"
    );
}

#[test]
fn sign_up_rejects_non_school_email_addresses() {
    let employees = InMemoryEmployeeRepo::new();
    let schools = InMemorySchoolRepo::new();
    let interactor = SignUpEmployeeInteractor::new(&employees, &schools);

    let result = interactor.execute(SignUpEmployeeInput::new(
        "someone@gmail.com",
        "AnyPassword123!",
        "Sam",
        "Taylor",
        "Teacher",
        "P.S. 082 - The Hammond School",
        "(212) 555-0000",
        Some("School Hours"),
    ));

    assert!(result.is_err(), "expected sign up to be rejected");
    assert!(!employees.exists("someone@gmail.com"));
}

#[test]
fn sign_up_requires_delivery_window() {
    let employees = InMemoryEmployeeRepo::new();
    let schools = InMemorySchoolRepo::new();
    let interactor = SignUpEmployeeInteractor::new(&employees, &schools);

    let result = interactor.execute(SignUpEmployeeInput::new(
        "QWilliams@schools.nyc.gov",
        "CorrectHorseBatteryStaple",
        "Quanisha",
        "Williams",
        "Coordinator",
        "P.S. 082 - The Hammond School",
        "(718) 526-4139 Ext. 2131",
        None,
    ));

    assert!(result.is_err(), "expected sign up to be rejected");
}

#[test]
fn sign_up_rejects_short_passwords() {
    let employees = InMemoryEmployeeRepo::new();
    let schools = InMemorySchoolRepo::new();
    let interactor = SignUpEmployeeInteractor::new(&employees, &schools);

    let result = interactor.execute(SignUpEmployeeInput::new(
        "QWilliams@schools.nyc.gov",
        "ShortPassword16",
        "Quanisha",
        "Williams",
        "Coordinator",
        "P.S. 082 - The Hammond School",
        "(718) 526-4139 Ext. 2131",
        Some("School Hours"),
    ));

    assert!(result.is_err(), "expected sign up to be rejected");
}
