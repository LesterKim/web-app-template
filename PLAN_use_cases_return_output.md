# Plan: Return Output Data from Use Case Interactors (Remove Output Boundaries)

## Goals
- Use case interactors return output DTOs directly.
- Remove output boundary traits/interfaces and presenter side-effects.
- Presenters become pure mappers: input DTO -> view model.

## Scope Summary
- Core use cases: return concrete output structs.
- Core ports: remove `output_boundary` module and any boundary traits.
- Web presenters: accept output data and produce view models.
- Tests: update to assert returned outputs instead of capturing presenter calls.

## Steps (TDD Order)
1) **Update core tests first (RED)**
   - One test at a time: change a test to expect direct `Output` returns from the interactor.
   - Remove presenter doubles in that test only.
   - Run the test to confirm it fails to compile or fails at runtime (RED).

2) **Refactor use case interactors (GREEN)**
   - Change the targeted interactor `execute` to return `Result<OutputType, UseCaseError>`.
   - Remove presenter dependencies from that interactor’s constructor.
   - Keep error handling identical.
   - Re-run tests to pass (GREEN), then clean up (REFACTOR).
   - Repeat per test until all use cases are migrated.

3) **Remove output boundaries (after all tests are GREEN)**
   - Delete `core/ports/src/output_boundary.rs`.
   - Remove boundary trait references from `core/ports/src/lib.rs`.
   - Move output DTO structs into `core/use_cases` (or a new `core/use_cases::output` module).

4) **Update web presenters and routes**
   - Convert presenters into pure mappers: `fn present(output) -> ViewModel`.
   - Update routes to call interactor, then pass output into presenter mapper.

5) **Clean up imports and wiring**
   - Remove references to `core_ports::output_boundary::*`.
   - Update `Cargo.toml` dependencies if any were only used for output boundaries.
   - Run `cargo test -p core_use_cases` and `cargo check -p web`.

## Files Likely Touched
- `core/ports/src/lib.rs`
- `core/ports/src/output_boundary.rs` (delete)
- `core/use_cases/src/lib.rs`
- `core/use_cases/tests/use_cases_test.rs`
- `apps/web/src/presenters/mod.rs`
- `apps/web/src/routes.rs`

## Open Questions
- Should output DTOs live in `core/use_cases` or in `core/entities`? (Plan assumes `core/use_cases` to keep them app-specific.)
- Do you want presenters as free functions or structs with `impl` methods?
