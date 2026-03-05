# Backlog

Planned features for zyn, broken into sequential phases. Each phase builds on the previous.

## Phases

| Phase | Name | Status | Description |
|---|---|---|---|
| [1](./01-attribute.md) | Attribute | planned | `Attribute` trait + `#[derive(Attribute)]` for typed extraction from attributes and args |
| [2](./02-error-accumulation.md) | Error Accumulation | planned | Collect and emit all validation errors together instead of short-circuiting |

## What Gets Replaced

| Current module | Replaced by | Phase |
|---|---|---|
| Manual `Arg`/`Args` querying | `Attribute` trait + `#[derive(Attribute)]` | 1 |
| `AttrExt`/`AttrsExt` as primary API | `Attribute::attribute` (name baked into derive) | 1 |

`Arg`, `Args`, `AttrExt`, `AttrsExt` remain as low-level internals used by `Attribute`, but are no longer the primary user-facing API.
