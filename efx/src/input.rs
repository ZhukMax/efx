use syn::{Expr, LitStr, Token};

pub(crate) struct EfxInput {
    pub(crate) ui: Expr,
    pub(crate) template: LitStr,
}

impl syn::parse::Parse for EfxInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ui = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let template = input.parse::<LitStr>()?;
        Ok(EfxInput { ui, template })
    }
}

pub(crate) struct EfxCtxInput {
    pub(crate) ctx: Expr,
    _comma: Token![,],
    pub(crate) template: LitStr,
}

impl syn::parse::Parse for EfxCtxInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ctx = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let template = input.parse::<LitStr>()?;
        Ok(EfxCtxInput { ctx, _comma: input.parse()?, template })
    }
}
