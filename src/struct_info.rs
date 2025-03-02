use crate::dim::Dim;
use crate::lazy_field::LazyField;
use crate::quantity::Quantity;
use syn::{
    braced,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    ConstParam, Error, GenericParam, Generics, Ident, Result, Token, Type,
    TypePath,
};

use quote::quote;

pub struct StructInfo {
    name: Ident,
    generics: Generics,
    fields: Vec<LazyField>,
}

impl Parse for StructInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![struct] = input.parse()?;
        let name: Ident = input.parse()?;
        let generics: Generics = input.parse()?;

        let content;
        braced!(content in input);

        let fields: Vec<LazyField> =
            Punctuated::<LazyField, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();

        Ok(Self {
            name,
            generics,
            fields,
        })
    }
}

impl StructInfo {
    pub fn generics(&self) -> &Generics {
        &self.generics
    }

    pub fn fields(&self) -> &Vec<LazyField> {
        &self.fields
    }

    pub fn fields_floating(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| {
                matches!(f.quantity(), Some(Quantity::Vector(Dim::Undefined)))
            })
            .collect()
    }

    pub fn fields_generic(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| {
                matches!(f.quantity(), Some(Quantity::Vector(Dim::Generic(_))))
            })
            .collect()
    }

    pub fn check_const_generic(&self, dim: &Dim) -> bool {
        let id: &Ident;

        if let Dim::Generic(_id) = dim {
            id = _id;
        } else {
            return false;
        }

        /// Checks if the provided field matches any provided generics  
        let dtype: &str = "usize";
        let matches: Vec<(&Ident, &Type)> = self
            .generics
            .params
            .iter()
            .filter_map(|g| {
                if let GenericParam::Const(ConstParam { ident, ty, .. }) = g {
                    if let Type::Path(TypePath { path, .. }) = ty {
                        println!("{} - {}", quote! {#ident}, quote! {#id});
                        if path.is_ident(dtype) && ident == id {
                            return Some((ident, ty));
                        }
                    }
                }
                None
            })
            .collect();

        !matches.is_empty()
    }
}
