use crate::macro_attributes::{Dim, MacroAttributes};
use crate::quantity::{self, Quantity};
use crate::struct_info::StructInfo;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ConstParam, Error, GenericParam, Result, Type, TypePath};

pub struct MacroInfo {
    attrs: MacroAttributes,
    struct_info: StructInfo,
}

impl MacroInfo {
    pub fn new(attrs: MacroAttributes, struct_info: StructInfo) -> Self {
        Self { attrs, struct_info }
    }
    pub fn validate(&self) -> Result<()> {
        // If attrs.dim is a Generic then ensure it is specified
        if let Some(Dim::Generic(macro_generic)) = self.attrs.dim() {
            // Filter for matching const generics of type usize
            let matching_generics: Vec<_> = self
                .struct_info
                .generics
                .params
                .iter()
                .filter_map(|g| {
                    if let GenericParam::Const(ConstParam {
                        ident, ty, ..
                    }) = g
                    {
                        if let Type::Path(TypePath { path, .. }) = ty {
                            if path.is_ident("usize") && ident == macro_generic
                            {
                                return Some((ident, ty));
                            }
                        }
                    }
                    None
                })
                .collect();

            if matching_generics.is_empty() {
                return Err(Error::new(
                    self.struct_info.generics.span(),
                    format!("Failed to match the provided generic `{macro_generic}`. Expected `< .. , const {macro_generic} : usize, .. >`. "),
                ));
            }
        }

        // if any attributed fields do not specify dim {
        //      ensure dim given
        // }

        let fields_float_dim: bool = !self
            .struct_info
            .fields
            .iter()
            .map(|f| f.quantity().is_none())
            .collect::<Vec<bool>>()
            .is_empty();

        let dim_not_given: bool = self.attrs.dim().is_none();

        if fields_float_dim && dim_not_given {
            return Err(Error::new(
                    self.struct_info.name.span(),
                    "At least one field does not specify its dimension. Please either use the macro attribute `dim` or specify the dimension of all vectors",
                ));
        }

        // if attributed fields specifies generic dim {
        //      ensure dim is generic and generics match
        // }

        if let Some(Dim::Generic(macro_generic)) = self.attrs.dim() {}
        Ok(())
    }
}
