use crate::quantity::Quantity;

use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Result, Token, Type, Visibility,
};

pub struct LazyField {
    quantity: Option<Quantity>,
    visibility: Option<Visibility>,
    field: Ident,
    ty: Type,
}

impl LazyField {
    pub fn quantity(&self) -> &Option<Quantity> {
        &self.quantity
    }
    pub fn visibility(&self) -> &Option<Visibility> {
        &self.visibility
    }
    pub fn field(&self) -> &Ident {
        &self.field
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl PartialEq for LazyField {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl Parse for LazyField {
    fn parse(input: ParseStream) -> Result<Self> {
        let quantity = match input.parse::<Quantity>() {
            Ok(quantity) => Some(quantity),
            Err(_) => None,
        };
        let visibility: Option<Visibility> = input.parse().ok();
        let field: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;

        Ok(LazyField {
            quantity,
            visibility,
            field,
            ty,
        })
    }
}
