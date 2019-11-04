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
