use std::fmt;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Error, Ident, LitInt, Result, Token,
};

#[derive(Clone)]
pub enum Quantity {
    Scalar,
    Vector(Option<usize>),
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Quantity::Scalar => write!(f, "Scalar"),
            Quantity::Vector(Some(n)) => write!(f, "Vector({n})"),
            Quantity::Vector(None) => write!(f, "Vector(Dim)"),
        }
    }
}

fn get_dim(input: ParseStream) -> Result<Option<usize>> {
    if input.peek(Paren) {
        let content;
        parenthesized!(content in input);
        let dim: usize = content.parse::<LitInt>()?.base10_parse()?;
        return Ok(Some(dim));
    }
    Ok(None)
}

impl Parse for Quantity {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![#]>()?;

        let content;
        bracketed!(content in input);

        let ident: Ident = content.parse()?;

        let quantity = match ident.to_string().as_str() {
            "scalar" => Quantity::Scalar,
            "vector" => Quantity::Vector(get_dim(&content)?),
            _ => {
                return Err(Error::new(
                    ident.span(),
                    "Expected 'scalar' or 'vector'",
                ))
            }
        };

        Ok(quantity)
    }
}
