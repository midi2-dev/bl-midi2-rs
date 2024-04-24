use crate::common;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use std::string::ToString;
use syn::{parse_macro_input, Ident, ItemStruct, Type};

struct Property {
    ident: Ident,
    meta_type: TokenStream,
    ty: Type,
    builder: bool,
    message: bool,
}

fn imports() -> TokenStream {
    quote! {
        use crate::ci::CiProperty as CiPropertyGenCiProc;
    }
}

fn has_attr(field: &syn::Field, id: &str) -> bool {
    field.attrs.iter().any(|attr| {
        let syn::Meta::Path(path) = &attr.meta else {
            return false;
        };
        path.segments
            .last()
            .iter()
            .any(|&segment| segment.ident.to_string() == id)
    })
}

fn meta_type(field: &syn::Field) -> TokenStream {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            use syn::Meta::*;
            match &attr.meta {
                List(list) => Some(list),
                _ => None,
            }
        })
        .find(|list| {
            list.path
                .segments
                .last()
                .iter()
                .any(|&segment| segment.ident.to_string() == "meta_type")
        })
        .map(|list| list.tokens.clone())
        .expect("fields must be annotated with the meta_type attribute")
}

fn properties(input: &ItemStruct) -> Vec<Property> {
    let syn::Fields::Named(named_fields) = &input.fields else {
        panic!("Expected struct with named fields");
    };
    named_fields
        .named
        .iter()
        .map(|field| Property {
            ident: field
                .ident
                .as_ref()
                .expect("Named fields should have a name")
                .clone(),
            ty: field.ty.clone(),
            meta_type: meta_type(field),
            builder: has_attr(field, "builder"),
            message: has_attr(field, "message"),
        })
        .collect()
}

fn message_type(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    quote! {
        pub struct #ident<'a>(crate::message::sysex_bytes::Sysex7BytesBorrowed<'a>);
    }
}

fn borrowed_message_impl(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    let builder_ident = common::message_borrowed_builder_ident(root_ident);
    quote! {
        impl<'a> #ident<'a> {
            fn builder(buffer: &mut [u8]) -> #builder_ident {
                #builder_ident::new(buffer)
            }
        }
    }
}

fn borrowed_builder_type(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_builder_ident(root_ident);
    quote! {
        pub struct #ident<'a>(crate::message::sysex_bytes::Sysex7BytesBorrowedBuilder<'a>);
    }
}

fn borrowed_builder_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let builder_ident = common::message_borrowed_builder_ident(root_ident);
    let message_ident = common::message_borrowed_ident(root_ident);
    let mut property_setters = TokenStream::new();
    let mut write_defaults = TokenStream::new();
    for property in properties.iter() {
        let property_ident = &property.ident;
        let property_type = &property.ty;
        let property_meta_type = &property.meta_type;
        write_defaults.extend(quote! {
            #property_meta_type::to_sysex(&#property_meta_type::default(), &mut sysex_builder);
        });
        if property.builder {
            property_setters.extend(quote! {
                pub fn #property_ident(mut self, v: #property_type) -> Self {
                    #property_meta_type::to_sysex(&v, &mut self.0);
                    self
                }
            });
        }
    }
    quote! {
        impl<'a> #builder_ident<'a> {
            pub fn build(self) -> crate::result::Result<#message_ident<'a>> {
                Ok(#message_ident(self.0.build()?))
            }
            pub fn new(buffer: &'a mut [u8]) -> Self {
                let mut sysex_builder = crate::message::sysex_bytes::Sysex7BytesBorrowedBuilder::new(buffer);
                #write_defaults
                Self(sysex_builder)
            }
            #property_setters
        }
    }
}

fn specialised_message_trait(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = common::specialised_message_trait_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| p.message) {
        methods.extend(specialised_message_trait_method(property));
    }
    quote! {
        pub trait #ident<'a, 'b: 'a>: crate::traits::Sysex<'a> + Sized {
            #methods
        }
    }
}

fn specialised_message_trait_method(property: &Property) -> TokenStream {
    let ident = &property.ident;
    let ty = &property.ty;
    let meta_type = &property.meta_type;
    quote! {
        fn #ident(&'b self) -> #ty {
            #meta_type::from_sysex(self).unwrap()
        }
    }
}

fn borrowed_message_byte_data_trait_impl(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    quote! {
        impl<'a> crate::traits::ByteData for #ident<'a> {
            fn byte_data(&self) -> &[u8] {
                self.0.byte_data()
            }
        }
    }
}

fn borrowed_message_sysex_trait_impl(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    quote! {
        impl<'a> crate::traits::Sysex<'a> for #ident<'a> {
            type PayloadIterator = <crate::message::sysex_bytes::Sysex7BytesBorrowed<'a> as crate::traits::Sysex<'a>>::PayloadIterator;
            fn payload<'b: 'a>(&'b self) -> Self::PayloadIterator {
                self.0.payload()
            }
        }
    }
}

fn borrowed_message_specialised_trait_impl(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    let trait_ident = common::specialised_message_trait_ident(root_ident);
    quote! {
        impl<'a, 'b: 'a> #trait_ident<'a, 'b> for #ident<'a> {}
    }
}

fn borrowed_message_ci_trait_impl(root_ident: &Ident) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    quote! {
        impl<'a> crate::ci::Ci for #ident<'a> {}
    }
}

fn borrowed_message_from_byte_data_trait_impl(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let ident = common::message_borrowed_ident(root_ident);
    let mut validation_steps = TokenStream::new();
    for property in properties.iter() {
        let meta_type = &property.meta_type;
        validation_steps.extend(quote! {
            #meta_type::from_sysex(&sysex)?;
        });
    }
    quote! {
        impl<'a> crate::traits::FromByteData<'a> for #ident<'a> {
            type Target = Self;
            fn validate_byte_data(buffer: &'a [u8]) -> crate::result::Result<()> {
                let sysex = crate::message::sysex_bytes::Sysex7BytesBorrowed::from_byte_data(buffer)?;
                #validation_steps
                Ok(())
            }
            fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
                Self(crate::message::sysex_bytes::Sysex7BytesBorrowed::from_byte_data_unchecked(buffer))
            }
        }
    }
}

pub fn generate_ci(_attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemStruct);
    let root_ident = &input.ident;
    let properties = properties(&input);

    let imports = imports();
    let message_type = message_type(root_ident);
    let borrowed_builder_type = borrowed_builder_type(root_ident);
    let specialised_message_trait = specialised_message_trait(root_ident, &properties);
    let borrowed_message_byte_data_trait_impl = borrowed_message_byte_data_trait_impl(root_ident);
    let borrowed_message_sysex_trait_impl = borrowed_message_sysex_trait_impl(root_ident);
    let borrowed_message_specialised_trait_impl =
        borrowed_message_specialised_trait_impl(root_ident);
    let borrowed_message_ci_trait_impl = borrowed_message_ci_trait_impl(root_ident);
    let borrowed_message_from_byte_data_trait_impl =
        borrowed_message_from_byte_data_trait_impl(root_ident, &properties);
    let borrowed_builder_impl = borrowed_builder_impl(root_ident, &properties);
    let borrowed_message_impl = borrowed_message_impl(root_ident);

    quote! {
        #imports
        #message_type
        #borrowed_builder_type
        #specialised_message_trait
        #borrowed_message_byte_data_trait_impl
        #borrowed_message_sysex_trait_impl
        #borrowed_message_specialised_trait_impl
        #borrowed_message_ci_trait_impl
        #borrowed_message_from_byte_data_trait_impl
        #borrowed_builder_impl
        #borrowed_message_impl
    }
    .into()
}
