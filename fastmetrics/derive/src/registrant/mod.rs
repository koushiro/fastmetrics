mod attribute;
mod field;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn expand_derive_registrant(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(body) => match body.fields {
            syn::Fields::Named(fields) => fields,
            syn::Fields::Unnamed(fields) => {
                return Err(Error::new_spanned(
                    fields,
                    "Can not derive Registrant for struct with unnamed fields.",
                ));
            },
            syn::Fields::Unit => {
                return Err(Error::new_spanned(name, "Can not derive Registrant for unit struct."));
            },
        },
        syn::Data::Enum(_) => {
            return Err(Error::new_spanned(name, "Can not derive Registrant for enum."));
        },
        syn::Data::Union(_) => {
            return Err(Error::new_spanned(name, "Can not derive Registrant for union."));
        },
    };

    let register_calls = fields
        .named
        .into_iter()
        .try_fold(vec![], |mut acc, field| {
            acc.push(field::Field::try_from(field)?);
            Ok::<Vec<field::Field>, syn::Error>(acc)
        })?
        .into_iter()
        .filter_map(|field| {
            if field.skip() {
                return None;
            }

            let ident = field.ident();
            let name = field.name();
            let help = field.help();
            let body = match field.unit() {
                Some(unit) => {
                    quote! {
                        registry.register_with_unit(
                            #name,
                            #help,
                            ::fastmetrics::registry::Unit::Other(::std::borrow::Cow::Borrowed(#unit)),
                            self.#ident.clone(),
                        )?;
                    }
                }
                None => {
                    quote! {
                        registry.register(
                            #name,
                            #help,
                            self.#ident.clone(),
                        )?;
                    }
                }
            };

            Some(body)
        });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::fastmetrics::registry::Registrant for #name #ty_generics #where_clause {
            fn register(&mut self, registry: &mut ::fastmetrics::registry::Registry) -> ::core::result::Result<&mut Self, ::fastmetrics::registry::RegistryError> {
                #(#register_calls)*
                ::core::result::Result::Ok(self)
            }
        }
    })
}
