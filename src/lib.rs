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

fn debug_item(item: &TokenStream) {
    let ast: syn::Item = syn::parse(item.clone()).unwrap();
    println!("{:#?}", ast);
}

#[proc_macro_attribute]
pub fn func_wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    debug_item(&item);
    let Wrapper { func } = parse_macro_input!(item as Wrapper);
    impl_func_wrapper(func)
}

fn impl_func_wrapper(func: syn::ItemFn) -> TokenStream {
    let wrapped = build_func_wrap(&func);
    let original = build_func_original(&func);
    (quote! {
        #original
        #wrapped
    })
    .into()
}

fn build_func_wrap(func: &syn::ItemFn) -> quote::__rt::TokenStream {
    let sig = func.clone().sig;
    let name = sig.ident;
    let inputs = sig.inputs;
    let output = sig.output;
    let extracted_inputs = extract_inputs(&inputs)
        .iter()
        .map(|input| input.ident.to_owned())
        .collect::<Vec<syn::Ident>>();

    let wrap_name = quote::format_ident!("rust_{}", name);
    let wrapped = quote! {
    #[no_mangle]
    pub extern "C" fn #wrap_name(#inputs) #output {
        #name(
            #(#extracted_inputs,)*
        )
    }
    };
    wrapped
}

fn build_func_original(func: &syn::ItemFn) -> quote::__rt::TokenStream {
    let original = quote! { #func };
    original
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Wrapper {
            func: input.parse()?,
        })
    }
}
