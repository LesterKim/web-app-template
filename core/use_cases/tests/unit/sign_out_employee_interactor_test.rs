use core_ports::ordering::SessionStore;
use core_ports::RepoError;
use core_use_cases::ordering::{SignOutEmployeeInput, SignOutEmployeeInteractor};
use std::cell::RefCell;
use std::collections::HashSet;

struct InMemorySessionStore {
    active: RefCell<HashSet<String>>,
}

impl InMemorySessionStore {
    fn new() -> Self {
        Self {
            active: RefCell::new(HashSet::new()),
        }
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
fn sign_out_terminates_session() {
    let sessions = InMemorySessionStore::new();
    sessions
        .start_session("QWilliams@schools.nyc.gov")
        .expect("session should start");

    let interactor = SignOutEmployeeInteractor::new(&sessions);
    interactor
        .execute(SignOutEmployeeInput::new("QWilliams@schools.nyc.gov"))
        .expect("sign out should succeed");

    assert!(
        !sessions
            .is_active("QWilliams@schools.nyc.gov")
            .expect("session should be queryable")
    );
}
