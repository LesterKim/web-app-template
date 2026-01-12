# Project: Rust Clean Architecture Web App

## Quick Reference
- **Stack**: Rust, Axum, Askama (SSR), Tokio, PostgreSQL
- **Build**: `cargo build`
- **Test**: `cargo test --verbose`
- **Run**: `cargo run -p web`

## Architecture (Dependencies Point Inward)
```
Frameworks (apps/web, datastore) → Use Cases (core/use_cases) → Entities (core/entities)
                                          ↑
                                   Ports (core/ports)
```

**Dependency Rule**: Inner layers define interfaces via `core/ports`; outer layers implement them.

## Testing Requirements

### Two-Level Strategy
| Level | Where | Focus |
|-------|-------|-------|
| Acceptance (ATDD) | `core/use_cases/tests/` | WHAT system does from user view |
| Unit (TDD) | Within crate `tests/` modules | HOW components work in isolation |

### TDD Cycle
1. Write failing test → 2. Run to confirm failure → 3. Minimal code to pass → 4. Run to confirm → 5. Refactor → 6. Repeat

### ATDD Four Layers
When writing acceptance tests:
- **Test Cases**: Executable specs in domain language
- **DSL**: Shared abstractions for test operations
- **Protocol Drivers**: Adapters between DSL and system
- **SUT**: The actual implementation

### Test Output
Always show: test name, pass/fail status, relevant assertions. Use `--verbose` flag.

## Design Principles
- Composition over inheritance
- Depend on traits, not concrete types
- Favor dynamic polymorphism over static polymorphism
- Prefer trait objects for ports/adapters
- Use domain language (ubiquitous language)
- Business rules in `core/` have zero framework dependencies

## Web Application Rules
- Server handles: filtering, sorting, searching, validation
- Default to server-side rendering (Askama templates)
- Frontend is presentation only
- `apps/web` is a thin delivery mechanism

## Implementation Constraints
- Only implement tests for explicitly specified behavior
- Do not assume unspecified scenarios
- Note additional test suggestions without implementing them
