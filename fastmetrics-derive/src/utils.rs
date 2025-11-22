use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, ExprLit, Lit, LitStr, Result};

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

#[derive(Clone)]
/// Represents either a string literal or an arbitrary expression that evaluates to a string.
pub enum StringValue {
    /// String literal: "name"
    Literal(LitStr),
    /// Any expression expected to evaluate to a string: CONST, function_call(), concat!(...), etc.
    Expression(TokenStream),
}

impl StringValue {
    /// Build a `StringValue` from a parsed expression.
    ///
    /// Accepts:
    /// - string literals: "abc"
    /// - path expressions: CONST_STR, module::CONST
    /// - function calls: some_fn(), module::some_fn()
    /// - method calls: obj.method()
    /// - field access: obj.field
    /// - index access: arr[0]
    /// - macro invocations: concat!("a", "b"), format!("x")
    /// - conditional expressions: if .. { .. } else { .. }
    /// - match expressions: match x { ... }
    /// - reference / try / paren / group wrappers
    pub fn from_expr(expr: &Expr) -> Result<Self> {
        match expr {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(Self::Literal(s.clone())),
            Expr::Path(_)
            | Expr::Call(_)
            | Expr::MethodCall(_)
            | Expr::Field(_)
            | Expr::Index(_)
            | Expr::Macro(_)
            | Expr::If(_)
            | Expr::Match(_)
            | Expr::Reference(_)
            | Expr::Try(_) => Ok(Self::Expression(quote!(#expr))),
            Expr::Paren(paren) => Self::from_expr(&paren.expr),
            Expr::Group(group) => Self::from_expr(&group.expr),
            _ => Err(syn::Error::new_spanned(
                expr,
                "expect a string literal or any expression that evaluates to a string",
            )),
        }
    }

    /// Convert into tokens ready for insertion in generated code.
    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            Self::Literal(lit) => quote!(#lit),
            Self::Expression(ts) => ts.clone(),
        }
    }
}
