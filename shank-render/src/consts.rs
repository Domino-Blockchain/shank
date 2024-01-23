use proc_macro2::TokenStream;
use quote::quote;

pub fn domichain_program_pubkey() -> TokenStream {
    quote! { ::domichain_program::pubkey::Pubkey }
}
