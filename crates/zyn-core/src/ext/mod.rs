//! Extension traits for common `syn` AST types.
//!
//! Provides ergonomic helpers for working with attributes, metadata, fields,
//! types, data, and items in procedural macros. Requires the `ext` feature
//! (`zyn = { features = ["ext"] }`).
//!
//! # Overview
//!
//! | Trait | Target | Purpose |
//! |-------|--------|---------|
//! | [`AttrExt`] | `syn::Attribute` | Name checking, argument parsing, dot-path metadata queries |
//! | [`MetaExt`] | `syn::Meta` | Variant predicates, conversions, nested navigation |
//! | [`FieldExt`] | `syn::Field` | Dot-path metadata queries on struct/enum fields |
//! | [`FieldsExt`] | `syn::Fields`, `syn::ItemStruct`, etc. | Variant predicates and field lookup via [`FieldKey`] |
//! | [`TypeExt`] | `syn::Type`, `syn::Field` | Detecting `Option`/`Result` wrappers, inner type extraction |
//! | [`DataExt`] | `syn::Data` | Variant predicates and conversions for struct/enum/union data |
//! | [`ItemExt`] | `syn::Item` | Variant predicates, conversions, and common field accessors |
//! | [`VariantExt`] | `syn::Variant` | Dot-path metadata queries on enum variants |
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::{AttrExt, MetaExt, FieldExt, FieldsExt, FieldKey, TypeExt, DataExt, ItemExt, VariantExt};
//!
//! // Attribute querying with dot-paths
//! if attr.is("serde") {
//!     let rename = attr.get("rename");
//! }
//!
//! // Field attribute navigation
//! let meta = field.get("serde.rename");
//!
//! // Fields lookup
//! let key: FieldKey = "id".into();
//! if let Some(f) = fields.get(&key) {
//!     // ...
//! }
//!
//! // Type inspection
//! if field.is_option() {
//!     let inner = field.inner_type().unwrap();
//! }
//!
//! // Data variant checking
//! if data.is_enum() {
//!     let e = data.as_enum().unwrap();
//! }
//!
//! // Item accessors
//! let ident = item.ident();
//! let attrs = item.attrs();
//! ```

mod attr;
mod data;
mod field;
mod fields;
mod item;
mod meta;
mod ty;
mod variant;

pub use attr::*;
pub use data::*;
pub use field::*;
pub use fields::*;
pub use item::*;
pub use meta::*;
pub use ty::*;
pub use variant::*;
