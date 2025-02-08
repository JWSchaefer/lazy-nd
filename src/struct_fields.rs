use crate::lazy_field::LazyField;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

pub struct StructFields {
    fields: Vec<LazyField>,
}

impl Parse for StructFields {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated =
            Punctuated::<LazyField, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            fields: punctuated.into_iter().collect(),
        })
    }
}
