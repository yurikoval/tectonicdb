# Repository contract

## Architecture
- `crates/tdb-core`: DTF storage, metadata, analytics, and order-book processing.
- `crates/tdb-server-core`: async TCP server and broker/state; `influx` is optional.
- `crates/tdb-cli`: synchronous Rust client.
- `bins/`: server, CLI, and DTF utility entrypoints.
- `cli/`: separately managed language clients, including TypeScript.

## Boundaries and checks
Rust source is the normal edit surface. `cli/tectonicjs/built`, tracked DTF fixtures, and runtime outputs are generated or fixture data; do not rewrite them unless directly relevant. Keep changes small and do not mass-format the legacy Rust tree.

Run `bin/check` for the reproducible workspace build and sequential default/no-default feature tests. Focused tests use `RUST_TEST_THREADS=1 cargo test ...`. `cargo fmt --all -- --check` is optional; the legacy tree is not yet rustfmt-clean.

Tests must remain sequential because integration tests use server/database resources and create runtime output in a temporary database directory. The integration test performs 10,000 inserts and waits for server readiness and final count.

The root `influx` feature is enabled by default and forwards to `tdb_server_core/influx`; the no-default-features check verifies the dependency can build without it.
