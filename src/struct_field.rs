use syn::{parse::ParseStream, Ident, Result, Token, Type, Visibility};

#[allow(dead_code)]
pub struct StructField {
    pub visibility: Visibility,
    pub field: Ident,
    pub ty: Type,
}

impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.field.to_string() == other.field.to_string()
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
