# engine-rust

Cargo workspace for the index/query core. `unsafe_code = "forbid"` at the
workspace level; clippy `-D warnings` + `cargo fmt --check` gate CI.

Conventions:
- One crate per subsystem (`loom-postings`, `loom-index`, `loom-query`, …);
  keep crates small and single-purpose
- Decoders are total: arbitrary bytes must never panic (fuzz targets enforce)
- Core guarantees are proptests (SPIMI == naive oracle, BMW == exhaustive
  top-k, codec roundtrips), not example tests
- On-disk formats are documented byte-level in `docs/STORAGE_FORMATS.md`
  before or with the code that writes them
- Index builds are immutable dirs + atomic `CURRENT` pointer swap; never
  mutate a published build
