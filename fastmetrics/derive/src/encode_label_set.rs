use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_encode_label_set(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Ensure we're deriving for a struct with named fields
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "`EncodeLabelSet` can only be derived for structs with named fields.",
                ))
            },
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "`EncodeLabelSet` can only be derived for structs.",
            ))
        },
    };

    // Process all fields
    let field_list = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let ident_str = ident.to_string();
            quote! {
                encoder.encode(&(#ident_str, &self.#ident))?
            }
        })
        .collect::<Vec<_>>();

    let is_empty = field_list.is_empty();

    // Generate the trait implementation
    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelSet for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelSetEncoder) -> ::std::fmt::Result {
                use ::fastmetrics::encoder::EncodeLabel;

                #(#field_list;)*

                Ok(())
            }

            #[inline]
            fn is_empty(&self) -> bool {
                #is_empty
            }
        }
    };

    Ok(expanded)
}
