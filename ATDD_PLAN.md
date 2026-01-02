# ATDD Plan: NYC DOE Ordering App (Business Logic Only)

## Goal
Build the core business logic for a system where NYC public school employees can sign up with school emails, authenticate, manage carts, view quotes, and submit quotes that generate invoices delivered by email.

## Scope (Explicit)
- Sign up with required fields: email, password, first name, last name, title, school, phone, delivery window.
- Only `schools.nyc.gov` email addresses are allowed.
- Passwords must be longer than 16 characters.
- Sign in and sign out.
- Add items to cart.
- View quote for cart contents.
- Submit quote: create invoice, deliver invoice via email, and clear cart.
- Quote number format: `{school_code}xMMDDYY`.
- Invoices include tax and shipping fields.
- Invoice emails include a structured body and a PDF attachment.
- Tax and shipping are fixed percentage rates applied to the subtotal.

## ATDD Workflow (Test-First Order)
1. Write the first failing acceptance test for a happy-path sign up.
2. Implement the minimal DSL and in-memory protocol drivers to express the scenario.
3. Implement the use case interactor(s) to pass the test.
4. Repeat for sign in/out, cart, quote, and submit flow.
5. Add acceptance tests for explicitly specified constraints (email domain, required fields, cart cleared).
6. Add unit tests only where deeper business rules need isolation.

## Executable Specifications (Acceptance Tests)

```gherkin
Feature: Employee sign up and authentication

Scenario: Employee signs up with a school email
  Given a school "P.S. 082 - The Hammond School" with code "28Q082"
  And a new employee with:
    | email           | QWilliams@schools.nyc.gov |
    | password        | CorrectHorseBatteryStaple |
    | first_name      | Quanisha                 |
    | last_name       | Williams                 |
    | title           | Coordinator              |
    | school          | P.S. 082 - The Hammond School |
    | phone           | (718) 526-4139 Ext. 2131 |
    | delivery_window | School Hours             |
  When they sign up
  Then an employee account exists for "QWilliams@schools.nyc.gov"
  And the account is associated with "P.S. 082 - The Hammond School"

Scenario: Sign up rejects non-school emails
  Given a new employee with:
    | email           | someone@gmail.com |
    | password        | AnyPassword123!   |
    | first_name      | Sam              |
    | last_name       | Taylor           |
    | title           | Teacher          |
    | school          | P.S. 082 - The Hammond School |
    | phone           | (212) 555-0000   |
    | delivery_window | School Hours     |
  When they sign up
  Then the sign up is rejected
  And no account exists for "someone@gmail.com"

Scenario: Sign up requires all required fields
  Given a new employee with:
    | email           | QWilliams@schools.nyc.gov |
    | password        | CorrectHorseBatteryStaple |
    | first_name      | Quanisha                 |
    | last_name       | Williams                 |
    | title           | Coordinator              |
    | school          | P.S. 082 - The Hammond School |
    | phone           | (718) 526-4139 Ext. 2131 |
    | delivery_window |                        |
  When they sign up
  Then the sign up is rejected

Scenario: Sign up rejects short passwords
  Given a new employee with:
    | email           | QWilliams@schools.nyc.gov |
    | password        | ShortPassword16 |
    | first_name      | Quanisha        |
    | last_name       | Williams        |
    | title           | Coordinator     |
    | school          | P.S. 082 - The Hammond School |
    | phone           | (718) 526-4139 Ext. 2131 |
    | delivery_window | School Hours    |
  When they sign up
  Then the sign up is rejected

Scenario: Employee signs in
  Given an employee account for "QWilliams@schools.nyc.gov" with password "CorrectHorseBatteryStaple"
  When they sign in with email "QWilliams@schools.nyc.gov" and password "CorrectHorseBatteryStaple"
  Then an authenticated session is established for "QWilliams@schools.nyc.gov"

Scenario: Employee signs out
  Given an authenticated session for "QWilliams@schools.nyc.gov"
  When they sign out
  Then the session is terminated
```

```gherkin
Feature: Cart and quote

Scenario: Employee adds items to their cart
  Given a signed-in employee "QWilliams@schools.nyc.gov"
  And the catalog contains:
    | description                       | price |
    | Poland Spring Water (48 ct/8 oz) | 20.00 |
  When they add 8 of "Poland Spring Water (48 ct/8 oz)" to their cart
  Then the cart contains:
    | description                       | quantity |
    | Poland Spring Water (48 ct/8 oz) | 8        |

Scenario: Employee views a quote for their cart
  Given a signed-in employee "QWilliams@schools.nyc.gov"
  And their school code is "28Q082"
  And today is "12/22/25"
  And the tax rate is 0.08875
  And the shipping rate is 0.01
  And the cart contains:
    | description                       | quantity | price |
    | Poland Spring Water (48 ct/8 oz) | 8        | 20.00 |
  When they view their quote
  Then the quote number is "28Q082x122225"
  And the quote includes:
    | description                       | quantity | price | line_total |
    | Poland Spring Water (48 ct/8 oz) | 8        | 20.00 | 160.00     |
  And the subtotal is 160.00
  And the tax is subtotal * 0.08875
  And the shipping is subtotal * 0.01
  And the total is subtotal + tax + shipping
  And the delivery window is "School Hours"
```

```gherkin
Feature: Submit quote and invoice email

Scenario: Employee submits a quote and receives an invoice email
  Given a signed-in employee "QWilliams@schools.nyc.gov"
  And their school code is "28Q082"
  And today is "12/22/25"
  And the tax rate is 0.08875
  And the shipping rate is 0.01
  And the cart contains:
    | description                       | quantity | price |
    | Poland Spring Water (48 ct/8 oz) | 8        | 20.00 |
  When they submit the quote
  Then an invoice is created with quote number "28Q082x122225"
  And an invoice email is sent to "QWilliams@schools.nyc.gov"
  And the invoice email includes:
    | school                         | P.S. 082 - The Hammond School |
    | delivery_window                | School Hours                  |
    | line_item_description          | Poland Spring Water (48 ct/8 oz) |
    | line_item_quantity             | 8                              |
    | line_item_price                | 20.00                          |
  And the invoice includes tax and shipping fields
  And the invoice email includes a structured body and a PDF attachment
  And the cart is empty
```

## DSL Layer (Problem Domain Language)
- `given_school(name, code)`
- `given_new_employee(fields)`
- `when_sign_up()`
- `given_employee_account(email, password)`
- `when_sign_in(email, password)`
- `when_sign_out()`
- `given_catalog_item(description, price)`
- `when_add_to_cart(description, quantity)`
- `when_view_quote()`
- `when_submit_quote()`
- `given_tax_rate(rate)`
- `given_shipping_rate(rate)`
- `then_account_exists(email)`
- `then_sign_up_rejected()`
- `then_session_active(email)`
- `then_session_terminated()`
- `then_cart_contains(line_items)`
- `then_quote_matches(expected_quote)`
- `then_quote_tax(amount)`
- `then_quote_shipping(amount)`
- `then_quote_total(amount)`
- `then_invoice_sent(email, invoice_summary)`
- `then_invoice_has_tax_and_shipping()`
- `then_invoice_email_has_pdf_attachment()`
- `then_cart_is_empty()`

## Protocol Drivers / Stubs (In-Memory Only)
- `UserRepository` (in-memory)
- `SessionStore` (in-memory)
- `CatalogRepository` (in-memory, seeded with a minimal subset)
- `CartRepository` (in-memory)
- `QuoteRepository` (in-memory)
- `InvoiceRepository` (in-memory)
- `EmailOutbox` (captures sent emails)
- `InvoiceRenderer` (renders a PDF attachment for invoice)
- `Clock` (fixed test date)
- `RateProvider` (fixed tax/shipping rates for tests and configuration)
- `IdGenerator` (deterministic IDs if needed)

## Use Cases (Interactors)
- `SignUpEmployee`
- `SignInEmployee`
- `SignOutEmployee`
- `AddItemToCart`
- `ViewQuote`
- `SubmitQuote`

## Entities (Core Business Rules)
- `Employee` (email, name, title, school, phone, delivery window)
- `School` (name, code)
- `Cart`, `CartItem`
- `Quote`, `QuoteLineItem`
- `Invoice`, `InvoiceLineItem`

## Verification Output Requirements
- Test runner prints each test name and purpose.
- Test runner prints pass/fail for each scenario.
- Assertions include the expected vs actual values for quote number, totals, and email contents.
- Verbose logging enabled for test runs.

## Web Delivery Mechanism Plan (Axum + Askama + HTMX + Tailwind)
1. Controllers (Axum handlers) accept HTTP requests, validate inputs, and map to use case input DTOs.
2. Use case interactors return output data structures to presenters.
3. Presenters translate outputs into view models for templates (no business logic).
4. View models drive Askama templates that render SSR HTML and HTMX partials.
5. HTMX routes return partial fragments for cart updates, quote previews, and form error states.
6. Tailwind CSS styles the Askama templates; CSS is generated once and served as static assets.
7. Axum routes cover auth, cart, quote, and invoice flows; errors map to user-facing templates.
8. Request authentication is enforced in Axum middleware; sign-in/out use cases issue or terminate session tokens; session storage is an adapter implementing a port.
9. Email delivery stays in outer adapters; the use case depends only on an email port.

## Notes
- NYC sales tax rate is 8.875% (fixed). Shipping uses a fixed bulk truck delivery rate applied to subtotal.
  - Shipping rate is 1% (fixed) for now.
