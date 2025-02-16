use crate::struct_attributes::{Dim, StructAttributes};
use crate::struct_fields::StructFields;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ConstParam, Error, GenericParam, Result, Type, TypePath};

pub struct StructInfo {
    attrs: StructAttributes,
    fields: StructFields,
}

impl StructInfo {
    pub fn new(attrs: StructAttributes, fields: StructFields) -> Self {
        Self { attrs, fields }
    }
    pub fn validate(&self) -> Result<()> {
        // if attrs.dim is Generic {
        //      ensure generic is const usize
        // }
        if let Some(Dim::Generic(macro_generic)) = self.attrs.dim() {
            let matching_generics: Vec<_> = self
                .fields
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
                    self.fields.generics.span(),
                    format!("Failed to match the provided generic `{macro_generic}`. Expected `< .. , const {macro_generic} : usize, .. >`. "),
                    
                ));
            }
        }

        //
        // if any attributed fields do not specify dim {
        //      ensure dim is not None
        // }

        // if attributed fields specifies generic dim {
        //      ensure dim is generic and generics match
        // }

        if let Some(Dim::Generic(macro_generic)) = self.attrs.dim() {}
        Ok(())
    }
}
