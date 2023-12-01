use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_str, AngleBracketedGenericArguments, Field, Fields, GenericArgument,
    Ident, ItemEnum, ItemStruct, PathArguments, PathSegment, Type, TypePath,
};

struct Property {
    name: Ident,
    constant: bool,
    ty: Type,
    ump_representation: Type,
    bytes_representation: Type,
}

fn buffer_representation_type(generic_arg: &GenericArgument) -> Type {
    let GenericArgument::Type(ty) = generic_arg else {
        panic!()
    };
    ty.clone()
}

impl Property {
    fn from_field(field: &Field) -> Option<Self> {
        let Some(name) = &field.ident else {
            return None;
        };
        let Type::Path(TypePath { path, .. }) = &field.ty else {
            return None;
        };
        let Some(PathSegment {
            ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
        }) = path.segments.last()
        else {
            return None;
        };
        if ident != "Property" || args.len() != 3 {
            return None;
        }
        let GenericArgument::Type(ty) = &args[0] else {
            return None;
        };
        Some(Self {
            name: name.clone(),
            constant: is_constant_property(ty),
            ty: ty.clone(),
            ump_representation: buffer_representation_type(&args[1]),
            bytes_representation: buffer_representation_type(&args[2]),
        })
    }
}

fn is_constant_property(ty: &Type) -> bool {
    let Type::Path(path_type) = ty else {
        return false;
    };
    let Some(PathSegment { ident, .. }) = path_type.path.segments.last() else {
        return false;
    };
    ident == "NumericalConstant"
}

fn message_owned_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Owned", root_ident), Span::call_site())
}

fn message_borrowed_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Borrowed", root_ident), Span::call_site())
}

fn builder_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Builder", root_ident), Span::call_site())
}

fn aggregate_message_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Message", root_ident), Span::call_site())
}

fn imports() -> TokenStream {
    quote! {
        use crate::{
            buffer::*,
            traits::*,
            util::{schema::*, BitOps},
            *,
        };
    }
}

fn message_owned(root_ident: &Ident) -> TokenStream {
    let ident = message_owned_ident(root_ident);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub struct #ident([u32; 4]);
    }
}

fn message_owned_impl(root_ident: &Ident) -> TokenStream {
    let ident = message_owned_ident(root_ident);
    let builder_ident = builder_ident(root_ident);
    quote! {
        impl #ident {
             pub fn builder() -> #builder_ident<Self> {
                #builder_ident::new()
            }
        }
    }
}

fn impl_aggregate_message(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let builder_ident = builder_ident(root_ident);
    quote! {
        impl<'a> #ident<'a> {
             pub fn builder() -> #builder_ident<#ident<'a>> {
                #builder_ident::new()
            }
        }
    }
}

fn message_borrowed(root_ident: &Ident) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub struct #ident<'a>(&'a [u32]);
    }
}

fn aggregate_message(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    let borrowed_ident = message_borrowed_ident(root_ident);
    quote! {
        #[derive(derive_more::From, midi2_attr::Grouped, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
        pub enum #ident<'a> {
            Owned(#owned_ident),
            Borrowed(#borrowed_ident<'a>),
        }

    }
}

fn to_owned_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    quote! {
        impl<'a> ToOwned for #ident<'a> {
            type Owned = #owned_ident;
            fn to_owned(self) -> Self::Owned {
                let mut data = [0x0_u32; 4];
                data.copy_from_slice(self.0);
                #owned_ident(data)
            }
        }
    }
}

fn to_owned_impl_aggregate(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    quote! {
        impl<'a> ToOwned for #ident<'a> {
            type Owned = #owned_ident;
            fn to_owned(self) -> #owned_ident {
                match self {
                    Self::Owned(m) => m,
                    Self::Borrowed(m) => m.to_owned(),
                }
            }
        }
    }
}

fn builder(root_ident: &Ident) -> TokenStream {
    let ident = builder_ident(root_ident);
    let message_ident = message_owned_ident(root_ident);
    quote! {
        pub struct #ident<M: core::convert::From<#message_ident>>(Option<[u32; 4]>, core::marker::PhantomData<M>);
    }
}

fn specialised_message_trait_ident(root_ident: &Ident) -> TokenStream {
    quote! {#root_ident}
}

fn specialised_message_trait(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = specialised_message_trait_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(message_impl_method(property, false));
    }
    quote! {
        pub trait #ident: Data {
            #methods
        }
    }
}

fn specialised_message_trait_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl #trait_ident for #message_ident {}
    }
}

fn specialised_message_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl<'a> #trait_ident for #message_ident<'a> {}
    }
}

fn specialised_message_trait_impl_aggregate(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(aggregate_message_impl_method(property));
    }
    let message_ident = aggregate_message_ident(root_ident);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl<'a> #trait_ident for #message_ident<'a> {
            #methods
        }
    }
}

fn message_impl_method(property: &Property, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let ump_schema = &property.ump_representation;
    let bytes_schema = &property.bytes_representation;
    let visibility = if public {
        parse_str::<TokenStream>("pub").unwrap()
    } else {
        TokenStream::new()
    };
    quote! {
        #visibility fn #name(&self) -> #ty {
            <Ump as Property<#ty, #ump_schema, #bytes_schema>>::get(self.data())
        }
    }
}

fn aggregate_message_impl_method(property: &Property) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    quote! {
        fn #name(&self) -> #ty {
            match self {
                Self::Owned(m) => m.#name(),
                Self::Borrowed(m) => m.#name(),
            }
        }
    }
}

fn builder_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = builder_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(builder_impl_method(property, true));
    }
    let message_ident = message_owned_ident(root_ident);
    let write_const_data = builder_new_write_const_data(properties);
    quote! {
        impl<M: core::convert::From<#message_ident>> #ident<M> {
            #methods
            pub fn new() -> Self {
                let mut buffer = [0x0_u32; 4];
                #write_const_data
                Self(Some(buffer), Default::default())
            }
            pub fn build(self) -> Result<M> {
                if let Some(buffer) = self.0 {
                    Ok(#message_ident(buffer).into())
                } else {
                    Err(Error::BufferOverflow)
                }
            }
        }
    }
}

fn grouped_builder_impl(root_ident: &Ident) -> TokenStream {
    let ident = builder_ident(root_ident);
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl<M: core::convert::From<#message_ident>> GroupedBuilder for #ident<M> {
            fn group(mut self, v: u4) -> Self {
                if let Some(buffer) = &mut self.0 {
                    <Ump as Property<u4, UmpSchema<0x0F00_0000, 0x0, 0x0, 0x0>, ()>>::write(buffer, v);
                }
                self
            }
        }
    }
}

fn builder_impl_method(property: &Property, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let ump_schema = &property.ump_representation;
    let bytes_schema = &property.bytes_representation;
    let visibility = {
        let mut ret = TokenStream::new();
        if public {
            ret.extend(parse_str::<TokenStream>("pub").unwrap());
        }
        ret
    };
    quote! {
        #visibility fn #name(mut self, v: #ty) -> Self {
            if let Some(buffer) = &mut self.0 {
                <Ump as Property<#ty, #ump_schema, #bytes_schema>>::write(buffer, v);
            }
            self
        }
    }
}

fn data_trait_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl Data for #message_ident {
            fn data(&self) -> &[u32] {
                &self.0[..]
            }
        }
    }
}

fn data_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Data for #message_ident<'a> {
            fn data(&self) -> &[u32] {
                self.0
            }
        }
    }
}

fn from_data_trait_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    let validation_steps = validation_steps(properties);
    quote! {
        impl<'a> FromData<'a> for #message_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(data: &'a [u32]) -> Self {
                #message_ident(data)
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                #validation_steps
                if buffer.len() != 4 {
                    return Err(Error::InvalidData);
                }
                Ok(())
            }
        }
    }
}

fn from_data_trait_impl_aggreagate(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let borrowed_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> FromData<'a> for #ident<'a> {
            type Target = Self;
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                #borrowed_ident::validate_data(buffer)
            }
            fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
                Self::Borrowed(#borrowed_ident::from_data_unchecked(buffer))
            }
        }
    }
}

fn validation_steps(properties: &Vec<Property>) -> TokenStream {
    let mut ret = TokenStream::new();
    for property in properties {
        let ty = &property.ty;
        let ump_schema = &property.ump_representation;
        let bytes_schema = &property.bytes_representation;
        ret.extend(quote! {
            <Ump as Property<#ty, #ump_schema, #bytes_schema>>::validate(buffer)?;
        })
    }
    ret
}

fn builder_new_write_const_data(properties: &Vec<Property>) -> TokenStream {
    let mut ret = TokenStream::new();
    for property in properties.iter().filter(|property| property.constant) {
        let ump_schema = &property.ump_representation;
        let bytes_schema = &property.bytes_representation;
        let ty = &property.ty;
        ret.extend(quote! {
            <Ump as Property::<#ty, #ump_schema, #bytes_schema>>::write(&mut buffer, Default::default());
        })
    }
    ret
}

fn grouped_message_trait_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl Grouped for #message_ident {
            fn group(&self) -> u4 {
                self.0[0].nibble(1)
            }
        }
    }
}

fn grouped_message_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Grouped for #message_ident<'a> {
            fn group(&self) -> u4 {
                self.0[0].nibble(1)
            }
        }
    }
}

fn debug_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl core::fmt::Debug for #message_ident {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!(#message_ident)))?;
                let mut iter = self.0.iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#010X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(",")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    }
}

fn debug_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> core::fmt::Debug for #message_ident<'a> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!(#message_ident)))?;
                let mut iter = self.0.iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#010X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(",")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    }
}

#[proc_macro_attribute]
pub fn generate_message(_attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemStruct);

    let root_ident = input.ident;
    let properties = {
        let mut ret = Vec::<Property>::new();
        if let Fields::Named(fields) = &input.fields {
            for field in &fields.named {
                if let Some(property) = Property::from_field(field) {
                    ret.push(property);
                }
            }
        }
        ret
    };

    let imports = imports();
    let specialised_message = specialised_message_trait(&root_ident, &properties);

    let message_owned = message_owned(&root_ident);
    let message_owned_impl = message_owned_impl(&root_ident);
    let message_borrowed = message_borrowed(&root_ident);
    let specialised_message_trait_impl_owned = specialised_message_trait_impl_owned(&root_ident);
    let specialised_message_trait_impl_borrowed =
        specialised_message_trait_impl_borrowed(&root_ident);
    let builder = builder(&root_ident);
    let builder_impl = builder_impl(&root_ident, &properties);
    let grouped_builder_impl = grouped_builder_impl(&root_ident);
    let data_trait_impl_owned = data_trait_impl_owned(&root_ident);
    let data_trait_impl_borrowed = data_trait_impl_borrowed(&root_ident);
    let from_data_trait_impl = from_data_trait_impl(&root_ident, &properties);
    let grouped_message_trait_impl_owned = grouped_message_trait_impl_owned(&root_ident);
    let grouped_message_trait_impl_borrowed = grouped_message_trait_impl_borrowed(&root_ident);
    let debug_impl_owned = debug_impl_owned(&root_ident);
    let debug_impl_borrowed = debug_impl_borrowed(&root_ident);
    let impl_aggregate_message = impl_aggregate_message(&root_ident);
    let aggregate_message = aggregate_message(&root_ident);
    let to_owned_impl_borrowed = to_owned_impl_borrowed(&root_ident);
    let to_owned_impl_aggregate = to_owned_impl_aggregate(&root_ident);
    let specialised_message_trait_impl_aggregate =
        specialised_message_trait_impl_aggregate(&root_ident, &properties);
    let from_data_trait_impl_aggreagate = from_data_trait_impl_aggreagate(&root_ident);

    quote! {
        #imports
        #specialised_message

        #message_owned
        #message_owned_impl
        #message_borrowed
        #specialised_message_trait_impl_owned
        #specialised_message_trait_impl_borrowed
        #builder
        #builder_impl
        #grouped_builder_impl
        #data_trait_impl_owned
        #data_trait_impl_borrowed
        #from_data_trait_impl
        #grouped_message_trait_impl_owned
        #grouped_message_trait_impl_borrowed
        #debug_impl_owned
        #debug_impl_borrowed
        #impl_aggregate_message
        #aggregate_message
        #to_owned_impl_aggregate
        #to_owned_impl_borrowed
        #specialised_message_trait_impl_aggregate
        #from_data_trait_impl_aggreagate
    }
    .into()
}

fn enum_lifetime(item: &ItemEnum) -> TokenStream {
    match item.generics.params.first() {
        Some(syn::GenericParam::Lifetime(lifetime)) => {
            quote! { #lifetime }
        }
        _ => TokenStream::new(),
    }
}

#[proc_macro_derive(Data)]
pub fn derive_data(item: TokenStream1) -> TokenStream1 {
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

#[proc_macro_derive(Grouped)]
pub fn derive_grouped(item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemEnum);
    let ident = &input.ident;
    let mut match_arms = TokenStream::new();
    for variant in &input.variants {
        let variant_ident = &variant.ident;
        match_arms.extend(quote! {
            #variant_ident(m) => m.group(),
        });
    }
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> Grouped for #ident<#lifetime_param> {
            fn group(&self) -> u4 {
                use #ident::*;
                match self {
                    #match_arms
                }
            }
        }
    }
    .into()
}
