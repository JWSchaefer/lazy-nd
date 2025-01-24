mod attribute_field;
mod attribute_info;
mod generic_info;
mod quantity;
mod struct_field;
mod struct_info;

use attribute_info::AttributeInfo;
use generic_info::validate_generics;
use proc_macro::TokenStream;
use quote::quote;
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
    let attr = parse_macro_input!(attr as AttributeInfo);
    let parsed = parse_macro_input!(item as StructInfo);

    let (_name, generics, _struct_fields, _attributed_fields) = parsed.unpack();

    let (_impl_generics, _type_generics, _where_clause) = &generics.split_for_impl();

    if let Err(error) = validate_generics(attr, generics) {
        return error.to_compile_error().into();
    }

    let gen = quote! { struct Test {}};

    gen.into()
}
