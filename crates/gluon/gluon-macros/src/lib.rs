use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(GluonMessage)]
pub fn derive_gluon_message(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote::quote! {
        impl #impl_generics ::gluon::Process
            for #name #ty_generics #where_clause
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
    .into()
}
