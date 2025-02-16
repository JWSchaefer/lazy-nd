#![allow(dead_code)]
#![allow(unused)]

mod implement;
mod lazy_field;
mod quantity;
mod struct_attributes;
mod struct_fields;
mod struct_info;

use proc_macro::TokenStream;
use quote::quote;
use struct_attributes::StructAttributes;
use struct_fields::StructFields;
use struct_info::StructInfo;
use syn::parse_macro_input;

/// Parses the following
/// #[lazy_nd(inner = $BOOL, dim = $[GENERIC | USIZE])]
/// struct $STRUCT_NAME<$GENERICS> {
///     #[$QUANTITY([GENERIC | USIZE]?)]
///     $NAME : $TYPE,
/// }

#[proc_macro_attribute]
pub fn lazy_nd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_info = StructInfo::new(
        parse_macro_input!(attr as StructAttributes),
        parse_macro_input!(item as StructFields),
    );

    if let Err(err) = struct_info.validate() {
        return err.to_compile_error().into();
    }

    quote! {}.into()
}
