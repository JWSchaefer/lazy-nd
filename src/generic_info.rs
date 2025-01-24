use syn::{spanned::Spanned, ConstParam, Error, Generics, Result};

use crate::attribute_info::AttributeInfo;

pub fn validate_generics(
    attr: AttributeInfo,
    generics: Generics,
) -> Result<()> {
    let correct_params: Vec<&ConstParam> = generics
        .const_params()
        .filter(|&p| p.ident.to_string() == attr.ty.to_string())
        .collect();

    if correct_params.len() < 1 {
        return Err(Error::new(
            generics.span(),
            format!(
                "Expected generic `<const {} : usize>`",
                attr.ty.to_string()
            ),
        ));
    }

    if correct_params.len() > 1 {
        return Err(Error::new(
            generics.const_params().last().span(),
            format!(
                "Generic `<const {} : usize>` already declared.",
                attr.ty.to_string()
            ),
        ));
    }

    let mut names: Vec<String> = generics
        .type_params()
        .into_iter()
        .map(|p| p.ident.to_string())
        .collect();

    let mut const_param_names: Vec<String> = generics
        .const_params()
        .map(|p| p.ident.to_string())
        .collect();

    names.append(&mut const_param_names);

    let matching_names: Vec<String> = names
        .iter()
        .filter(|&n| *n == attr.ty.to_string())
        .map(|n| n.clone())
        .collect();

    if matching_names.len() > 1 {
        return Err(Error::new(
            generics.span(),
            format!("Generic `{}` already declared.", attr.ty.to_string()),
        ));
    }

    Ok(())
}
