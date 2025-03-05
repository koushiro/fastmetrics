use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_encode_label_set(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let body: TokenStream = match input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|f| {
                    let ident = f.ident.unwrap();
                    let ident_string = ident.to_string();
                    quote! {
                        (&#ident_string, &self.#ident).encode(encoder.label_encoder().as_mut())?;
                    }
                })
                .collect(),
            syn::Fields::Unnamed(_) => panic!("Can't derive `EncodeLabelSet` for tuple struct."),
            syn::Fields::Unit => panic!("Can't derive `EncodeLabelSet` for unit struct."),
        },
        syn::Data::Enum(_) => panic!("Can't derive `EncodeLabelSet` for enum."),
        syn::Data::Union(_) => panic!("Can't derive `EncodeLabelSet` for union."),
    };

    let impl_block = quote! {
        #[automatically_derived]
        impl openmetrics_client::encoder::EncodeLabelSet for #name {
            fn encode(&self, encoder: &mut dyn openmetrics_client::encoder::LabelSetEncoder) -> std::fmt::Result {
                use openmetrics_client::encoder::EncodeLabel;

                #body

                Ok(())
            }
        }
    };

    Ok(impl_block)
}
