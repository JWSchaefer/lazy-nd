use crate::quantity::Quantity;

use quote::ToTokens;
use syn::{
    bracketed, parenthesized, parse::ParseStream, Ident, Result, Token, Type,
};

#[allow(dead_code)]
pub struct AttributeField {
    pub quantity: Quantity,
    pub field: Ident,
    pub ty: Type,
}

impl AttributeField {
    pub fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>> {
        let mut attrs = Vec::new();
        while input.peek(Token![#]) {
            attrs.push(input.call(Self::single_parse_inner)?);
        }
        Ok(attrs)
    }

    fn single_parse_inner(input: ParseStream) -> Result<Self> {
        input.parse::<Token![#]>()?;

        let content;
        bracketed!(content in input);

        let quantity: Quantity = content.parse()?;

        let inner_content;
        parenthesized!(inner_content in content);

        let field: Ident = inner_content.parse()?;

        inner_content.parse::<Token![:]>()?;

        let ty: Type = inner_content.parse()?;

        Ok(Self {
            quantity,
            field,
            ty,
        })
    }
}
