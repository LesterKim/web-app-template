# Plan: Quote Detail View from /quotes

## Goals
- Let a user click a quote on `/quotes` and view full quote details.
- Show items, quantities, submission timestamp, fee, tax, subtotal, and total.
- Keep Clean Architecture: use cases return DTOs, presenters map to view models.

## Data/Domain Changes
1) **Add timestamp to stored quotes**
   - Extend `QuoteDraft` and `QuoteRecord` with a `submitted_at` field (UTC).
   - Introduce a small `Clock` port for testable timestamps.
   - In-memory quote repository stores the timestamp; Postgres stub returns error as before.

## Use Case Changes
2) **GetQuoteDetails use case**
   - Input: `employee_id`, `quote_id`.
   - Output DTO: `QuoteDetailsOutput` with items, totals, and timestamp.
   - Validation: ensure quote belongs to the employee; else `UseCaseError::NotFound`.
   - TDD: add failing unit test first, then implement.

## Repository/Port Updates
3) **QuoteRepository additions**
   - Add `get_quote(employee_id, quote_id)` to fetch a single quote.
   - Update in-memory repo to implement it.

4) **Clock port**
   - Add `Clock` trait in `core/ports` (or `core/use_cases` if you prefer).
   - Implement `SystemClock` adapter in datastore for production.
   - Fake clock in tests for deterministic timestamps.

## Web/UI Changes
5) **Routes and templates**
   - `/quotes` page: each row links to `/quotes/:id`.
   - New route `/quotes/:id` to render detail view.
   - New template `quote_detail.html` with item list + totals + timestamp.
   - Presenter mapper: `QuoteDetailsOutput` → `QuoteDetailViewModel`.

## Tests (RED‑GREEN‑REFACTOR)
6) **Core tests first**
   - Add unit test for `GetQuoteDetailsInteractor`.
   - Add/update tests for timestamp storage and ownership checks.
   - Run `cargo test -p core_use_cases` after each test change.

## Validation
7) **Compile + smoke**
   - `cargo check -p web`
   - Manual click-through: `/quotes` → `/quotes/:id`.

## Files Likely Touched
- `core/entities/src/lib.rs`
- `core/ports/src/lib.rs`
- `core/use_cases/src/outputs.rs`
- `core/use_cases/src/lib.rs`
- `core/use_cases/tests/use_cases_test.rs`
- `datastore/src/lib.rs`
- `datastore/src/postgres.rs`
- `apps/web/src/routes.rs`
- `apps/web/src/presenters/mod.rs`
- `apps/web/src/view_models.rs`
- `apps/web/templates/quotes.html`
- `apps/web/templates/quote_detail.html` (new)
