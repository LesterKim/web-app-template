use crate::atdd::dsl::{ScenarioLog, TestDsl};
use core_use_cases::ordering::SignUpEmployeeInput;

#[test]
fn employee_signs_up_with_school_email() {
    let _log = ScenarioLog::start(
        "Employee signs up with a school email",
        "creates an employee account tied to the school",
    );
    let mut app = TestDsl::new();
    app.given_school("P.S. 082 - The Hammond School", "28Q082");
    app.given_new_employee(SignUpEmployeeInput::new(
        "QWilliams@schools.nyc.gov",
        "CorrectHorseBatteryStaple",
        "Quanisha",
        "Williams",
        "Coordinator",
        "P.S. 082 - The Hammond School",
        "(718) 526-4139 Ext. 2131",
        Some("School Hours"),
    ));
    app.when_sign_up();
    app.then_account_exists("QWilliams@schools.nyc.gov");
    app.then_account_associated_with_school(
        "QWilliams@schools.nyc.gov",
        "P.S. 082 - The Hammond School",
    );
}

#[test]
fn sign_up_rejects_non_school_emails() {
    let _log = ScenarioLog::start(
        "Sign up rejects non-school emails",
        "enforces schools.nyc.gov email addresses",
    );
    let mut app = TestDsl::new();
    app.given_new_employee(SignUpEmployeeInput::new(
        "someone@gmail.com",
        "AnyPassword123!",
        "Sam",
        "Taylor",
        "Teacher",
        "P.S. 082 - The Hammond School",
        "(212) 555-0000",
        Some("School Hours"),
    ));
    app.when_sign_up();
    app.then_sign_up_rejected();
    app.then_account_does_not_exist("someone@gmail.com");
}

#[test]
fn sign_up_requires_all_required_fields() {
    let _log = ScenarioLog::start(
        "Sign up requires all required fields",
        "rejects missing delivery window values",
    );
    let mut app = TestDsl::new();
    app.given_new_employee(SignUpEmployeeInput::new(
        "QWilliams@schools.nyc.gov",
        "CorrectHorseBatteryStaple",
        "Quanisha",
        "Williams",
        "Coordinator",
        "P.S. 082 - The Hammond School",
        "(718) 526-4139 Ext. 2131",
        None,
    ));
    app.when_sign_up();
    app.then_sign_up_rejected();
}

#[test]
fn sign_up_rejects_short_passwords() {
    let _log = ScenarioLog::start(
        "Sign up rejects short passwords",
        "requires passwords longer than 16 characters",
    );
    let mut app = TestDsl::new();
    app.given_new_employee(SignUpEmployeeInput::new(
        "QWilliams@schools.nyc.gov",
        "ShortPassword16",
        "Quanisha",
        "Williams",
        "Coordinator",
        "P.S. 082 - The Hammond School",
        "(718) 526-4139 Ext. 2131",
        Some("School Hours"),
    ));
    app.when_sign_up();
    app.then_sign_up_rejected();
}

#[test]
fn employee_signs_in() {
    let _log = ScenarioLog::start(
        "Employee signs in",
        "establishes an authenticated session",
    );
    let mut app = TestDsl::new();
    app.given_employee_account("QWilliams@schools.nyc.gov", "CorrectHorseBatteryStaple");
    app.when_sign_in("QWilliams@schools.nyc.gov", "CorrectHorseBatteryStaple");
    app.then_session_active("QWilliams@schools.nyc.gov");
}

#[test]
fn employee_signs_out() {
    let _log = ScenarioLog::start(
        "Employee signs out",
        "terminates the authenticated session",
    );
    let mut app = TestDsl::new();
    app.given_authenticated_session("QWilliams@schools.nyc.gov");
    app.when_sign_out();
    app.then_session_terminated("QWilliams@schools.nyc.gov");
}
