use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, Result, Token,
};

pub struct AttributeInfo {
    pub ty: Ident,
}

impl Parse for AttributeInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(Error::new(input.span(), "Missing token `dim`."));
        }

        let name: Ident = input.parse()?;

        if &name.to_string() != "dim" {
            return Err(Error::new(
                name.span(),
                format!("Unexpected token `{}`.", name.to_string()),
            ));
        }

        input.parse::<Token![=]>()?;

        let ty: Ident = input.parse()?;

        Ok(Self { ty })
    }
}
