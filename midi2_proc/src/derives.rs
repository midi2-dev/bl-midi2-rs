use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum, ItemStruct};

fn enum_lifetime(item: &ItemEnum) -> TokenStream {
    match item.generics.params.first() {
        Some(syn::GenericParam::Lifetime(lifetime)) => {
            quote! { #lifetime }
        }
        _ => TokenStream::new(),
    }
}

fn struct_lifetime(item: &ItemStruct) -> TokenStream {
    match item.generics.params.first() {
        Some(syn::GenericParam::Lifetime(lifetime)) => {
            quote! { #lifetime }
        }
        _ => TokenStream::new(),
    }
}

pub fn data(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms.extend(quote! {
            #variant_ident(m) => m.data(),
        });
    }
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> Data for #ident<#lifetime_param> {
            fn data(&self) -> &[u32] {
                use #ident::*;
                match self {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn grouped(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> Grouped for #ident<#lifetime_param> {}
    }
    .into()
}

pub fn write_byte_data(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms.extend(quote! {
            #variant_ident(m) => m.write_byte_data(buffer),
        });
    }
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> WriteByteData for #ident<#lifetime_param> {
            fn write_byte_data<'b>(&self, buffer: &'b mut [u8]) -> &'b mut [u8] {
                use #ident::*;
                match self {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn channeled(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms.extend(quote! {
            #variant_ident(m) => m.channel(),
        });
    }
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> Channeled for #ident<#lifetime_param> {
            fn channel(&self) -> u4 {
                use #ident::*;
                match self {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn ump_debug(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as Item);
    let ident = match &input {
        Item::Enum(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        _ => panic!("Only enums and structs supported"),
    };
    let lifetime_param = match &input {
        Item::Enum(i) => enum_lifetime(&i),
        Item::Struct(i) => struct_lifetime(&i),
        _ => panic!("Only enums and structs supported"),
    };
    quote! {
        impl<#lifetime_param> core::fmt::Debug for #ident<#lifetime_param> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!(#ident)))?;
                let mut iter = self.data().iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#010X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(", ")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    }
    .into()
}

pub fn bytes_debug(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as Item);
    let ident = match &input {
        Item::Enum(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        _ => panic!("Only enums and structs supported"),
    };
    let lifetime_param = match &input {
        Item::Enum(i) => enum_lifetime(&i),
        Item::Struct(i) => struct_lifetime(&i),
        _ => panic!("Only enums and structs supported"),
    };
    quote! {
        impl<#lifetime_param> core::fmt::Debug for #ident<#lifetime_param> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!(#ident)))?;
                let mut iter = self.byte_data().iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#04X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(", ")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    }
    .into()
}
