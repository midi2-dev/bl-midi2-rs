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

fn is_unit_tuple(ty: &Type) -> bool {
    match ty {
        Type::Tuple(tup) => tup.elems.len() == 0,
        _ => false,
    }
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

    fn has_byte_scheme(&self) -> bool {
        !is_unit_tuple(&self.bytes_representation)
    }

    fn has_ump_scheme(&self) -> bool {
        !is_unit_tuple(&self.ump_representation)
    }

    fn is_channel(&self) -> bool {
        self.name == "channel"
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

fn is_not_zero_int_literal(arg: &GenericArgument) -> bool {
    let GenericArgument::Const(syn::Expr::Lit(syn::ExprLit { lit, .. })) = arg else {
        return false;
    };
    let syn::Lit::Int(int_lit) = lit else {
        return false;
    };
    int_lit.base10_parse::<u32>().expect("valid int literal") != 0
}

fn deduce_message_size<Repr: Fn(&Property) -> &Type>(
    properties: &Vec<Property>,
    repr: Repr,
) -> usize {
    properties.iter().fold(0, |accum, prop| {
        let Type::Path(TypePath { path, .. }) = repr(prop) else {
            return accum;
        };
        let Some(PathSegment {
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
            ..
        }) = path.segments.last()
        else {
            return accum;
        };
        let sz = args
            .iter()
            .rev()
            .position(is_not_zero_int_literal)
            .map(|v| args.len() - v)
            .unwrap_or(0);
        core::cmp::max(accum, sz)
    })
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

fn into_owned_impl_borrowed(root_ident: &Ident, sz: usize) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    quote! {
        impl<'a> IntoOwned for #ident<'a> {
            type Owned = #owned_ident;
            fn into_owned(self) -> Self::Owned {
                let mut data = [0x0_u32; 4];
                data[..#sz].copy_from_slice(self.0);
                #owned_ident(data)
            }
        }
    }
}

fn into_owned_impl_aggregate(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    quote! {
        impl<'a> IntoOwned for #ident<'a> {
            type Owned = #owned_ident;
            fn into_owned(self) -> #owned_ident {
                match self {
                    Self::Owned(m) => m,
                    Self::Borrowed(m) => m.into_owned(),
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

fn specialised_message_trait(
    root_ident: &Ident,
    properties: &Vec<Property>,
    channeled_message: bool,
) -> TokenStream {
    let ident = specialised_message_trait_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties
        .iter()
        .filter(|p| !p.constant && !(p.is_channel() && channeled_message))
    {
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
    channeled_message: bool,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties
        .iter()
        .filter(|p| !p.constant && !(p.is_channel() && channeled_message))
    {
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

fn builder_impl(root_ident: &Ident, properties: &Vec<Property>, grouped: bool) -> TokenStream {
    let ident = builder_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(builder_impl_method(property, true));
    }
    if grouped {
        methods.extend(quote!{
            pub fn group(mut self, v: u4) -> Self {
                if let Some(buffer) = &mut self.0 {
                    <Ump as Property<u4, UmpSchema<0x0F00_0000, 0x0, 0x0, 0x0>, ()>>::write(buffer, v);
                }
                self
            }
        });
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

fn data_trait_impl_owned(root_ident: &Ident, sz: usize) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl Data for #message_ident {
            fn data(&self) -> &[u32] {
                &self.0[..#sz]
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

fn from_data_trait_impl(root_ident: &Ident, properties: &Vec<Property>, sz: usize) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    let validation_steps = validation_steps(properties);
    quote! {
        impl<'a> FromData<'a> for #message_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(data: &'a [u32]) -> Self {
                #message_ident(&data[..#sz])
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                #validation_steps
                if buffer.len() < #sz {
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

fn level2_message_impl_owned(root_ident: &Ident) -> TokenStream {
    let ident = message_owned_ident(root_ident);
    quote! {
        impl Level2Message for #ident {}
    }
}

fn level2_message_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Level2Message for #ident<'a> {}
    }
}

fn from_byte_data_impl_owned(
    root_ident: &Ident,
    properties: &Vec<Property>,
    sz: usize,
) -> TokenStream {
    let owned_ident = message_owned_ident(root_ident);

    let mut validation_steps = TokenStream::new();
    for prop in properties {
        validation_steps.extend(from_byte_data_validation_step(prop));
    }

    let mut convert_property_steps = TokenStream::new();
    for prop in properties
        .iter()
        .filter(|p| p.has_ump_scheme() && p.has_byte_scheme())
    {
        convert_property_steps.extend(from_byte_data_convert_property_step(prop));
    }

    let mut write_const_data_steps = TokenStream::new();
    for prop in properties
        .iter()
        .filter(|p| p.constant && p.has_ump_scheme())
    {
        write_const_data_steps.extend(from_byte_data_write_const_data_step(prop));
    }

    quote! {
        impl<'a> FromByteData<'a> for #owned_ident {
            type Target = Self;
            fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
                if buffer.len() < #sz {
                    return Err(Error::BufferOverflow);
                }
                #validation_steps
                Ok(())
            }
            fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
                let mut outbuffer = [0x0_u32; 4];
                #write_const_data_steps
                #convert_property_steps
                Self(outbuffer)
            }
        }
    }
}

fn from_byte_data_impl_aggregate(root_ident: &Ident) -> TokenStream {
    let ident = aggregate_message_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    quote! {
        impl<'a, 'b> FromByteData<'a> for #ident<'b> {
            type Target = Self;
            fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
                #owned_ident::validate_byte_data(buffer)
            }
            fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
                Self::Owned(#owned_ident::from_byte_data_unchecked(buffer))
            }
        }
    }
}

fn write_byte_data_impl(ident: &Ident, lifetime: bool, properties: &Vec<Property>) -> TokenStream {
    let mut write_const_data_steps = TokenStream::new();
    for prop in properties
        .iter()
        .filter(|p| p.constant && p.has_byte_scheme())
    {
        write_const_data_steps.extend(write_byte_data_write_const_data_step(prop));
    }

    let mut convert_property_steps = TokenStream::new();
    for prop in properties
        .iter()
        .filter(|p| p.has_ump_scheme() && p.has_byte_scheme())
    {
        convert_property_steps.extend(write_byte_data_convert_property_step(prop));
    }

    let lifetime_generic = if lifetime {
        quote!('b)
    } else {
        TokenStream::new()
    };

    quote! {
        impl<#lifetime_generic> WriteByteData for #ident<#lifetime_generic> {
            fn write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> &'a mut [u8] {
                #write_const_data_steps
                #convert_property_steps
                buffer
            }
        }
    }
}

fn from_byte_data_validation_step(prop: &Property) -> TokenStream {
    let ty = &prop.ty;
    let ump_schema = &prop.ump_representation;
    let byte_schema = &prop.bytes_representation;
    quote! {
        <Bytes as Property<#ty, #ump_schema, #byte_schema>>::validate(buffer)?;
    }
}

fn write_byte_data_convert_property_step(prop: &Property) -> TokenStream {
    let ty = &prop.ty;
    let ump_schema = &prop.ump_representation;
    let byte_schema = &prop.bytes_representation;
    quote! {
        {
            let v = <Ump as Property<#ty, #ump_schema, #byte_schema>>::get(self.data());
            <Bytes as Property<#ty, #ump_schema, #byte_schema>>::write(buffer, v);
        }
    }
}

fn from_byte_data_convert_property_step(prop: &Property) -> TokenStream {
    let ty = &prop.ty;
    let ump_schema = &prop.ump_representation;
    let byte_schema = &prop.bytes_representation;
    quote! {
        {
            let v = <Bytes as Property<#ty, #ump_schema, #byte_schema>>::get(buffer);
            <Ump as Property<#ty, #ump_schema, #byte_schema>>::write(&mut outbuffer, v);
        }
    }
}

fn write_byte_data_write_const_data_step(prop: &Property) -> TokenStream {
    let ty = &prop.ty;
    let ump_schema = &prop.ump_representation;
    let byte_schema = &prop.bytes_representation;
    quote! {
        <Bytes as Property<#ty, #ump_schema, #byte_schema>>::write(buffer, Default::default());
    }
}

fn from_byte_data_write_const_data_step(prop: &Property) -> TokenStream {
    let ty = &prop.ty;
    let ump_schema = &prop.ump_representation;
    let byte_schema = &prop.bytes_representation;
    quote! {
        <Ump as Property<#ty, #ump_schema, #byte_schema>>::write(&mut outbuffer, Default::default());
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
        impl Grouped for #message_ident {}
    }
}

fn grouped_message_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Grouped for #message_ident<'a> {}
    }
}

fn channeled_message_trait_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl Channeled for #message_ident {}
    }
}

fn channeled_message_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Channeled for #message_ident<'a> {}
    }
}

fn channeled_message_trait_impl_aggregate(root_ident: &Ident) -> TokenStream {
    let message_ident = aggregate_message_ident(root_ident);
    quote! {
        impl<'a> Channeled for #message_ident<'a> {}
    }
}

fn debug_impl_owned(root_ident: &Ident, sz: usize) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl core::fmt::Debug for #message_ident {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!(#message_ident)))?;
                let mut iter = self.0[..#sz].iter().peekable();
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

fn should_implement_from_byte_data(properties: &Vec<Property>) -> bool {
    properties.iter().any(|p| p.has_byte_scheme())
}

#[derive(Debug)]
struct GenerateMessageArgs {
    grouped: bool,
    channeled: bool,
}

impl syn::parse::Parse for GenerateMessageArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();
        loop {
            if let Ok(ident) = input.parse::<Ident>() {
                args.push(ident.to_string());
                match input.parse::<syn::Token![,]>() {
                    Err(_) => {
                        assert!(input.is_empty());
                        break;
                    }
                    _ => {}
                }
            } else {
                assert!(input.is_empty());
                break;
            }
        }
        Ok(GenerateMessageArgs {
            grouped: args.iter().find(|s| *s == "Grouped").is_some(),
            channeled: args.iter().find(|s| *s == "Channeled").is_some(),
        })
    }
}

#[proc_macro_attribute]
pub fn generate_message(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attrs as GenerateMessageArgs);

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

    let sz_ump = deduce_message_size(&properties, |p| &p.ump_representation);
    let sz_bytes = deduce_message_size(&properties, |p| &p.bytes_representation);

    let imports = imports();
    let specialised_message = specialised_message_trait(&root_ident, &properties, args.channeled);

    let message_owned = message_owned(&root_ident);
    let message_owned_impl = message_owned_impl(&root_ident);
    let message_borrowed = message_borrowed(&root_ident);
    let specialised_message_trait_impl_owned = specialised_message_trait_impl_owned(&root_ident);
    let specialised_message_trait_impl_borrowed =
        specialised_message_trait_impl_borrowed(&root_ident);
    let builder = builder(&root_ident);
    let builder_impl = builder_impl(&root_ident, &properties, args.grouped);
    let data_trait_impl_owned = data_trait_impl_owned(&root_ident, sz_ump);
    let data_trait_impl_borrowed = data_trait_impl_borrowed(&root_ident);
    let from_data_trait_impl = from_data_trait_impl(&root_ident, &properties, sz_ump);
    let debug_impl_owned = debug_impl_owned(&root_ident, sz_ump);
    let debug_impl_borrowed = debug_impl_borrowed(&root_ident);
    let impl_aggregate_message = impl_aggregate_message(&root_ident);
    let aggregate_message = aggregate_message(&root_ident);
    let into_owned_impl_borrowed = into_owned_impl_borrowed(&root_ident, sz_ump);
    let into_owned_impl_aggregate = into_owned_impl_aggregate(&root_ident);
    let specialised_message_trait_impl_aggregate =
        specialised_message_trait_impl_aggregate(&root_ident, &properties, args.channeled);
    let from_data_trait_impl_aggreagate = from_data_trait_impl_aggreagate(&root_ident);
    let level2_message_impl_borrowed = level2_message_impl_borrowed(&root_ident);
    let level2_message_impl_owned = level2_message_impl_owned(&root_ident);

    let mut ret = quote! {
        #imports
        #specialised_message
        #message_owned
        #message_owned_impl
        #message_borrowed
        #specialised_message_trait_impl_owned
        #specialised_message_trait_impl_borrowed
        #builder
        #builder_impl
        #data_trait_impl_owned
        #data_trait_impl_borrowed
        #from_data_trait_impl
        #debug_impl_owned
        #debug_impl_borrowed
        #impl_aggregate_message
        #aggregate_message
        #into_owned_impl_aggregate
        #into_owned_impl_borrowed
        #specialised_message_trait_impl_aggregate
        #from_data_trait_impl_aggreagate
        #level2_message_impl_borrowed
        #level2_message_impl_owned
    };

    if args.grouped {
        let grouped_message_trait_impl_owned = grouped_message_trait_impl_owned(&root_ident);
        let grouped_message_trait_impl_borrowed = grouped_message_trait_impl_borrowed(&root_ident);

        ret.extend(quote! {
            #grouped_message_trait_impl_owned
            #grouped_message_trait_impl_borrowed
        });
    }

    if args.channeled {
        let channeled_message_trait_impl_owned = channeled_message_trait_impl_owned(&root_ident);
        let channeled_message_trait_impl_borrowed =
            channeled_message_trait_impl_borrowed(&root_ident);
        let channeled_message_trait_impl_aggregate =
            channeled_message_trait_impl_aggregate(&root_ident);

        ret.extend(quote! {
            #channeled_message_trait_impl_owned
            #channeled_message_trait_impl_borrowed
            #channeled_message_trait_impl_aggregate
        });
    }

    if should_implement_from_byte_data(&properties) {
        let from_byte_data_impl_owned =
            from_byte_data_impl_owned(&root_ident, &properties, sz_bytes);
        let from_byte_data_impl_aggregate = from_byte_data_impl_aggregate(&root_ident);
        let write_byte_data_borrowed =
            write_byte_data_impl(&message_borrowed_ident(&root_ident), true, &properties);
        let write_byte_data_owned =
            write_byte_data_impl(&message_owned_ident(&root_ident), false, &properties);
        let write_byte_data_aggregate =
            write_byte_data_impl(&aggregate_message_ident(&root_ident), true, &properties);
        ret.extend(quote! {
            #from_byte_data_impl_owned
            #from_byte_data_impl_aggregate
            #write_byte_data_borrowed
            #write_byte_data_owned
            #write_byte_data_aggregate
        });
    }

    ret.into()
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
    let lifetime_param = enum_lifetime(&input);
    quote! {
        impl<#lifetime_param> Grouped for #ident<#lifetime_param> {}
    }
    .into()
}

#[proc_macro_derive(WriteByteData)]
pub fn derive_write_byte_data(item: TokenStream1) -> TokenStream1 {
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

#[proc_macro_derive(Channeled)]
pub fn derive_channeled(item: TokenStream1) -> TokenStream1 {
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
