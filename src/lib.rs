extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

struct Wrapper {
    func: syn::ItemFn,
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

#[allow(dead_code)]
struct Input {
    ident: syn::Ident,
    mutability: std::option::Option<syn::token::Mut>,
    by_ref: std::option::Option<syn::token::Ref>,
    ty: syn::Ident,
}

impl Input {
    #[allow(dead_code)]
    fn new(
        ident: &syn::Ident,
        mutability: &std::option::Option<syn::token::Mut>,
        by_ref: &std::option::Option<syn::token::Ref>,
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
fn extract_inputs(punc: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> Vec<Input> {
    punc.iter()
        .map(|arg| match *arg {
            syn::FnArg::Typed(syn::PatType {
                ref pat, ref ty, ..
            }) => extract_input(pat, ty),

            _ => unimplemented!(),
        })
        .collect()
}

#[proc_macro_attribute]
pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::Item = syn::parse(item.clone()).unwrap();
    println!("{:#?}", ast);
    let Wrapper { func } = parse_macro_input!(item as Wrapper);
    let name = func.clone().sig.ident;
    let wrap_name = quote::format_ident!("rust_{}", name);
    let inputs = func.clone().sig.inputs;
    let output = func.clone().sig.output;
    let extracted_inputs = extract_inputs(&inputs)
        .iter()
        .map(|input| input.ident.to_owned())
        .collect::<Vec<syn::Ident>>();
    println!("{:#?}", extracted_inputs);
    let wrap = quote! {
    #[no_mangle]
    pub extern "C" fn #wrap_name(#inputs) #output {
        #name(
            #(#extracted_inputs,)*
        )
    }
    };
    let original = quote! { #func };
    (quote! {
        #original
        #wrap
    })
    .into()
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Wrapper {
            func: input.parse()?,
        })
    }
}
