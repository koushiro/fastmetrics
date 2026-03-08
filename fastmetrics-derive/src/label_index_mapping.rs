use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Result, parse_quote};

use crate::{label_attributes::LabelAttributes, utils::wrap_in_const};

pub fn expand_derive(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Enum(data) => expand_enum(input, data),
        Data::Struct(data) => expand_struct(input, data),
        _ => {
            let error = "#[derive(LabelIndexMapping)] can only be used for enums or structs with named fields";
            Err(Error::new_spanned(&input.ident, error))
        },
    }
}

fn expand_enum(input: &DeriveInput, data: &syn::DataEnum) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if data.variants.is_empty() {
        let error = "#[derive(LabelIndexMapping)] requires at least one variant in the enum";
        return Err(Error::new_spanned(name, error));
    }

    let cardinality = data.variants.len();
    let index_arms = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Unit => {},
                _ => {
                    let error = "#[derive(LabelIndexMapping)] can only be used for enums with unit variants";
                    return Err(Error::new_spanned(variant, error));
                },
            }

            Ok(quote! {
                #name::#variant_name => #index
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let from_index_arms = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_name = &variant.ident;
            quote! {
                #index => #name::#variant_name
            }
        })
        .collect::<Vec<_>>();

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::metrics::family::LabelIndexMapping
            for #name #ty_generics #where_clause
        {
            const CARDINALITY: usize = #cardinality;

            #[inline]
            fn index(&self) -> usize {
                match self {
                    #(#index_arms,)*
                }
            }

            #[inline]
            fn from_index(index: usize) -> Self {
                match index {
                    #(#from_index_arms,)*
                    _ => panic!("invalid label index"),
                }
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}

fn expand_struct(input: &DeriveInput, data: &syn::DataStruct) -> Result<TokenStream> {
    let name = &input.ident;
    let fields = match &data.fields {
        Fields::Named(FieldsNamed { named, .. }) => named,
        _ => {
            let error =
                "#[derive(LabelIndexMapping)] can only be used for structs with named fields";
            return Err(Error::new_spanned(name, error));
        },
    };

    let parsed_fields = fields
        .iter()
        .map(|field| Ok((field, LabelAttributes::parse(field)?)))
        .collect::<Result<Vec<_>>>()?;

    let indexed_fields = parsed_fields
        .iter()
        .filter_map(|(field, attrs)| if attrs.label.skip { None } else { Some(*field) })
        .collect::<Vec<_>>();
    let skipped_fields = parsed_fields
        .iter()
        .filter_map(|(field, attrs)| if attrs.label.skip { Some(*field) } else { None })
        .collect::<Vec<_>>();

    let mut impl_generics = input.generics.clone();
    let where_clause = impl_generics.make_where_clause();
    for field in &indexed_fields {
        let ty = &field.ty;
        where_clause
            .predicates
            .push(parse_quote!(#ty: ::fastmetrics::metrics::family::LabelIndexMapping));
    }
    for field in &skipped_fields {
        let ty = &field.ty;
        where_clause.predicates.push(parse_quote!(#ty: ::core::default::Default));
    }
    let (impl_generics, ty_generics, where_clause) = impl_generics.split_for_impl();

    let cardinality_factors = indexed_fields.iter().map(|field| {
        let ty = &field.ty;
        quote!(<#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::CARDINALITY)
    });

    let index_stmts = indexed_fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("fields must be named");
        let ty = &field.ty;

        quote! {
            index = index
                * <#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::CARDINALITY
                + <#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::index(&self.#ident);
        }
    });

    let decode_stmts = indexed_fields.iter().rev().map(|field| {
        let ident = field.ident.as_ref().expect("fields must be named");
        let ty = &field.ty;
        let index_ident = format_ident!("__{}_index", ident);

        quote! {
            let #index_ident =
                remaining % <#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::CARDINALITY;
            remaining /= <#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::CARDINALITY;
        }
    });

    let init_fields = parsed_fields.iter().map(|(field, attrs)| {
        let ident = field.ident.as_ref().expect("fields must be named");
        if attrs.label.skip {
            quote! {
                #ident: ::core::default::Default::default()
            }
        } else {
            let ty = &field.ty;
            let index_ident = format_ident!("__{}_index", ident);

            quote! {
                #ident: <#ty as ::fastmetrics::metrics::family::LabelIndexMapping>::from_index(#index_ident)
            }
        }
    });

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::metrics::family::LabelIndexMapping
            for #name #ty_generics #where_clause
        {
            const CARDINALITY: usize = 1usize #(* #cardinality_factors)*;

            #[inline]
            fn index(&self) -> usize {
                let mut index = 0usize;
                #(#index_stmts)*
                index
            }

            #[inline]
            fn from_index(index: usize) -> Self {
                assert!(index < Self::CARDINALITY, "label index out of bounds");
                let mut remaining = index;
                #(#decode_stmts)*
                debug_assert_eq!(remaining, 0, "label index out of bounds");

                Self {
                    #(#init_fields,)*
                }
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
