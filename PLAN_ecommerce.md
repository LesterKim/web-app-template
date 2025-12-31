# Mock E-commerce Site Plan (NYC Public School Employee)

## Goals
- Deliver a server-rendered mock e-commerce flow with sign up/in/out, cart, quote, and invoice confirmation.
- Keep business rules in core entities/use cases; web and datastore remain adapters.
- Provide a mock email "send" on invoice confirmation (recorded and viewable).

## Open Questions / Assumptions (please confirm)
- NYC employee validation: ok to require email domain `@schools.nyc.gov` (or specify other domains)?
- Quote pricing: no tax/shipping, just item totals unless you want a flat tax or fee.
- Password handling: use a simple hashing adapter (Argon2) vs. plaintext (mock-only).
- Invoice email: display a "sent" confirmation page and store the message in-memory (no real SMTP).

## Clean Architecture Outline
- Entities (core/entities):
  - Employee (id, name, email, password_hash)
  - Product (id, name, category [Food|Accessory], unit_price)
  - Cart + CartItem (product_id, quantity)
  - Quote (line totals, subtotal)
  - Invoice (id, employee_id, items, total, status)
  - Money type for currency-safe math
- Use Cases (core/use_cases):
  - RegisterEmployee
  - SignIn / SignOut (session management via port)
  - ListCatalog
  - AddItemToCart / UpdateCart / ViewCart
  - GetQuote
  - ConfirmOrder (create invoice, send email)
- Ports (core/ports):
  - EmployeeRepository
  - CatalogRepository
  - CartRepository
  - SessionRepository
  - InvoiceRepository
  - EmailGateway
  - Output boundaries + DTOs for each use case
- Adapters:
  - datastore: in-memory repos + seeded catalog + email outbox
  - apps/web: presenters, view models, routes, templates

## Implementation Steps
1) Domain modeling
   - Replace greeting sample with ecommerce entities and Money type.
   - Define invariants (email domain, positive quantity, non-negative money).
2) Ports and output boundaries
   - Create repository traits and output DTOs for each use case.
3) Use cases + TDD
   - Write failing unit tests per use case (core/use_cases/tests).
   - Implement minimal logic to pass tests (registration, auth, cart, quote, confirm).
4) Datastore adapters (mock)
   - In-memory stores for employees, sessions, carts, invoices.
   - Seeded catalog (food + accessories).
   - Email outbox adapter that records sent invoices.
5) Web adapter
   - New routes for sign up/in/out, catalog, cart, quote, confirm.
   - Presenters + view models to keep controllers thin.
   - Templates and styles for the storefront, cart, and invoice confirmation.
6) Wiring + config
   - Update `apps/web/src/main.rs` and `apps/web/src/http.rs` to pass new deps.
   - Remove greeting endpoints and templates.
7) Validation
   - Run `cargo test -p core_use_cases` and `cargo test` (workspace).
   - Spot-check the server with `cargo run -p web`.

## Deliverables
- Clean Architecture-aligned Rust domain for the ecommerce flow.
- Server-rendered UI with catalog + cart + quote + invoice confirmation.
- Mock email "send" stored in-memory and visible on confirmation.
