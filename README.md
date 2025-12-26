# Clean Architecture Monorepo Template

This repository is a Clean Architecture monorepo with Rust backends, HTMX-based server rendering, and infrastructure templates.

## Structure
- `core/entities`: Domain entities (no external dependencies)
- `core/ports`: Interfaces (input/output boundaries, repositories)
- `core/use_cases`: Interactors (business rules)
- `datastore`: Persistence adapters (PostgreSQL, in-memory)
- `apps/web`: Axum delivery mechanism (routes, presenters, HTMX templates)
- `pipelines`: Rust pipeline binaries
- `ai/evals`: AI eval runner and fixtures
- `infra`: Docker and Terraform for GCP
- `shared`: Cross-cutting utilities

## Local development
- Node version (via nvm): `nvm install 24.12.0` then `nvm use` (pinned in `.nvmrc`)
- Build workspace: `cargo build`
- Test core use cases: `cargo test -p core_use_cases`
- Run web app: `cargo run -p web`
- Frontend assets (from `apps/web`):
  - `npm install`
  - `npm run build`

## Clean Architecture flow (web)
1. Route handler invokes a use case interactor.
2. Interactor calls the output boundary (trait in `core/ports`).
3. Presenter implements the boundary, builds a view model.
4. Template renders HTMX using the view model.

## Configuration
- `WEB_ADDR` controls bind address (default `127.0.0.1:3000`).
- `DATABASE_URL` selects the PostgreSQL adapter; omitted uses in-memory data.
