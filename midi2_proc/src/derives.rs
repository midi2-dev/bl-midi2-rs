use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum};

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
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics  crate::traits::Data<B> for #ident #ty_generics #where_clause  {
            fn data(&self) -> &[B::Unit] {
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
    let mut match_arms_read = TokenStream::new();
    let mut match_arms_write = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms_read.extend(quote! {
            #variant_ident(m) => m.group(),
        });
        match_arms_write.extend(quote! {
            #variant_ident(m) => m.set_group(group),
        });
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics crate::traits::Grouped<B> for #ident #ty_generics #where_clause {
            fn group(&self) -> crate::u4 {
                use #ident::*;
                match self {
                    #match_arms_read
                }
            }
            fn set_group(&mut self, group: crate::u4)
            where
                B: crate::buffer::BufferMut
            {
                use #ident::*;
                match self {
                    #match_arms_write
                }
            }
        }
    }
    .into()
}

pub fn channeled(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms_read = TokenStream::new();
    let mut match_arms_write = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms_read.extend(quote! {
            #variant_ident(m) => m.channel(),
        });
        match_arms_write.extend(quote! {
            #variant_ident(m) => m.set_channel(channel),
        });
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics crate::traits::Channeled<B> for #ident #ty_generics #where_clause {
            fn channel(&self) -> crate::u4 {
                use #ident::*;
                match self {
                    #match_arms_read
                }
            }
            fn set_channel(&mut self, channel: crate::u4)
            where
                B: crate::buffer::BufferMut
            {
                use #ident::*;
                match self {
                    #match_arms_write
                }
            }
        }
    }
    .into()
}

pub fn debug(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as Item);
    let ident = match &input {
        Item::Enum(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        _ => panic!("Only enums and structs supported"),
    };
    let generics = match &input {
        Item::Enum(i) => &i.generics,
        Item::Struct(i) => &i.generics,
        _ => panic!("Only enums and structs supported"),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics core::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}([", stringify!(#ident)))?;
                match <<B as crate::buffer::Buffer>::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U8 => {
                        let buff = self.0.buffer();
                        let mut iter = buff.specialise_u8().iter().peekable();
                        while let Some(v) = iter.next() {
                            fmt.write_fmt(format_args!("{:#04X}", v))?;
                            if iter.peek().is_some() {
                                fmt.write_str(", ")?;
                            }
                        }
                    }
                    crate::buffer::UNIT_ID_U32 => {
                        let buff = self.0.buffer();
                        let mut iter = buff.specialise_u32().iter().peekable();
                        while let Some(v) = iter.next() {
                            fmt.write_fmt(format_args!("{:#010X}", v))?;
                            if iter.peek().is_some() {
                                fmt.write_str(", ")?;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
                fmt.write_str("])")?;
                Ok(())
            }
        }
    }
    .into()
}
