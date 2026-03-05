# Grammar

Formal EBNF grammar for the zyn template language. `zyn!(...)` accepts an `Element` as input.

```ebnf
Element     = Node*

Node        = TokensNode
            | InterpNode
            | GroupNode
            | AtNode

TokensNode  = token_tree+
            (* any token tree(s) that are NOT: '@', '{', '(', '[' *)

InterpNode  = '{' '{' Expr ('|' Pipe)* '}' '}'
            (* outer brace content must be exactly one inner brace group *)

Expr        = token_tree+
            (* all token trees before the first '|' or the closing inner '}' *)

Pipe        = ident (':' PipeArg)*
PipeArg     = token_tree+
            (* tokens until the next ':' or '|' *)

GroupNode   = '(' Element ')'
            | '[' Element ']'
            | '{' Element '}'    (* only when brace disambiguation fails *)

AtNode      = '@' 'if'    IfBody
            | '@' 'for'   ForBody
            | '@' 'match' MatchBody
            | '@' ElementName ElementBody

IfBody      = '(' Expr ')' '{' Element '}' ElseClause*
ElseClause  = '@' 'else' 'if' '(' Expr ')' '{' Element '}'
            | '@' 'else' '{' Element '}'

ForBody     = '(' ident 'in' Expr ')' '{' Element '}'
            | '(' Expr ')' '{' Element '}'
            (* classic form: @for (count) { body } expands to for _ in 0..count *)

MatchBody   = '(' Expr ')' '{' MatchArm* '}'
MatchArm    = Pattern '=>' '{' Element '}' ','?
Pattern     = token_tree+    (* all tokens before '=>' *)

ElementName = ident ('::' ident)*
ElementBody = ('(' Props ')')? ('{' Element '}')?
Props       = PropField (',' PropField)* ','?
PropField   = ident '=' PropValue
PropValue   = token_tree+    (* all tokens until ',' or ')' *)
```

## Parsing Model

`Element::parse` is the root parser. It reads token trees in a loop, dispatching on the leading token:

| Leading token | Action |
|---|---|
| `@` | Flush pending tokens; parse `AtNode` |
| `{` | Flush pending tokens; run brace disambiguation |
| `(` | Flush pending tokens; parse `GroupNode` (Parenthesis) |
| `[` | Flush pending tokens; parse `GroupNode` (Bracket) |
| anything else | Accumulate into pending `TokenStream` |

When the input is exhausted, any pending tokens are flushed as a `TokensNode`.

## Brace Disambiguation

When the parser sees `{`, it must decide between an `InterpNode` and a brace-delimited `GroupNode`:

1. Consume the outer `{`, capturing its content stream.
2. **Fork** the content stream and attempt to consume a single inner `{` group.
3. If the fork consumed exactly one inner brace group AND the outer content is now empty → **interpolation** (`InterpNode`).
4. Otherwise → **group** (`GroupNode` with `Delimiter::Brace`).

In concrete terms:
- `{{ expr }}` — outer brace contains only `{ expr }` → `InterpNode`
- `{ anything }` where content is not a single inner brace group → `GroupNode`
- `{{ }}` (empty inner) → parse error: `"empty interpolation"`

The `{{` and `}}` are two separate `{` / `}` tokens, not a special lexeme.
