use syn::Attribute;

use crate::meta::Args;

/// Extension methods for a single `syn::Attribute`.
pub trait AttrExt {
    /// Returns `true` if the attribute's path matches the given name.
    fn is(&self, name: &str) -> bool;
    /// Parses the attribute's arguments into an [`Args`] list.
    fn args(&self) -> syn::Result<Args>;
}

impl AttrExt for Attribute {
    fn is(&self, name: &str) -> bool {
        self.path().is_ident(name)
    }

    fn args(&self) -> syn::Result<Args> {
        self.parse_args::<Args>()
    }
}

/// Extension methods for a slice of `syn::Attribute`.
pub trait AttrsExt {
    /// Returns the first attribute matching the given name.
    fn find_attr(&self, name: &str) -> Option<&Attribute>;
    /// Parses the arguments of the first attribute matching the given name.
    fn find_args(&self, name: &str) -> syn::Result<Option<Args>>;
    /// Returns `true` if any attribute matches the given name.
    fn has_attr(&self, name: &str) -> bool;
    /// Merges arguments from all attributes matching the given name.
    fn merge_args(&self, name: &str) -> syn::Result<Args>;
}

impl AttrsExt for [Attribute] {
    fn find_attr(&self, name: &str) -> Option<&Attribute> {
        self.iter().find(|a| a.is(name))
    }

    fn find_args(&self, name: &str) -> syn::Result<Option<Args>> {
        match self.find_attr(name) {
            Some(attr) => Ok(Some(attr.args()?)),
            None => Ok(None),
        }
    }

    fn has_attr(&self, name: &str) -> bool {
        self.iter().any(|a| a.is(name))
    }

    fn merge_args(&self, name: &str) -> syn::Result<Args> {
        let mut result = Args::new();

        for attr in self.iter().filter(|a| a.is(name)) {
            let args = attr.args()?;
            result.extend(args);
        }

        Ok(result)
    }
}
