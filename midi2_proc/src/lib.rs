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

#[proc_macro_derive(WriteByteData)]
pub fn derive_write_byte_data(item: TokenStream1) -> TokenStream1 {
    derives::write_byte_data(item)
}

#[proc_macro_derive(Channeled)]
pub fn derive_channeled(item: TokenStream1) -> TokenStream1 {
    derives::channeled(item)
}

#[proc_macro_derive(UmpDebug)]
pub fn derive_ump_debug(item: TokenStream1) -> TokenStream1 {
    derives::ump_debug(item)
}

#[proc_macro_derive(BytesDebug)]
pub fn derive_bytes_debug(item: TokenStream1) -> TokenStream1 {
    derives::bytes_debug(item)
}
