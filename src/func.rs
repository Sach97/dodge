#[allow(dead_code)]
pub struct Input {
    pub ident: syn::Ident,
    mutability: Option<syn::token::Mut>,
    by_ref: Option<syn::token::Ref>,
    pub ty: syn::Ident,
}

impl Input {
    #[allow(dead_code)]
    fn new(
        ident: &syn::Ident,
        mutability: &Option<syn::token::Mut>,
        by_ref: &Option<syn::token::Ref>,
        ty: &syn::Ident,
    ) -> Self {
        Input {
            ident: ident.to_owned(),
            mutability: mutability.to_owned(),
            by_ref: by_ref.to_owned(),
            ty: ty.to_owned(),
        }
    }
}

#[allow(dead_code)]
pub fn extract_inputs(
    punc: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Vec<Input> {
    punc.iter()
        .map(|arg| match *arg {
            syn::FnArg::Typed(syn::PatType {
                ref pat, ref ty, ..
            }) => extract_input(pat, ty),

            _ => unimplemented!(),
        })
        .collect()
}

#[allow(dead_code)]
fn extract_input(pat: &syn::Pat, ty: &Box<syn::Type>) -> Input {
    let (ident, mutability, by_ref) = match &*pat {
        syn::Pat::Ident(syn::PatIdent {
            by_ref,
            mutability,
            ident,
            ..
        }) => (ident, mutability, by_ref),
        _ => unimplemented!(),
    };
    Input::new(ident, mutability, by_ref, extract_type(ty))
}

#[allow(dead_code)]
fn extract_type(ty: &std::boxed::Box<syn::Type>) -> &syn::Ident {
    match &**ty {
        syn::Type::Path(syn::TypePath { path, .. }) => extract_path(path),
        _ => unimplemented!(),
    }
}

#[allow(dead_code)]
fn extract_path(path: &syn::Path) -> &syn::Ident {
    &path.segments.first().unwrap().ident
}
