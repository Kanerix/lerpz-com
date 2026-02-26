use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, PatIdent, PatType, Type, TypePath, parse_macro_input, parse_quote};

pub fn with_instance_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &input.sig.fn_token,
            "#[with_instance] only supports async handler functions",
        )
        .to_compile_error()
        .into();
    }

    let mut has_original_uri = false;
    let mut existing_uri_ident: Option<syn::Ident> = None;

    for arg in input.sig.inputs.iter() {
        let FnArg::Typed(PatType { pat, ty, .. }) = arg else {
            continue;
        };
        if is_original_uri_type(ty) {
            has_original_uri = true;
            existing_uri_ident = extract_pat_ident(pat);
            break;
        }
    }

    let uri_ident = existing_uri_ident.unwrap_or_else(|| format_ident!("__lerpz_instance_uri"));

    if !has_original_uri {
        input
            .sig
            .inputs
            .push(parse_quote!(axum::extract::OriginalUri(#uri_ident): axum::extract::OriginalUri));
    }

    let block = input.block;

    input.block = Box::new(parse_quote!({
        let __lerpz_result = (async move #block).await;
        __lerpz_result.map_err(|err| err.with_instance(#uri_ident.to_string()))
    }));

    TokenStream::from(quote!(#input))
}

fn is_original_uri_type(ty: &Box<Type>) -> bool {
    let Type::Path(TypePath { path, .. }) = ty.as_ref() else {
        return false;
    };
    path.segments
        .last()
        .map(|seg| seg.ident == "OriginalUri")
        .unwrap_or(false)
}

fn extract_pat_ident(pat: &Box<Pat>) -> Option<syn::Ident> {
    match pat.as_ref() {
        Pat::Ident(PatIdent { ident, .. }) => Some(ident.clone()),
        Pat::Type(PatType { pat, .. }) => extract_pat_ident(pat),
        _ => None,
    }
}
