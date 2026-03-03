# Phase 4: Pipes + Pipe Trait ✅

## Status: Merged into Phase 1

Pipe logic was merged into the AST module alongside parsing and expansion. The `Pipe` trait and case conversion functions live in `src/lib.rs`. Pipe node types (Parse + Expand) live in `src/ast/pipe_node.rs`. There is no separate `crates/derive/src/pipe.rs`.

Tests: `tests/pipes.rs` (8 tests) and `tests/case_conversion.rs` (18 tests).

See [1-ast.md](1-ast.md) for full details.
