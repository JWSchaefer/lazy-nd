use syn::parse::Parse;

struct StructAttrs {
    dim: Ident,
    inner: bool,
}

impl Parse for StructAttrs {}
