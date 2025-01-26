use proc_macro2::{Punct, Spacing};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse::ParseStream, Ident, Result, Token, Type, Visibility};

#[allow(dead_code)]
pub struct StructField {
    pub visibility: Visibility,
    pub field: Ident,
    pub ty: Type,
}

impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl StructField {
    pub fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>> {
        let mut attrs = Vec::new();
        while input.peek2(Token![:]) {
            attrs.push(input.call(Self::single_parse_inner)?);
        }
        Ok(attrs)
    }

    fn single_parse_inner(input: ParseStream) -> Result<Self> {
        let visibility: Visibility =
            input.parse().unwrap_or_else(|_| Visibility::Inherited);

        let name: Ident = input.parse()?;

        input.parse::<Token![:]>()?;

        let ty: Type = input.parse()?;

        input.parse::<Token![,]>()?;

        Ok(Self {
            visibility,
            field: name,
            ty,
        })
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.visibility.to_tokens(tokens);
        self.field.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.ty.to_tokens(tokens);
    }
}
