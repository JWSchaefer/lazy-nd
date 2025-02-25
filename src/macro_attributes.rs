use crate::dim::Dim;

use proc_macro2::Span;
use syn::{parse::Parse, Error, Ident, LitBool, LitInt, Result, Token};

pub struct MacroAttributes {
    dim: Option<Dim>,
    inner: Option<LitBool>,
}
impl MacroAttributes {
    pub fn dim(&self) -> Option<&Dim> {
        self.dim.as_ref()
    }
    pub fn inner(&self) -> Option<bool> {
        if let Some(inner) = &self.inner {
            return Some(inner.value());
        }
        None
    }
}

impl Parse for MacroAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut dim_assignments = Vec::<Dim>::new();
        let mut inner_assignments = Vec::<LitBool>::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if ident == "dim" {
                dim_assignments.push(input.parse()?);
            } else if ident == "inner" {
                inner_assignments.push(input.parse()?)
            } else {
                return Err(Error::new(
                    input.span(),
                    "Expected `dim` or `inner`.",
                ));
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if dim_assignments.len() > 1 {
            return Err(Error::new(
                input.span(),
                "More than one assignment to `dim`",
            ));
        }

        if inner_assignments.len() > 1 {
            return Err(Error::new(
                input.span(),
                "More than one assignment to `inner`",
            ));
        }

        Ok(Self {
            dim: dim_assignments.into_iter().next(),
            inner: inner_assignments.into_iter().next(),
        })
    }
}
