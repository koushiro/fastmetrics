use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput};

/// Wraps the impl block in a "dummy const"
pub fn wrap_in_const(input: DeriveInput, impl_block: TokenStream) -> TokenStream {
    let attrs = input.attrs.into_iter().filter(is_lint_attribute);

    quote! {
        #[doc(hidden)]
        const _: () = {
            #(#attrs)*
            #impl_block
        };
    }
}

// Check if the attribute is `#[allow(..)]`, `#[deny(..)]`, `#[forbid(..)]` or `#[warn(..)]`.
pub fn is_lint_attribute(attr: &Attribute) -> bool {
    attr.path().is_ident("allow")
        || attr.path().is_ident("deny")
        || attr.path().is_ident("forbid")
        || attr.path().is_ident("warn")
}
