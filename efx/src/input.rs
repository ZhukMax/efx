use syn::{Expr, LitStr};

pub(crate) struct EfxInput {
    pub(crate) ui: Expr,
    pub(crate) template: LitStr,
}

impl syn::parse::Parse for EfxInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ui = input.parse::<Expr>()?;
        input.parse::<syn::Token![,]>()?;
        let template = input.parse::<LitStr>()?;
        Ok(EfxInput { ui, template })
    }
}
