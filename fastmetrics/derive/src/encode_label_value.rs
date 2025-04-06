use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_encode_label_value(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Ensure we're deriving for an enum
    let data_enum = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "`EncodeLabelValue` can only be derived for enums",
            ))
        },
    };

    // Ensure the enum has at least one variant
    if data_enum.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            "`EncodeLabelValue` requires at least one variant in the enum",
        ));
    }

    // Generate match arms for each variant
    let variant_arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            // Check that this is a unit variant (no fields)
            match &variant.fields {
                syn::Fields::Unit => {},
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "`EncodeLabelValue` can only be derived for enums with unit variants",
                    ))
                },
            }

            // The string representation is the variant name
            let variant_str = variant_name.to_string();

            Ok(quote! {
                #name::#variant_name => encoder.encode_str_value(&#variant_str)?
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    // Generate the trait implementation
    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelValue for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelEncoder) -> ::std::fmt::Result {
                match self {
                    #(#variant_arms,)*
                }
                Ok(())
            }
        }
    };

    Ok(expanded)
}
