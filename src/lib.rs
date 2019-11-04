extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;

use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

mod func;
mod impls;
mod utils;

struct Wrapper {
    func: syn::ItemFn,
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Wrapper {
            func: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    utils::debug_ast(&item);
    let Wrapper { func } = parse_macro_input!(item as Wrapper);
    impls::impl_func_cwrapper(func)
}
