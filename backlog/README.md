# Backlog

Planned features for zyn, broken into sequential phases. Each phase builds on the previous.

## Phases

| Phase | Name | Status | Description |
|---|---|---|---|
| [1](./01-attribute-trait.md) | FromInput Trait | complete | `FromInput` + `FromArg` traits, `Input` enum, typed `Arg` accessors, built-in scalar/container impls |
| [2](./02-derive-structs.md) | Derive Structs | planned | `#[derive(Attribute)]` for structs — generates `FromInput` (attribute mode) or `from_args` (argument mode) |
| [3](./03-derive-enums.md) | Derive Enums | planned | `#[derive(Attribute)]` for enums — generates `from_arg` for discriminated union field types |
| [4](./04-element-integration.md) | Element Integration | planned | `#[element]` params implementing `FromInput` are extracted automatically; plain params remain props |
| [5](./05-error-accumulation.md) | Error Accumulation | planned | Collect and emit all validation errors together instead of short-circuiting |

## What Gets Replaced

| Current approach | Replaced by | Phase |
|---|---|---|
| Manual `Arg`/`Args` querying | `#[derive(Attribute)]` + `FromInput` | 1–2 |
| `AttrExt`/`AttrsExt` as primary API | `FromInput::from_input(input.attrs())` via generated impl | 2 |

`Arg`, `Args`, `AttrExt`, `AttrsExt` remain as low-level internals, but are no longer the primary user-facing API.
