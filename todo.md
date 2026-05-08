# Tasks

## Phase 1: Foundation & Error Handling
- [x] Create `todo.md` and `journal.md` <!-- Done -->
- [x] Refine `GqueuesError` in `src/client.rs`
- [x] Update all public methods in `GqueuesClient` to return `Result<T, GqueuesError>`
- [x] Remove `anyhow` from dependencies in `Cargo.toml`

## Phase 2: Client Builder Implementation
- [x] Implement `GqueuesClientBuilder` in `src/client.rs`
- [x] Update `GqueuesClient` to use the configured settings from the builder

## Phase 3: Documentation & Examples
- [x] Add crate-level documentation in `src/lib.rs`
- [x] Add detailed doc comments (`///`) to `GqueuesClient`, `GqueuesClientBuilder`, and all public methods
- [x] Add doc comments to all models in `src/models.rs`
- [x] Create `README.md` with a "Quick Start" guide and examples

## Phase 4: Integration Testing
- [x] Add `wiremock` and `tokio` (with macros) to `[dev-dependencies]` in `Cargo.toml`
- [x] Create `tests/integration_tests.rs`
- [x] Implement mock tests for `get_queues`, `get_tasks`, `create_task`, and error cases

## Phase 5: Security & Logging Audit
- [x] Review all `log::debug!` calls
- [x] Ensure `access_token` is never logged
- [x] Sanitize or limit logging of response bodies

## Phase 6: Infrastructure & Final Review
- [x] Run `cargo check`, `cargo fmt`, and `cargo clippy`
- [x] Run all tests
- [x] Verify `Cargo.toml` fields

## Other TODOs:
- [ ] Make sure the API timeout is respecting the Gqueues provided "Retry after..."