use syn::{
    parse::Parse, punctuated::Punctuated, Error, Ident, LitInt, Result, Token,
};

pub enum Dim {
    Generic(Ident),
    Explicit(usize),
}

impl Parse for Dim {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(Dim::Generic)
        } else if lookahead.peek(LitInt) {
            input.parse::<LitInt>()?.base10_parse().map(Dim::Explicit)
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct Assignment<T>
where
    T: Parse,
{
    field: Ident,
    value: T,
}

impl<T> Parse for Assignment<T>
where
    T: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let field: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: T = input.parse()?;
        Ok(Self { field, value })
    }
}

pub struct StructAttrs {
    dim: Option<Assignment<Dim>>,
    inner: Option<Assignment<LitInt>>,
}

impl Parse for StructAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dim_assignments: Vec<Assignment<Dim>> =
            Punctuated::<Assignment<Dim>, Token![,]>::parse_terminated(&input)?
                .into_iter()
                .collect();

        let inner_assignments: Vec<Assignment<LitInt>> =
            Punctuated::<Assignment<LitInt>, Token![,]>::parse_terminated(
                &input,
            )?
            .into_iter()
            .collect();

        if dim_assignments.len() > 1 {
            return Err(Error::new(
                dim_assignments.into_iter().last().unwrap().field.span(),
                "More than one assignment to `dim`",
            ));
        }

        if inner_assignments.len() > 1 {
            return Err(Error::new(
                inner_assignments.into_iter().last().unwrap().field.span(),
                "More than one assignment to `inner`",
            ));
        }

        Ok(Self {
            dim: dim_assignments.into_iter().next(),
            inner: inner_assignments.into_iter().next(),
        })
    }
}
