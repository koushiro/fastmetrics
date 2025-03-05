use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_encode_label_value(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;

    let body = match input.data {
        syn::Data::Struct(_) => panic!("Can't derive `EncodeLabelValue` for struct."),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let match_arms: TokenStream = variants
                .into_iter()
                .map(|v| {
                    let variant = v.ident;
                    quote! {
                        #name::#variant => encoder.encode_str_value(stringify!(#variant))?,
                    }
                })
                .collect();

            quote! {
                match self {
                    #match_arms
                }
            }
        },
        syn::Data::Union(_) => panic!("Can't derive `EncodeLabelValue` for union."),
    };

    let impl_block = quote! {
        #[automatically_derived]
        impl openmetrics_client::encoder::EncodeLabelValue for #name {
            fn encode(&self, encoder: &mut dyn openmetrics_client::encoder::LabelEncoder) -> std::fmt::Result {

                #body

                Ok(())
            }
        }
    };

    Ok(impl_block)
}
