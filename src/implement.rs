// use crate::{
//     attribute_field::AttributeField, attribute_info::AttributeInfo,
//     quantity::Quantity, struct_info::StructInfo,
// };
// use proc_macro::TokenStream;
// use quote::quote;
// use std::collections::{HashMap, HashSet};
// use syn::{spanned::Spanned, Ident, Type};
//
// pub fn inner_implement(
//     attribute: AttributeInfo,
//     struct_: StructInfo,
// ) -> TokenStream {
//     let dim = attribute.ty;
//     let (name, generics, struct_fields, attribute_fields) = struct_.unpack();
//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
//
//     let inner_name = Ident::new(&format!("{}Inner", name), name.span());
//
//     let attribute_idents: Vec<Ident> =
//         attribute_fields.iter().map(|f| f.field.clone()).collect();
//
//     let attribute_types: Vec<Type> =
//         attribute_fields.iter().map(|f| f.ty.clone()).collect();
//
//     let return_types: Vec<Ident> = attribute_fields
//         .iter()
//         .map(|a| match a.quantity {
//             Quantity::Vector => Ident::new("ArrayView2", a.ty.span()),
//             Quantity::Scalar => Ident::new("ArrayView1", a.ty.span()),
//         })
//         .collect();
//
//     let struct_field_names: Vec<Ident> =
//         struct_fields.iter().map(|f| f.field.clone()).collect();
//
//     let unique_types: Vec<Ident> = attribute_types
//         .clone()
//         .into_iter()
//         .map(|t| Ident::new(&format!("{}", quote! {#t}), t.span()))
//         .collect::<HashSet<_>>()
//         .into_iter()
//         .collect();
//
//     // let unique_type_default_names: Vec<Ident> = unique_types
//     //     .clone()
//     //     .iter()
//     //     .map(|i| Ident::new(&format!("default_{}", i), i.span()))
//     //     .collect();
//
//     let inner_fields: Vec<Ident> = attribute_types
//         .iter()
//         .map(|i| Ident::new(&format!("data_{}", quote!(#i)), i.span()))
//         .collect();
//
//     let unique_inner_fields: Vec<Ident> = unique_types
//         .clone()
//         .into_iter()
//         .map(|t| Ident::new(&format!("data_{}", quote! {#t}), t.span()))
//         .collect();
//
//     let mut v = Vec::<usize>::new();
//     let mut s = Vec::<usize>::new();
//     let max_v_by_type: Vec<usize>;
//     let max_s_by_type: Vec<usize>;
//     {
//         let mut counts: HashMap<String, (usize, usize)> = unique_types
//             .clone()
//             .into_iter()
//             .map(|key| (format!("{}", quote! {#key}), (0, 0)))
//             .collect();
//
//         for a in attribute_fields.clone() {
//             let ty = &a.ty;
//             let key = format!("{}", quote! {#ty});
//             let (mut v_, mut s_) = &counts.get(&key).unwrap();
//             v.push(v_);
//             s.push(s_);
//             match a.quantity {
//                 Quantity::Vector => v_ += 1,
//                 Quantity::Scalar => s_ += 1,
//             }
//             counts.insert(key, (v_, s_));
//         }
//
//         let max_by_type: Vec<(usize, usize)> = unique_types
//             .clone()
//             .iter()
//             .map(|t| *counts.get(&format!("{}", quote! {#t})).unwrap())
//             .collect();
//
//         (max_v_by_type, max_s_by_type) = max_by_type.into_iter().unzip();
//     }
//
//     let zipped: Vec<(AttributeField, Ident, usize, usize)> = attribute_fields
//         .clone()
//         .into_iter()
//         .zip(inner_fields.clone())
//         .zip(s)
//         .zip(v)
//         .map(|(((a, f), s), v)| (a, f, s, v))
//         .collect();
//
//     let function_inners: Vec<_> = zipped
//         .clone()
//         .iter()
//         .map(|(a, f, s, v)| match a.quantity {
//             Quantity::Vector => quote! {
//                 let start = #v * #dim + #s;
//                 let end = (#v + 1) * #dim + #s;
//                 self. #f .slice(s![start..end, ..])
//
//             },
//             Quantity::Scalar => quote! {
//                 let row = #v * D + #s;
//                 self. #f .row(row)
//             },
//         })
//         .collect();
//
//     let use_statements = quote! {
//         use ndarray::{s, Array2, ArrayView1, ArrayViewMut1,  ArrayView2, ArrayViewMut2};
//     };
//
//     let inner_definition = quote! {
//         struct #inner_name #impl_generics #where_clause {
//             #(
//                 #unique_inner_fields : Array2<#unique_types>,
//             )*
//         }
//     };
//
//     let inner_impl = quote! {
//         impl #impl_generics #inner_name #ty_generics #where_clause {
//
//             fn new(n:usize ) -> Self {
//                 Self {
//                     #(
//                         #unique_inner_fields : Array2::<#unique_types>::from_elem(
//                             ((#max_v_by_type * #dim) + #max_s_by_type ,n),
//                             #unique_types :: default()
//                         ),
//                     )*
//                 }
//             }
//
//             #(
//                 fn #attribute_idents (&self) -> #return_types<#attribute_types> {
//                     #function_inners
//                 }
//             )*
//         }
//     };
//
//     let struct_definition = quote! {
//         struct #name #impl_generics #where_clause {
//             #(
//                 #struct_fields,
//             )*
//             inner : #inner_name #ty_generics
//         }
//     };
//
//     let struct_impl = quote! {
//         impl #impl_generics #name #ty_generics #where_clause {
//             fn new(n:usize, #( #struct_fields, )*  ) -> Self {
//                 Self {
//                     #(
//                         #struct_field_names : #struct_field_names,
//                     )*
//                     inner : #inner_name ::new(n),
//                 }
//             }
//         }
//     };
//
//     let gen = quote! {
//         #use_statements
//         #inner_definition
//         #inner_impl
//         #struct_definition
//         #struct_impl
//     };
//
//     gen.into()
// }
