# Zyn Language Grammar

zyn is a proc-macro template DSL for Rust. It compiles at macro-expansion time into code that builds a `proc_macro2::TokenStream`. The input to `zyn!(...)` is a stream of template nodes; the output is a Rust block expression evaluating to a `TokenStream`.

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
            | '@' 'throw' LitStr
            | '@' 'warn'  LitStr
            | '@' ElementName ElementBody

IfBody      = '(' Expr ')' '{' Element '}' ElseClause*
ElseClause  = '@' 'else' 'if' '(' Expr ')' '{' Element '}'
            | '@' 'else' '{' Element '}'

ForBody     = '(' ident 'of' Expr ')' '{' Element '}'

MatchBody   = '(' Expr ')' '{' MatchArm* '}'
MatchArm    = Pattern '=>' '{' Element '}' ','?
Pattern     = token_tree+    (* all tokens before '=>' *)

ElementName = ident ('::' ident)*
ElementBody = ('(' Props ')')? ('{' Element '}')?
Props       = PropField (',' PropField)* ','?
PropField   = ident '=' PropValue
PropValue   = token_tree+    (* all tokens until ',' or ')' *)
```
