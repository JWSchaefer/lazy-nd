#![allow(dead_code)]
#![allow(unused)]

mod implement;
mod lazy_field;
mod macro_attributes;
mod macro_info;
mod quantity;
mod struct_info;

use macro_attributes::MacroAttributes;
use macro_info::MacroInfo;
use proc_macro::TokenStream;
use quote::quote;
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
    let struct_info = MacroInfo::new(
        parse_macro_input!(attr as MacroAttributes),
        parse_macro_input!(item as StructInfo),
    );

    if let Err(err) = struct_info.validate() {
        return err.to_compile_error().into();
    }

    quote! {}.into()
}
