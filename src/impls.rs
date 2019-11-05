use crate::func::extract_inputs;
use proc_macro::TokenStream;
use quote::quote;

pub fn impl_func_cwrapper(func: syn::ItemFn) -> TokenStream {
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
    [no_mangle]
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::quote::ToTokens;

    #[test]
    fn test_build_func_original() {
        let source = quote! {  fn add(a: i32, b: i32) -> i32 {
             a + b
        }};
        let expected = quote! {[ no_mangle ] pub extern "C" fn rust_add ( a : i32 , b : i32 ) -> i32 { add ( a , b , ) }};
        let input = syn::parse_quote::parse(source);
        assert_eq!(
            build_func_wrap(&input).to_string(),
            quote! {#expected}.to_string()
        );
    }
}
