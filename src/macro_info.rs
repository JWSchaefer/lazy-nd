use std::io::stderr;

use crate::dim::Dim;
use crate::lazy_field::LazyField;
use crate::macro_attributes::MacroAttributes;
use crate::quantity::{self, Quantity};
use crate::struct_info::{self, StructInfo};

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ConstParam, Error, GenericParam, Ident, Result, Type, TypePath};
pub struct MacroInfo {
    attrs: MacroAttributes,
    struct_info: StructInfo,
}

impl MacroInfo {
    pub fn new(attrs: MacroAttributes, struct_info: StructInfo) -> Self {
        Self { attrs, struct_info }
    }

    fn validate_macro_matches_struct(&self) -> Result<()> {
        if let Some(dim) = self.attrs.dim() {
            if matches!(dim, Dim::Generic(_))
                && !self.struct_info.check_const_generic(dim)
            {
                return Err(Error::new(
                    self.struct_info.generics().span(),
                    "Failed to match the provided generic.",
                ));
            }
        }
        Ok(())
    }

    fn validate_floating_fields(&self) -> Result<()> {
        let macro_dim_not_given: bool = self.attrs.dim().is_none();

        let floating_fields: Vec<&LazyField> =
            self.struct_info.fields_floating();

        if !floating_fields.is_empty() && macro_dim_not_given {
            return  Err(Error::new(
                floating_fields.first().unwrap().field().span(),
                "At least one field does not specify its dimension. Please either use the macro attribute `dim` or specify the dimension of all vectors"
            ));
        }
        Ok(())
    }

    fn validate_field_generic_dim(&self) -> Result<()> {
        let msg = "At least one field specifies a generic dimension that is undefined. Please either use the macro attribute `dim` to specify struct generics.";

        for field in self.struct_info.fields_generic() {
            if let Some(Quantity::Vector(dim)) = field.quantity() {
                if !self.struct_info.check_const_generic(dim) {
                    return Err(Error::new(field.field().span(), msg));
                }
            }
        }

        Ok(())
    }

    // fn validate_macro_struct
    //
    pub fn validate(&self) -> Result<()> {
        // If attrs.dim is a Generic then ensure it is given in struct definition
        self.validate_macro_matches_struct()?;

        // If any attributed fields do not specify dim then ensure dim is given
        self.validate_floating_fields()?;

        self.validate_field_generic_dim()?;

        // if attributed fields specifies generic dim {
        //      ensure dim is generic and generics match
        // }

        Ok(())
    }
}
