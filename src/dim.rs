use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Error,
    Expr::Paren,
    Ident, LitBool, LitInt, Result,
};

#[derive(Clone)]
pub enum Dim {
    Undefined,
    Generic(Ident),
    Literal(usize),
}

impl Parse for Dim {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitBool) {
            return Err(input.error(
                "Unexpected token. Expected usize literal or generic.",
            ));
        }
        let dim: Result<LitInt> = input.parse();
        if let Ok(d) = dim {
            return Ok(Dim::Literal(d.base10_parse()?));
        }

        let dim: Result<Ident> = input.parse();
        if let Ok(d) = dim {
            return Ok(Dim::Generic(d));
        }

        Err(Error::new(
            input.fork().span(),
            "Unexpected token. Expected usize literal or generic.",
        ))
    }
}
