# Phase 4: Pipes + Pipe Trait ✅

## Status: Merged into Phase 1

All pipes dispatch uniformly via the `Pipe` trait — no distinction between built-in and custom pipes. Built-in pipes are PascalCase unit structs (`Upper`, `Lower`, `Snake`, `Camel`, `Pascal`, `Screaming`) with case conversion logic inlined in their `pipe()` methods. `PipeNode` is a single struct with `name` + `args` fields.

Tests: `crates/core/tests/pipes.rs` (8 tests) and `crates/core/tests/case_conversion.rs` (18 tests).

See [1-ast.md](1-ast.md) for full details.
