use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Error,
    Expr::Paren,
    Ident, LitBool, LitFloat, LitInt, LitStr, Result,
};

#[derive(Clone, Debug)]
pub enum Dim {
    Undefined,
    Generic(Ident),
    Literal(usize),
}

impl Parse for Dim {
    fn parse(input: ParseStream) -> Result<Self> {
        let msg = "Expected literal int or generic.";

        if input.peek(LitBool) || input.peek(LitStr) || input.peek(LitFloat) {
            return Err(input.error(msg));
        }

        let dim: Result<LitInt> = input.parse();
        if let Ok(d) = dim {
            return Ok(Dim::Literal(d.base10_parse()?));
        }

        let dim: Result<Ident> = input.parse();
        if let Ok(d) = dim {
            return Ok(Dim::Generic(d));
        }

        Err(input.error(msg))
    }
}

impl PartialEq for Dim {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Dim::Undefined, Dim::Undefined) => true,
            (Dim::Undefined, _) => false,
            (Dim::Generic(i_a), Dim::Generic(i_b)) => i_a == i_b,
            (Dim::Generic(_), _) => false,
            (Dim::Literal(u_a), Dim::Literal(u_b)) => u_a == u_b,
            (Dim::Literal(_), _) => false,
        }
    }
}
