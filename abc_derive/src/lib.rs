use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Abc)]
pub fn abc_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let attrs = &derive_input.attrs;

    (quote! {
        fn print_all_attributes() {
            println!(
                stringify!(#( #attrs )*)
            );
        }
    })
    .into()
}
