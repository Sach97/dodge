extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

struct Wrapper {
    // name: syn::Ident,
    // inputs: syn::ParenthesizedGenericArguments,
    // output: syn::ReturnType,
    // block: syn::Block,
    func: syn::ItemFn,
}

#[proc_macro_attribute]
pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let ast: syn::Item = syn::parse(item.clone()).unwrap();
    // println!("{:#?}", ast);
    let Wrapper {
        func
        // name,
        // inputs,
        // output,
        // block,
    } = parse_macro_input!(item as Wrapper);
    let name = func.clone().sig.ident;
    // let call = quote::format_ident!("rust_{}", name);
    let wrap_name = quote::format_ident!("rust_{}", name);
    let inputs = func.clone().sig.inputs;
    let output = func.clone().sig.output;
    let wrap = quote! {
    #[no_mangle]
    pub extern "C" fn #wrap_name(#inputs) #output {
        a + b
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
            // name: input.parse()?,
            // inputs: input.parse()?,
            // output: input.parse()?,
            // block: input.parse()?,
            func: input.parse()?,
        })
    }
}
