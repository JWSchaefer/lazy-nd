use std::collections::HashSet;

use crate::lazy_field::LazyField;
use crate::quantity::Quantity;
use crate::{dim::Dim, quantity};
use syn::token::Continue;
use syn::{
    braced,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
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
    pub fn name(&self) -> &Ident {
        &self.name
    }

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

    pub fn attribute_fields(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| f.quantity().is_some())
            .collect()
    }

    pub fn vector_fields(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| matches!(f.quantity(), &Some(Quantity::Vector(_))))
            .collect()
    }

    pub fn scalar_fields(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| matches!(f.quantity(), &Some(Quantity::Scalar)))
            .collect()
    }

    pub fn struct_fields(&self) -> Vec<&LazyField> {
        self.fields()
            .iter()
            .filter(|f| f.quantity().is_none())
            .collect()
    }

    pub fn attribute_idents(&self) -> Vec<&Ident> {
        self.attribute_fields().iter().map(|f| f.field()).collect()
    }

    pub fn attribute_types(&self) -> Vec<&Type> {
        self.attribute_fields().iter().map(|f| f.ty()).collect()
    }

    pub fn return_types(&self) -> Vec<Ident> {
        self.attribute_fields()
            .iter()
            .filter_map(|f| match f.quantity() {
                Some(Quantity::Vector(_)) => {
                    Some(Ident::new("ArrayView2", f.ty().span()))
                }
                Some(Quantity::Scalar) => {
                    Some(Ident::new("ArrayView1", f.ty().span()))
                }
                None => None,
            })
            .collect()
    }

    pub fn unique_attribute_types(&self) -> Vec<Ident> {
        self.attribute_types()
            .into_iter()
            .map(|t| Ident::new(&format!("{}", quote! {#t}), t.span()))
            .collect::<HashSet<_>>()
            .into_iter()
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
