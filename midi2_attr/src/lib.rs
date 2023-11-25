use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    token::{Colon, Comma, Gt, Lt, PathSep, Plus},
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, Ident, ItemStruct, Path,
    PathArguments, PathSegment, TraitBound, TraitBoundModifier, Type, TypeParam, TypeParamBound,
    TypePath,
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

fn message_owned_ident_public(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Owned", root_ident), Span::call_site())
}

fn message_borrowed_ident_public(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Borrowed", root_ident), Span::call_site())
}

fn builder_ident_public(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Builder", root_ident), Span::call_site())
}

fn privatise(ident: &Ident) -> Ident {
    Ident::new(&format!("{}Private", ident), Span::call_site())
}

fn message_owned_ident(root_ident: &Ident) -> Ident {
    privatise(&message_owned_ident_public(root_ident))
}

fn message_borrowed_ident(root_ident: &Ident) -> Ident {
    privatise(&message_borrowed_ident_public(root_ident))
}

fn builder_ident(root_ident: &Ident) -> Ident {
    privatise(&builder_ident_public(root_ident))
}

fn imports() -> TokenStream {
    quote! {
        use crate::{
            buffer::*,
            traits::*,
            util::{schema::*, BitOps},
            *,
        };
        use generic_array::typenum::Unsigned;
    }
}

fn buffer_generic_with_constraints(properties: &Vec<Property>) -> TypeParam {
    let mut bounds = Punctuated::<TypeParamBound, Plus>::new();
    for property in properties {
        let segments = {
            let mut args = Punctuated::<GenericArgument, Comma>::new();
            args.push(GenericArgument::Type(property.ty.clone()));
            args.push(GenericArgument::Type(property.ump_representation.clone()));
            args.push(GenericArgument::Type(property.bytes_representation.clone()));
            let arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Lt {
                    spans: [Span::call_site()],
                },
                gt_token: Gt {
                    spans: [Span::call_site()],
                },
                args,
            });
            let segment = PathSegment {
                ident: parse_str("Property").unwrap(),
                arguments,
            };
            let mut s = Punctuated::<PathSegment, PathSep>::new();
            s.push(segment);
            s
        };
        let path = Path {
            leading_colon: None,
            segments,
        };
        let bound = TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path,
        };
        bounds.push(TypeParamBound::Trait(bound));
    }
    TypeParam {
        attrs: Default::default(),
        ident: parse_str("B").unwrap(),
        colon_token: Some(Colon {
            spans: [Span::call_site()],
        }),
        bounds,
        eq_token: None,
        default: None,
    }
}

fn message_owned(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = message_owned_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub(crate) struct #ident<#buffer_type>(generic_array::GenericArray<B::Data, B::Size>);
    }
}

fn message_owned_public(root_ident: &Ident) -> TokenStream {
    let ident = message_owned_ident_public(root_ident);
    let private_ident = message_owned_ident(root_ident);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub struct #ident(#private_ident<Ump>);
    }
}

fn message_owned_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = message_owned_ident(root_ident);
    let builder_ident = builder_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        impl<#buffer_type> #ident<B> {
             pub fn builder() -> #builder_ident<B> {
                #builder_ident::new()
            }
        }
    }
}

fn message_owned_impl_public(root_ident: &Ident) -> TokenStream {
    let ident = message_owned_ident_public(root_ident);
    let builder_ident = builder_ident_public(root_ident);
    quote! {
        impl #ident {
             pub fn builder() -> #builder_ident<#ident> {
                #builder_ident::new()
            }
        }
    }
}

fn message_borrowed(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub(crate) struct #ident<'a, #buffer_type>(&'a [B::Data]);
    }
}

fn message_borrowed_public(root_ident: &Ident) -> TokenStream {
    let ident = message_borrowed_ident_public(root_ident);
    let private_ident = message_borrowed_ident(root_ident);
    quote! {
        #[derive(Clone, PartialEq, Eq)]
        pub struct #ident<'a>(#private_ident<'a, Ump>);
    }
}

fn to_owned_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = message_borrowed_ident(root_ident);
    let owned_ident = message_owned_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        impl<'a, #buffer_type> ToOwned for #ident<'a, B> {
            type Owned = #owned_ident<B>;
            fn to_owned(self) -> Self::Owned {
                let mut data: generic_array::GenericArray<B::Data, B::Size> = Default::default();
                data.copy_from_slice(self.0);
                #owned_ident(data)
            }
        }
    }
}

fn to_owned_impl_public(root_ident: &Ident) -> TokenStream {
    let ident = message_borrowed_ident_public(root_ident);
    let owned_ident = message_owned_ident_public(root_ident);
    quote! {
        impl<'a> ToOwned for #ident<'a> {
            type Owned = #owned_ident;
            fn to_owned(self) -> Self::Owned {
                #owned_ident(self.0.to_owned())
            }
        }
    }
}

fn builder(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = builder_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        pub(crate) struct #ident<#buffer_type>(Option<generic_array::GenericArray<B::Data, B::Size>>);
    }
}

fn builder_public(root_ident: &Ident) -> TokenStream {
    let ident = builder_ident_public(root_ident);
    let message_ident = message_owned_ident_public(root_ident);
    let private_ident = builder_ident(root_ident);
    quote! {
        pub struct #ident<M: core::convert::From<#message_ident>>(#private_ident<Ump>, core::marker::PhantomData<M>);
    }
}

fn specialised_message_trait_ident(root_ident: &Ident) -> TokenStream {
    quote! {#root_ident}
}

fn specialized_message_trait(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = specialised_message_trait_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(specialised_message_trait_declare_method(property));
    }
    quote! {
        pub trait #ident {
            #methods
        }
    }
}

fn specialized_message_trait_impl_owned(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(message_impl_method(property, false));
    }
    let message_ident = message_owned_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl<#buffer_type> #trait_ident for #message_ident<B> {
            #methods
        }
    }
}

fn specialized_message_trait_impl_owned_public(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(message_impl_method_public(property, false));
    }
    let message_ident = message_owned_ident_public(root_ident);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl #trait_ident for #message_ident {
            #methods
        }
    }
}

fn specialized_message_trait_impl_borrowed(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(message_impl_method(property, false));
    }
    let message_ident = message_borrowed_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl<'a, #buffer_type> #trait_ident for #message_ident<'a, B> {
            #methods
        }
    }
}

fn specialized_message_trait_impl_borrowed_public(
    root_ident: &Ident,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(message_impl_method_public(property, false));
    }
    let message_ident = message_borrowed_ident_public(root_ident);
    let trait_ident = specialised_message_trait_ident(root_ident);
    quote! {
        impl<'a> #trait_ident for #message_ident<'a> {
            #methods
        }
    }
}

fn specialised_message_trait_declare_method(property: &Property) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    quote! {
        fn #name(&self) -> #ty;
    }
}

fn message_impl_method(property: &Property, public: bool) -> TokenStream {
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
        #visibility fn #name(&self) -> #ty {
            <B as Property<#ty, #ump_schema, #bytes_schema>>::get(self.data())
        }
    }
}

fn message_impl_method_public(property: &Property, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let visibility = {
        let mut ret = TokenStream::new();
        if public {
            ret.extend(parse_str::<TokenStream>("pub").unwrap());
        }
        ret
    };
    quote! {
        #visibility fn #name(&self) -> #ty {
            self.0.#name()
        }
    }
}

fn builder_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = builder_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(builder_impl_method(property, true));
    }
    let buffer_type = buffer_generic_with_constraints(properties);
    let message_ident = message_owned_ident(root_ident);
    let write_const_data = builder_new_write_const_data(properties);
    quote! {
        impl<#buffer_type> #ident<B> {
            #methods
            pub fn new() -> Self {
                let mut buffer: generic_array::GenericArray<B::Data, B::Size> = Default::default();
                #write_const_data
                Self(Some(buffer))
            }
            pub fn build(self) -> Result<#message_ident<B>> {
                if let Some(buffer) = self.0 {
                    Ok(#message_ident(buffer))
                } else {
                    Err(Error::BufferOverflow)
                }
            }
        }
    }
}

fn builder_impl_public(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let ident = builder_ident_public(root_ident);
    let private_ident = builder_ident(root_ident);
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(builder_impl_method_public(property, true));
    }
    let message_ident = message_owned_ident_public(root_ident);
    quote! {
        impl<M: core::convert::From<#message_ident>> #ident<M> {
            #methods
            pub fn new() -> Self {
                Self(#private_ident::<Ump>::new(), Default::default())
            }
            pub fn build(self) -> Result<M> {
                match self.0.build() {
                    Ok(message) => Ok(#message_ident(message).into()),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

fn grouped_builder_impl(root_ident: &Ident) -> TokenStream {
    let ident = builder_ident(root_ident);
    quote! {
        impl GroupedBuilder for #ident<Ump> {
            fn group(mut self, v: u4) -> Self {
                if let Some(buffer) = &mut self.0 {
                    <Ump as Property<u4, UmpSchema<0x0F00_0000, 0x0, 0x0, 0x0>, ()>>::write(buffer, v);
                }
                self
            }
        }
    }
}

fn grouped_builder_impl_public(root_ident: &Ident) -> TokenStream {
    let ident = builder_ident_public(root_ident);
    let message_ident = message_owned_ident_public(root_ident);
    quote! {
        impl<M: core::convert::From<#message_ident>> GroupedBuilder for #ident<M> {
            fn group(mut self, v: u4) -> Self {
                self.0 = self.0.group(v);
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
                <B as Property<#ty, #ump_schema, #bytes_schema>>::write(buffer, v);
            }
            self
        }
    }
}

fn builder_impl_method_public(property: &Property, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let visibility = {
        let mut ret = TokenStream::new();
        if public {
            ret.extend(parse_str::<TokenStream>("pub").unwrap());
        }
        ret
    };
    quote! {
        #visibility fn #name(mut self, v: #ty) -> Self {
            self.0 = self.0.#name(v);
            self
        }
    }
}

fn data_trait_impl_owned(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        impl<#buffer_type> DataPrivate<B> for #message_ident<B> {
            fn data(&self) -> &[B::Data] {
                &self.0[..]
            }
        }
    }
}

fn data_trait_impl_borrowed(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        impl<'a, #buffer_type> DataPrivate<B> for #message_ident<'a, B> {
            fn data(&self) -> &[B::Data] {
                self.0
            }
        }
    }
}

fn data_trait_impl_owned_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident_public(root_ident);
    quote! {
        impl Data for #message_ident {
            fn data(&self) -> &[u32] {
                &self.0.0[..]
            }
        }
    }
}

fn data_trait_impl_borrowed_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident_public(root_ident);
    quote! {
        impl<'a> Data for #message_ident<'a> {
            fn data(&self) -> &[u32] {
                self.0.0
            }
        }
    }
}

fn from_data_trait_impl(root_ident: &Ident, properties: &Vec<Property>) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    let validation_steps = validation_steps(properties);
    let buffer_type = buffer_generic_with_constraints(properties);
    quote! {
        impl<'a, #buffer_type> FromDataPrivate<'a, B> for #message_ident<'a, B> {
            type Target = Self;
            fn from_data_unchecked(data: &'a [<B as Buffer>::Data]) -> Self {
                #message_ident(data)
            }
            fn validate_data(buffer: &'a [<B as Buffer>::Data]) -> Result<()> {
                #validation_steps
                if buffer.len() != <B as Buffer>::Size::to_usize() {
                    return Err(Error::InvalidData);
                }
                Ok(())
            }
        }
    }
}

fn from_data_trait_impl_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident_public(root_ident);
    let private_message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> FromData<'a> for #message_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(data: &'a [u32]) -> Self {
                #message_ident(<#private_message_ident<Ump> as FromDataPrivate<Ump>>::from_data_unchecked(data))
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                <#private_message_ident<Ump> as FromDataPrivate<Ump>>::validate_data(buffer)
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
            <B as Property<#ty, #ump_schema, #bytes_schema>>::validate(buffer)?;
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
            <B as Property::<#ty, #ump_schema, #bytes_schema>>::write(&mut buffer, Default::default());
        })
    }
    ret
}

fn grouped_message_trait_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl Grouped for #message_ident<Ump> {
            fn group(&self) -> u4 {
                self.0[0].nibble(1)
            }
        }
    }
}

fn grouped_message_trait_impl_owned_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident_public(root_ident);
    quote! {
        impl Grouped for #message_ident {
            fn group(&self) -> u4 {
                self.0.group()
            }
        }
    }
}

fn grouped_message_trait_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> Grouped for #message_ident<'a, Ump> {
            fn group(&self) -> u4 {
                self.0[0].nibble(1)
            }
        }
    }
}

fn grouped_message_trait_impl_borrowed_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident_public(root_ident);
    quote! {
        impl<'a> Grouped for #message_ident<'a> {
            fn group(&self) -> u4 {
                self.0.group()
            }
        }
    }
}

fn debug_impl_owned(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident(root_ident);
    quote! {
        impl core::fmt::Debug for #message_ident<Ump> {
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
        impl core::fmt::Debug for #message_ident<Bytes> {
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

fn debug_impl_owned_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_owned_ident_public(root_ident);
    let private_message_ident = message_owned_ident(root_ident);
    quote! {
        impl core::fmt::Debug for #message_ident {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                <#private_message_ident<Ump> as core::fmt::Debug>::fmt(&self.0, fmt)
            }
        }
    }
}

fn debug_impl_borrowed(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> core::fmt::Debug for #message_ident<'a, Ump> {
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
        impl<'a> core::fmt::Debug for #message_ident<'a, Bytes> {
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

fn debug_impl_borrowed_public(root_ident: &Ident) -> TokenStream {
    let message_ident = message_borrowed_ident_public(root_ident);
    let private_message_ident = message_borrowed_ident(root_ident);
    quote! {
        impl<'a> core::fmt::Debug for #message_ident<'a> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                <#private_message_ident<'a, Ump> as core::fmt::Debug>::fmt(&self.0, fmt)
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
    let specialized_message = specialized_message_trait(&root_ident, &properties);

    let message_owned = message_owned(&root_ident, &properties);
    let message_owned_impl = message_owned_impl(&root_ident, &properties);
    let message_borrowed = message_borrowed(&root_ident, &properties);
    let to_owned_impl = to_owned_impl(&root_ident, &properties);
    let specialized_message_trait_impl_owned =
        specialized_message_trait_impl_owned(&root_ident, &properties);
    let specialized_message_trait_impl_borrowed =
        specialized_message_trait_impl_borrowed(&root_ident, &properties);
    let builder = builder(&root_ident, &properties);
    let builder_impl = builder_impl(&root_ident, &properties);
    let grouped_builder_impl = grouped_builder_impl(&root_ident);
    let data_trait_impl_owned = data_trait_impl_owned(&root_ident, &properties);
    let data_trait_impl_borrowed = data_trait_impl_borrowed(&root_ident, &properties);
    let from_data_trait_impl = from_data_trait_impl(&root_ident, &properties);
    let grouped_message_trait_impl_owned = grouped_message_trait_impl_owned(&root_ident);
    let grouped_message_trait_impl_borrowed = grouped_message_trait_impl_borrowed(&root_ident);
    let debug_impl_owned = debug_impl_owned(&root_ident);
    let debug_impl_borrowed = debug_impl_borrowed(&root_ident);

    let message_owned_public = message_owned_public(&root_ident);
    let message_owned_impl_public = message_owned_impl_public(&root_ident);
    let message_borrowed_public = message_borrowed_public(&root_ident);
    let to_owned_impl_public = to_owned_impl_public(&root_ident);
    let specialized_message_trait_impl_owned_public =
        specialized_message_trait_impl_owned_public(&root_ident, &properties);
    let specialized_message_trait_impl_borrowed_public =
        specialized_message_trait_impl_borrowed_public(&root_ident, &properties);
    let builder_public = builder_public(&root_ident);
    let builder_impl_public = builder_impl_public(&root_ident, &properties);
    let grouped_builder_impl_public = grouped_builder_impl_public(&root_ident);
    let data_trait_impl_owned_public = data_trait_impl_owned_public(&root_ident);
    let data_trait_impl_borrowed_public = data_trait_impl_borrowed_public(&root_ident);
    let from_data_trait_impl_public = from_data_trait_impl_public(&root_ident);
    let grouped_message_trait_impl_owned_public =
        grouped_message_trait_impl_owned_public(&root_ident);
    let grouped_message_trait_impl_borrowed_public =
        grouped_message_trait_impl_borrowed_public(&root_ident);
    let debug_impl_owned_public = debug_impl_owned_public(&root_ident);
    let debug_impl_borrowed_public = debug_impl_borrowed_public(&root_ident);

    quote! {
        #imports
        #specialized_message

        #message_owned
        #message_owned_impl
        #message_borrowed
        #to_owned_impl
        #specialized_message_trait_impl_owned
        #specialized_message_trait_impl_borrowed
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

        #message_owned_public
        #message_owned_impl_public
        #message_borrowed_public
        #to_owned_impl_public
        #specialized_message_trait_impl_owned_public
        #specialized_message_trait_impl_borrowed_public
        #builder_public
        #builder_impl_public
        #grouped_builder_impl_public
        #data_trait_impl_owned_public
        #data_trait_impl_borrowed_public
        #from_data_trait_impl_public
        #grouped_message_trait_impl_owned_public
        #grouped_message_trait_impl_borrowed_public
        #debug_impl_owned_public
        #debug_impl_borrowed_public
    }
    .into()
}
