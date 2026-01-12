# ATDD Workflow

Implement a feature using Acceptance Test Driven Development:

1. **Write the acceptance test first** in `core/use_cases/tests/`
   - Use domain language (what the user sees)
   - Describe WHAT, not HOW

2. **Create DSL layer** if needed
   - Abstract common test operations
   - Enable precision where needed

3. **Run tests to confirm failure**
   ```bash
   cargo test --verbose -p core_use_cases
   ```

4. **Implement minimal code to pass**
   - Work inward: entities → use_cases → adapters

5. **Run tests to confirm success**
   ```bash
   cargo test --verbose -p core_use_cases
   ```

6. **Refactor** while keeping tests green

Remember: Dependencies point inward only. Inner layers define interfaces in `core/ports`.
