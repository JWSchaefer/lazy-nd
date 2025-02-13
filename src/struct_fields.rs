use crate::lazy_field::LazyField;
use syn::{
    braced,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    Generics, Ident, Result, Token,
};

pub struct StructFields {
    name: Ident,
    generics: Generics,
    pub fields: Vec<LazyField>,
}

impl Parse for StructFields {
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
