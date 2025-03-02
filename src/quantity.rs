use crate::dim::Dim;
use quote::quote;
use std::fmt;
use syn::{
    bracketed, parenthesized,
    parse::{Lookahead1, Parse, ParseStream},
    token::Paren,
    Error, Ident, LitInt, Result, Token,
};

#[derive(Clone, Debug)]
pub enum Quantity {
    Scalar,
    Vector(Dim),
}

impl Parse for Quantity {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse attribute syntax - #[]
        input.parse::<Token![#]>()?;
        let content;
        bracketed!(content in input);

        // Read ident as either scalar or vector
        let ident: Ident = content.parse()?;

        // If scalar, ensure not further information
        if ident.to_string().as_str() == "scalar" {
            if content.is_empty() {
                Ok(Quantity::Scalar)
            } else {
                Err(Error::new(
                    content.fork().span(),
                    "Unexpected token. Expected `]`",
                ))
            }
        // If vector, check for dimension info
        } else if ident.to_string().as_str() == "vector" {
            let dim: Dim;
            // If no ($DIM) then floats dimension
            if !content.is_empty() && content.peek(Paren) {
                let inner_content;
                parenthesized!(inner_content in content);
                dim = inner_content.parse()?;
            } else {
                dim = Dim::Undefined;
            }
            Ok(Quantity::Vector(dim))
        // If neither, then error
        } else {
            Err(Error::new(ident.span(), "Expected 'scalar' or 'vector'"))
        }
    }
}
