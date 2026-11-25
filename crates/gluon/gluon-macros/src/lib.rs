use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Process)]
pub fn derive_process(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote::quote! {
        impl ::gluon::Process for #name {}
    }
    .into()
}
