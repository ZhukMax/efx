use crate::tags::util::*;
use efx_core::Element;
use quote::{ToTokens, quote};

pub fn render_scroll_area_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &[
        "axis",
        "always_show",
        "max_height",
        "max_width",
        "id",              // source of state identifier (optional)
        "stick_to_bottom", // true|false
        "stick_to_right",  // true|false
    ];

    let map = match attr_map(el, KNOWN, "ScrollArea") {
        Ok(m) => m,
        Err(err) => return err,
    };

    let axis_src = map.get("axis").copied().unwrap_or("vertical");
    let axis_ctor = match axis_src {
        "vertical" => quote!(egui::ScrollArea::vertical()),
        "horizontal" => quote!(egui::ScrollArea::horizontal()),
        "both" => quote!(egui::ScrollArea::both()),
        other => {
            let msg = format!(
                "efx: <ScrollArea> attribute `axis` must be one of vertical|horizontal|both, got `{}`",
                other
            );
            return quote! { compile_error!(#msg); };
        }
    };

    let always_show = bool_opt(&map, "always_show").unwrap_or(None);
    let stick_to_bottom = bool_opt(&map, "stick_to_bottom").unwrap_or(None);
    let stick_to_right = bool_opt(&map, "stick_to_right").unwrap_or(None);
    let max_h = f32_opt(&map, "max_height").unwrap_or(None);
    let max_w = f32_opt(&map, "max_width").unwrap_or(None);
    let id_src = map.get("id").map(|s| (*s).to_string());

    let children_ts = render_children_stmt(&quote!(ui), &el.children);

    let mut build = quote!( let mut __efx_sa = #axis_ctor; );
    if let Some(b) = always_show {
        build.extend(quote!( __efx_sa = __efx_sa.always_show_scroll(#b); ));
    }
    if let Some(b) = stick_to_bottom {
        build.extend(quote!( __efx_sa = __efx_sa.stick_to_bottom(#b);   ));
    }
    if let Some(b) = stick_to_right {
        build.extend(quote!( __efx_sa = __efx_sa.stick_to_right(#b);    ));
    }
    if let Some(h) = max_h {
        build.extend(quote!( __efx_sa = __efx_sa.max_height(#h as _);   ));
    }
    if let Some(w) = max_w {
        build.extend(quote!( __efx_sa = __efx_sa.max_width(#w as _);    ));
    }
    if let Some(id) = id_src {
        build.extend(quote!( __efx_sa = __efx_sa.id_source(#id);        ));
    }

    quote! {{
        #build
        let _ = __efx_sa.show(#ui, |ui| { #children_ts });
    }}
}
