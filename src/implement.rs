use crate::{
    attribute_field::AttributeField, attribute_info::AttributeInfo,
    quantity::Quantity, struct_info::StructInfo,
};
use proc_macro::TokenStream;
use quote::quote;
use std::{
    collections::{HashMap, HashSet},
    usize,
};
use syn::{spanned::Spanned, Ident, Type};

pub fn inner_implement(
    _attribute: AttributeInfo,
    struct_: StructInfo,
) -> TokenStream {
    let (name, generics, struct_fields, attribute_fields) = struct_.unpack();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inner_name = Ident::new(&format!("{}Inner", name), name.span());

    let attribute_idents: Vec<Ident> =
        attribute_fields.iter().map(|f| f.field.clone()).collect();

    let attribute_types: Vec<Type> =
        attribute_fields.iter().map(|f| f.ty.clone()).collect();

    let unique_types: Vec<Ident> = attribute_types
        .clone()
        .into_iter()
        .map(|t| Ident::new(&format!("{}", quote! {#t}), t.span()))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let inner_fields: Vec<Ident> = attribute_types
        .iter()
        .map(|i| Ident::new(&format!("data_{}", quote!(#i)), i.span()))
        .collect();

    let unique_inner_fields: Vec<Ident> = inner_fields
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut indicies = Vec::<(usize, usize)>::new();

    let mut counts: HashMap<String, (usize, usize)> = unique_types
        .clone()
        .into_iter()
        .map(|key| (format!("{}", quote! {#key}), (0, 0)))
        .collect();

    for a in attribute_fields.clone() {
        let ty = &a.ty;
        let key = format!("{}", quote! {#ty});
        let (mut v, mut s) = &counts.get(&key).unwrap();
        match a.quantity {
            Quantity::Vector => v += 1,
            Quantity::Scalar => s += 1,
        }
        counts.insert(key, (v, s));
        indicies.push((v, s));
    }

    let use_statements = quote! {
        use ndarray::{Array2, ArrayView2, ArrayViewMut2};
    };

    let inner_definition = quote! {
        struct #inner_name #impl_generics #where_clause {
            #(
                #unique_inner_fields : Array2<#unique_types>,
            )*
        }
    };

    let inner_impl = quote! {
        impl #impl_generics #inner_name #ty_generics #where_clause {
            #(
                fn #attribute_idents (&self) {

                }
            )*
        }
    };

    let struct_definition = quote! {
        struct #name #impl_generics #where_clause {
            #(
                #struct_fields,
            )*
            inner : #inner_name #ty_generics
        }
    };

    let struct_impl = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new() {
                println!("Hello, world!");
            }
        }
    };

    let gen = quote! {
        #use_statements
        #inner_definition
        #inner_impl
        #struct_definition
        #struct_impl
    };

    gen.into()
}
