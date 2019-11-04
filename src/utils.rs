use proc_macro::TokenStream;

pub fn debug_ast(item: &TokenStream) {
    let ast: syn::Item = syn::parse(item.clone()).unwrap();
    println!("{:#?}", ast);
}
