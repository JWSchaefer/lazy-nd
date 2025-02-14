#![allow(dead_code)]
#![allow(unused)]

mod implement;
mod lazy_field;
mod quantity;
mod struct_attrs;
mod struct_fields;

use proc_macro::TokenStream;
use quote::quote;
use struct_attrs::StructAttrs;
use struct_fields::StructFields;
use syn::parse_macro_input;

/// Parses the following syntax
/// #[lazy_nd(inner : bool = False, dim = ?)]
/// struct $STRUCT_NAME {
///     #[$QUANTITY(?DIM)]
///     $NAME : $TYPE,
/// }
///
/// #[lazy_nd(dim = D)]
/// impl Test<const D : usize> {
///     #[vector]
///     position : f64,
///     #[scalar]
///     mass : f32,
///     #[vector(5)]
///     other : bool,
///     name : &str,
/// }

#[proc_macro_attribute]
pub fn lazy_nd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as StructAttrs);
    let fields = parse_macro_input!(item as StructFields);
    quote! {}.into()
}
