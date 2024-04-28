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

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(item: TokenStream1) -> TokenStream1 {
    derives::from_bytes(item)
}

#[proc_macro_derive(FromUmp)]
pub fn derive_from_ump(item: TokenStream1) -> TokenStream1 {
    derives::from_ump(item)
}

#[proc_macro_derive(TryFromBytes)]
pub fn derive_try_from_bytes(item: TokenStream1) -> TokenStream1 {
    derives::try_from_bytes(item)
}

#[proc_macro_derive(TryFromUmp)]
pub fn derive_try_from_ump(item: TokenStream1) -> TokenStream1 {
    derives::try_from_ump(item)
}

#[proc_macro_derive(RebufferFrom)]
pub fn derive_rebuffer_from(item: TokenStream1) -> TokenStream1 {
    derives::rebuffer_from(item)
}

#[proc_macro_derive(TryRebufferFrom)]
pub fn derive_try_rebuffer_from(item: TokenStream1) -> TokenStream1 {
    derives::try_rebuffer_from(item)
}
