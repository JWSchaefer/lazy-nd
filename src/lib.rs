mod attribute_field;
mod attribute_info;
mod generic_info;
mod implement;
mod quantity;
mod struct_field;
mod struct_info;

use attribute_info::AttributeInfo;
use generic_info::validate_generics;
use implement::implement;
use proc_macro::TokenStream;
use struct_info::StructInfo;
use syn::parse_macro_input;

/// Parses the following syntax
/// #[lazy_nd(dim = $DIM)]
/// struct $STRUCT_NAME {
///     $(
///         #[$ATTR($FIELD : $TY)]
///     )*
///     $VISABILITY $NAME : $TYPE,
/// }
///
/// #[lazy_nd(dim = D)]
/// impl Test<const D : usize> {
///     #[scalar(position : f64)]
///     #[vector(velocity : f64)]
///     #[vector(acceleration : f64)]
///     #[scalar(mass: f64)]
///     #[scalar(id : u32)]
///     name : &str
/// }

#[proc_macro_attribute]
pub fn lazy_nd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_info = parse_macro_input!(attr as AttributeInfo);
    let struct_info = parse_macro_input!(item as StructInfo);

    if let Err(error) = validate_generics(&attr_info, &struct_info.generics()) {
        return error.to_compile_error().into();
    }

    implement(attr_info, struct_info)
}
