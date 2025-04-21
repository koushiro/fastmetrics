use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_state_set_value(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Ensure we're deriving for an enum
    let data_enum = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "`StateSetValue` can only be derived for enums",
            ))
        },
    };

    // Ensure the enum has at least one variant
    if data_enum.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            "`StateSetValue` requires at least one variant in the enum",
        ));
    }

    // Process all variants, ensuring they are unit variants
    let variants = data_enum
        .variants
        .iter()
        .map(|variant| {
            // Check if this is a unit variant (no fields)
            match &variant.fields {
                syn::Fields::Unit => {},
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "`StateSetValue` can only be derived for enums with unit variants",
                    ))
                },
            }

            let variant_name = &variant.ident;
            let variant_str = variant_name.to_string();

            Ok((
                quote! { #name::#variant_name }, // For variants() method
                quote! { #name::#variant_name => #variant_str }, // For as_str() method
            ))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    // Unzip the variants into separate vectors
    let (variant_list, variant_arms): (Vec<_>, Vec<_>) = variants.into_iter().unzip();

    // Generate the trait implementation
    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::metrics::state_set::StateSetValue for #name #ty_generics #where_clause {
            fn variants() -> &'static [Self] {
                static VARIANTS: &[#name] = &[
                    #(#variant_list,)*
                ];
                VARIANTS
            }

            fn as_str(&self) -> &'static str {
                match self {
                    #(#variant_arms,)*
                }
            }
        }
    };

    Ok(expanded)
}
