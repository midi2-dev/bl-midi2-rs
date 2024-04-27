use proc_macro::TokenStream as TokenStream1;

mod derives;
mod generate_message;

#[proc_macro_attribute]
pub fn generate_message(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    generate_message::generate_message(attrs, item)
}

#[proc_macro_derive(Data)]
pub fn derive_data(item: TokenStream1) -> TokenStream1 {
    derives::data(item)
}

#[proc_macro_derive(Grouped)]
pub fn derive_grouped(item: TokenStream1) -> TokenStream1 {
    derives::grouped(item)
}

#[proc_macro_derive(Channeled)]
pub fn derive_channeled(item: TokenStream1) -> TokenStream1 {
    derives::channeled(item)
}

#[proc_macro_derive(Debug)]
pub fn derive_ump_debug(item: TokenStream1) -> TokenStream1 {
    derives::debug(item)
}
