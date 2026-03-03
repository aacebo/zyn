# Elements

Elements are reusable template components. Define them with `#[zyn::element]` and invoke them with `@` in templates.

## Defining

Annotate a function that returns `syn::Result<proc_macro2::TokenStream>`:

```zyn
#[zyn::element]
fn field_decl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}
```

This generates a struct `FieldDecl` with public fields for each parameter, plus an implementation of the `Render` trait. The function name (snake_case) becomes the template name; the struct name is the PascalCase equivalent.

## Invoking

Reference the element by its snake_case name with `@`:

```zyn
zyn! {
    @field_decl(
        vis = syn::parse_quote!(pub),
        name = quote::format_ident!("age"),
        ty = syn::parse_quote!(u32),
    )
}
// output: pub age: u32,
```

## Children

Elements can accept children by including a `children: proc_macro2::TokenStream` parameter:

```zyn
#[zyn::element]
fn wrapper(name: proc_macro2::Ident, children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(struct #name { #children }))
}

zyn! {
    @wrapper(name = quote::format_ident!("Foo")) {
        x: i32,
    }
}
// output: struct Foo { x: i32, }
```

Children-only elements can omit parentheses entirely:

```zyn
#[zyn::element]
fn container(children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(mod inner { #children }))
}

zyn! {
    @container {
        struct Foo;
    }
}
```

## Zero Parameters

Elements with no parameters don't need parentheses:

```zyn
#[zyn::element]
fn divider() -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(const DIVIDER: &str = "---";))
}

zyn! { @divider }
zyn! { @divider() }  // also valid
```

## Custom Names

Override the template name with a string argument:

```zyn
#[zyn::element("say_hello")]
fn internal_greeting(name: proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(fn {{ name }}() {}))
}

zyn! { @say_hello(name = quote::format_ident!("world")) }
```

## Namespaced Elements

Elements defined in submodules can be referenced with path syntax:

```zyn
mod components {
    #[zyn::element]
    pub fn field_decl(name: proc_macro2::Ident, ty: proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
        Ok(zyn::zyn!({{ name }}: {{ ty }},))
    }
}

zyn! {
    @components::field_decl(
        name = quote::format_ident!("age"),
        ty = quote::format_ident!("u32"),
    )
}
```

## Elements in Loops

Elements compose naturally with control flow:

```zyn
zyn! {
    @for (name of names) {
        @greeting(name = name.clone())
    }
}
```
