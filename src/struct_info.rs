use crate::attribute_field::AttributeField;
use crate::struct_field::StructField;

use syn::{
    braced,
    parse::{Parse, ParseStream},
    Error, Generics, Ident, Result, Token,
};

use proc_macro2::Span;

use std::collections::HashMap;

pub struct StructInfo {
    name: Ident,
    generics: Generics,
    struct_fields: Vec<StructField>,
    attribute_fields: Vec<AttributeField>,
}

impl StructInfo {
    pub fn unpack(
        self,
    ) -> (Ident, Generics, Vec<StructField>, Vec<AttributeField>) {
        (
            self.name,
            self.generics,
            self.struct_fields,
            self.attribute_fields,
        )
    }
}

fn filter_duplicates(fields: Vec<(String, Span)>) -> Vec<(String, Span)> {
    let mut map = HashMap::<String, Span>::new();
    let mut out = Vec::<(String, Span)>::new();

    for (f, s) in fields {
        if !map.contains_key(&f) {
            map.insert(f, s);
        } else {
            out.push((f, s));
        }
    }

    out
}

fn validate_unique_fields(
    struct_fields: &Vec<StructField>,
    attribute_fields: &Vec<AttributeField>,
) -> Result<()> {
    let mut struct_fields: Vec<(String, Span)> = struct_fields
        .iter()
        .map(|f| (f.field.to_string(), f.field.span()))
        .collect();

    let mut attribute_fields: Vec<(String, Span)> = attribute_fields
        .iter()
        .map(|f| (f.field.to_string(), f.field.span()))
        .collect();

    attribute_fields.append(&mut struct_fields);

    let duplicate_fields = filter_duplicates(attribute_fields);

    if !duplicate_fields.is_empty() {
        let (f, s) = duplicate_fields.last().unwrap();
        return Err(Error::new(
            *s,
            format!("field `{}` already declared", f).to_string(),
        ));
    }

    Ok(())
}

impl Parse for StructInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![struct]>()?;

        let name: Ident = input.parse()?;

        let generics: Generics = input.parse()?;

        let content;
        braced!(content in input);

        let attrs = AttributeField::parse_inner(&content)?;
        let fields = StructField::parse_inner(&content)?;

        validate_unique_fields(&fields, &attrs)?;

        Ok(Self {
            name,
            generics,
            struct_fields: fields,
            attribute_fields: attrs,
        })
    }
}
