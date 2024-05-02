use crate::common;
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
    let buffer_type_ident = common::buffer_generic(&input.generics)
        .expect("Expected buffer generic")
        .ident();
    quote! {
        impl #impl_generics  crate::traits::Data<#buffer_type_ident> for #ident #ty_generics #where_clause  {
            fn data(&self) -> &[#buffer_type_ident::Unit] {
                use #ident::*;
                match self {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn from_bytes(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::from_bytes(m).into(),
        });
    }
    quote! {
        impl<
                A: crate::buffer::Bytes,
                B: crate::buffer::Ump
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferResize,
            > crate::traits::FromBytes<#ident<A>> for #ident<B>
        {
            fn from_bytes(other: #ident<A>) -> Self {
                match other {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn try_from_bytes(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::try_from_bytes(m)?.into(),
        });
    }
    quote! {
        impl<
                A: crate::buffer::Bytes,
                B: crate::buffer::Ump
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferTryResize,
            > crate::traits::TryFromBytes<#ident<A>> for #ident<B>
        {
            fn try_from_bytes(other: #ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                Ok(match other {
                    #match_arms
                })
            }
        }
    }
    .into()
}

pub fn from_ump(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::from_ump(m).into(),
        });
    }
    quote! {
        impl<
                A: crate::buffer::Ump,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferResize,
            > crate::traits::FromUmp<#ident<A>> for #ident<B>
        {
            fn from_ump(other: #ident<A>) -> Self {
                match other {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn try_from_ump(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::try_from_ump(m)?.into(),
        });
    }
    quote! {
        impl<
                A: crate::buffer::Ump,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferTryResize,
            > crate::traits::TryFromUmp<#ident<A>> for #ident<B>
        {
            fn try_from_ump(other: #ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                Ok(match other {
                    #match_arms
                })
            }
        }
    }
    .into()
}

pub fn rebuffer_from(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::rebuffer_from(m).into(),
        });
    }
    let generics = common::rebuffer_generics(
        match common::buffer_generic(&input.generics).expect("No buffer generic found.") {
            common::BufferGeneric::Ump(_) => common::Representation::Ump,
            common::BufferGeneric::Bytes(_) => common::Representation::Bytes,
            common::BufferGeneric::UmpOrBytes(_) => common::Representation::UmpOrBytes,
        },
    );
    quote! {
        impl #generics crate::traits::RebufferFrom<#ident<A>> for #ident<B>
        {
            fn rebuffer_from(other: #ident<A>) -> Self {
                match other {
                    #match_arms
                }
            }
        }
    }
    .into()
}

pub fn try_rebuffer_from(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        let message_type = message_type_from_variant(variant);
        match_arms.extend(quote! {
            #ident::#variant_ident(m) => #message_type::try_rebuffer_from(m)?.into(),
        });
    }
    let generics = common::try_rebuffer_generics(
        match common::buffer_generic(&input.generics).expect("No buffer generic found.") {
            common::BufferGeneric::Ump(_) => common::Representation::Ump,
            common::BufferGeneric::Bytes(_) => common::Representation::Bytes,
            common::BufferGeneric::UmpOrBytes(_) => common::Representation::UmpOrBytes,
        },
    );
    quote! {
        impl #generics crate::traits::TryRebufferFrom<#ident<A>> for #ident<B>
        {
            fn try_rebuffer_from(other: #ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                Ok(match other {
                    #match_arms
                })
            }
        }
    }
    .into()
}

fn message_type_from_variant(variant: &syn::Variant) -> TokenStream {
    let syn::Fields::Unnamed(fields) = &variant.fields else {
        panic!("Expected enum variant with unnamed fields");
    };
    let Some(syn::Field { ty, .. }) = fields.unnamed.last() else {
        panic!("Expected an unnamed field in the enum variant");
    };
    let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
        panic!("Expected a 'path' type");
    };
    let mut path = path.clone();
    let Some(last_segment) = path.segments.last() else {
        panic!("Expected type to have an ident");
    };
    let syn::PathArguments::AngleBracketed(args) = last_segment.arguments.clone() else {
        panic!("Expected type have generic args");
    };
    path.segments.last_mut().unwrap().arguments = syn::PathArguments::None;
    quote! {
        #path:: #args
    }
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
    quote! {
        impl<B: crate::buffer::Ump> crate::traits::Grouped<B> for #ident<B> {
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
    let buffer_id = common::buffer_generic(&input.generics)
        .expect("Expected buffer generic")
        .ident();
    quote! {
        impl #impl_generics crate::traits::Channeled<#buffer_id> for #ident #ty_generics #where_clause {
            fn channel(&self) -> crate::u4 {
                use #ident::*;
                match self {
                    #match_arms_read
                }
            }
            fn set_channel(&mut self, channel: crate::u4)
            where
                #buffer_id: crate::buffer::BufferMut
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
    let buffer_id = common::buffer_generic(&generics)
        .expect("Expected buffer generic")
        .ident();
    quote! {
        impl #impl_generics core::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}([", stringify!(#ident)))?;
                match <<#buffer_id as crate::buffer::Buffer>::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
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
