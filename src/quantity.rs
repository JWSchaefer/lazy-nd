use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Path, Result,
};

pub enum Quantity {
    Scalar,
    Vector,
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Quantity::Scalar => write!(f, "Scalar"),
            Quantity::Vector => write!(f, "Vector"),
        }
    }
}

impl Parse for Quantity {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: Path = input.parse()?;

        if let Some(ident) = path.get_ident() {
            match ident.to_string().as_str() {
                "scalar" => Ok(Quantity::Scalar),
                "vector" => Ok(Quantity::Vector),
                _ => Err(Error::new(
                    ident.span(),
                    "Expected 'scalar' or 'vector'",
                )),
            }
        } else {
            Err(Error::new(path.span(), "Expected an attribute name"))
        }
    }
}
