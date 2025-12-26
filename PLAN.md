# Clean Architecture Monorepo Plan

## Directory tree (planned)
```
.
├── AGENTS.md
├── PLAN.md
├── README.md
├── .gitignore
├── .dockerignore
├── Cargo.toml
├── core
│   ├── entities
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── ports
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── output_boundary.rs
│   └── use_cases
│       ├── Cargo.toml
│       ├── src
│       │   └── lib.rs
│       └── tests
│           └── use_cases_test.rs
├── shared
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       └── error.rs
├── datastore
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       └── postgres.rs
├── apps
│   └── web
│       ├── Cargo.toml
│       ├── package.json
│       ├── postcss.config.js
│       ├── tailwind.config.js
│       ├── tsconfig.json
│       ├── src
│       │   ├── main.rs
│       │   ├── http.rs
│       │   ├── config.rs
│       │   ├── routes.rs
│       │   ├── presenters
│       │   │   └── mod.rs
│       │   └── view_models.rs
│       ├── templates
│       │   ├── base.html
│       │   └── index.html
│       ├── assets
│       │   ├── app.css
│       │   └── app.ts
│       └── static
│           ├── app.css
│           └── app.js
├── pipelines
│   └── sample_pipeline
│       ├── Cargo.toml
│       ├── src
│       │   └── main.rs
│       └── tests
│           └── pipeline_test.rs
├── ai
│   └── evals
│       ├── Cargo.toml
│       ├── README.md
│       ├── evals.toml
│       ├── fixtures
│       │   └── sample_cases.jsonl
│       └── src
│           └── main.rs
├── infra
│   ├── Dockerfile.web
│   ├── docker-compose.yml
│   └── terraform
│       ├── main.tf
│       ├── variables.tf
│       ├── outputs.tf
│       ├── versions.tf
│       └── modules
│           └── gcp_app
│               ├── main.tf
│               ├── variables.tf
│               └── outputs.tf
└── .github
    └── workflows
        └── pipelines.yml
```

## Files to be created (one-line descriptions)
- README.md: Repository overview, architecture, and local dev commands.
- .gitignore: Ignore Rust, Node, Terraform, and editor artifacts.
- .dockerignore: Reduce Docker build context for Rust and Node.
- Cargo.toml: Workspace configuration for all Rust crates.
- core/entities/Cargo.toml: Entities crate manifest (no external deps).
- core/entities/src/lib.rs: Domain entities and invariants.
- core/ports/Cargo.toml: Ports crate manifest (interfaces only).
- core/ports/src/lib.rs: Ports as traits for persistence/external services.
- core/ports/src/output_boundary.rs: Output boundary trait(s) for use case interactors.
- core/use_cases/Cargo.toml: Use-cases crate manifest.
- core/use_cases/src/lib.rs: Application use cases (business rules only).
- core/use_cases/tests/use_cases_test.rs: Use case tests (fast unit style).
- shared/Cargo.toml: Shared utilities crate manifest.
- shared/src/lib.rs: Shared crate entry and module wiring.
- shared/src/error.rs: Common error types reused across crates.
- datastore/Cargo.toml: Datastore crate manifest (persistence adapters).
- datastore/src/lib.rs: Datastore crate entry and adapter wiring.
- datastore/src/postgres.rs: PostgreSQL adapter implementing core ports.
- apps/web/Cargo.toml: Axum web app crate manifest.
- apps/web/package.json: Frontend build scripts and dependencies (Tailwind/Alpine).
- apps/web/postcss.config.js: PostCSS pipeline for Tailwind.
- apps/web/tailwind.config.js: Tailwind configuration with template paths.
- apps/web/tsconfig.json: TypeScript config for minimal frontend scripts.
- apps/web/src/main.rs: Axum server bootstrap and wiring.
- apps/web/src/http.rs: HTTP server setup (router, middleware, state).
- apps/web/src/config.rs: Runtime configuration (env vars, defaults).
- apps/web/src/routes.rs: Route handlers and HTMX endpoints.
- apps/web/src/presenters/mod.rs: Presenters that build view models from use case output.
- apps/web/src/view_models.rs: View model data structures for HTMX views.
- apps/web/templates/base.html: Base HTMX layout.
- apps/web/templates/index.html: HTMX view template consuming view models.
- apps/web/assets/app.css: Tailwind input stylesheet.
- apps/web/assets/app.ts: Minimal frontend logic in TypeScript.
- apps/web/static/app.css: Built Tailwind CSS output.
- apps/web/static/app.js: Built JS output (compiled from app.ts).
- pipelines/sample_pipeline/Cargo.toml: Pipeline crate manifest.
- pipelines/sample_pipeline/src/main.rs: Sample pipeline binary entry.
- pipelines/sample_pipeline/tests/pipeline_test.rs: Pipeline tests (fast checks/characterization).
- ai/evals/Cargo.toml: AI evals runner crate manifest.
- ai/evals/README.md: Evals usage and how to add cases/metrics.
- ai/evals/evals.toml: Evals configuration (models, metrics, thresholds).
- ai/evals/fixtures/sample_cases.jsonl: Sample eval cases fixture.
- ai/evals/src/main.rs: Eval runner entry (loads cases, runs metrics).
- infra/Dockerfile.web: Docker image for Axum web app.
- infra/docker-compose.yml: Local compose for web + Postgres.
- infra/terraform/main.tf: Root Terraform module wiring.
- infra/terraform/variables.tf: Root module inputs.
- infra/terraform/outputs.tf: Root module outputs.
- infra/terraform/versions.tf: Terraform and provider version pins.
- infra/terraform/modules/gcp_app/main.tf: GCP app module resources.
- infra/terraform/modules/gcp_app/variables.tf: GCP module inputs.
- infra/terraform/modules/gcp_app/outputs.tf: GCP module outputs.
- .github/workflows/pipelines.yml: GitHub Actions workflow for pipeline crates.

## Files to be updated
- AGENTS.md: Add guidance to prefer dynamic polymorphism (trait objects) for ports/adapters.

## Dependencies to install
- Rust toolchain (cargo/rustc via rustup)
- Node.js + npm (for Tailwind and Alpine build tooling)

## Configuration files needed
- Cargo workspace configuration: `Cargo.toml`
- Tailwind + Alpine.js build setup: `apps/web/package.json`, `apps/web/tailwind.config.js`, `apps/web/postcss.config.js`
- TypeScript setup: `apps/web/tsconfig.json`
- Evals setup: `ai/evals/evals.toml`
- GitHub Actions workflow: `.github/workflows/pipelines.yml`
- Docker config: `infra/Dockerfile.web`, `infra/docker-compose.yml`, `.dockerignore`
- Terraform config: `infra/terraform/*.tf`, `infra/terraform/modules/gcp_app/*.tf`
- Repo hygiene: `.gitignore`, `README.md`
