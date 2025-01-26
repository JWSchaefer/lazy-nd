use crate::{attribute_info::AttributeInfo, struct_info::StructInfo};
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn implement(_attribute: AttributeInfo, struct_: StructInfo) -> TokenStream {
    let (name, generics, struct_fields, attribute_fields) = struct_.unpack();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inner_name = Ident::new(&format!("{}Inner", name), name.span());

    let inner_definition = quote! {
        struct #inner_name #impl_generics #where_clause {}
    };

    let inner_impl = quote! {};

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
        #inner_impl #inner_definition #struct_definition #struct_impl
    };
    gen.into()
}
