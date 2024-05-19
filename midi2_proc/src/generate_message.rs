use crate::common::{self, Representation};
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
struct GenerateMessageArgs {
    fixed_size: bool,
    min_size_ump: Option<usize>,
    min_size_bytes: Option<usize>,
    via: Option<syn::Type>,
}

impl GenerateMessageArgs {
    fn representation(&self) -> Representation {
        match (&self.min_size_ump, &self.min_size_bytes) {
            (&Some(_), &Some(_)) => Representation::UmpOrBytes,
            (None, &Some(_)) => Representation::Bytes,
            (&Some(_), None) => Representation::Ump,
            (None, None) => panic!("Couldn't deduce message representation"),
        }
    }
}

impl syn::parse::Parse for GenerateMessageArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args: GenerateMessageArgs = Default::default();
        loop {
            let Ok(ident) = input.parse::<syn::Ident>() else {
                assert!(input.is_empty());
                break;
            };

            if ident == "Via" {
                args.via = Some(common::parse_via_args(input));
            }
            if ident == "FixedSize" {
                args.fixed_size = true;
            }
            if ident == "MinSizeUmp" {
                args.min_size_ump = Some(parse_fixed_size(input));
            }
            if ident == "MinSizeBytes" {
                args.min_size_bytes = Some(parse_fixed_size(input));
            }

            if let Err(_) = input.parse::<syn::Token![,]>() {
                assert!(input.is_empty());
                break;
            }
        }

        Ok(args)
    }
}

pub fn parse_fixed_size(input: syn::parse::ParseStream) -> usize {
    let syn::ExprParen { expr, .. } = input
        .parse()
        .expect("Bracketed expression should follow size arg");

    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(int_lit),
        ..
    }) = *expr
    else {
        panic!("Size expressions should contain int literal");
    };

    int_lit
        .base10_parse::<usize>()
        .expect("Valid base 10 literal size")
}

pub struct Property {
    pub ident: syn::Ident,
    pub meta_type: syn::Type,
    pub ty: syn::Type,
    pub constant: bool,
    pub readonly: bool,
    pub writeonly: bool,
    pub resize: bool,
    pub std: bool,
}

impl Property {
    pub fn implement_via_trait(&self) -> bool {
        self.is_group() || self.is_channel() || self.is_sysex_payload()
    }
    pub fn is_group(&self) -> bool {
        self.ident == "group"
    }
    pub fn is_channel(&self) -> bool {
        self.ident == "channel"
    }
    pub fn is_sysex_payload(&self) -> bool {
        self.ident == "sysex_payload"
    }
}

pub fn properties(input: &syn::ItemStruct) -> Vec<Property> {
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
            meta_type: common::meta_type(field),
            constant: common::is_unit_tuple(&field.ty),
            readonly: common::has_attr(field, "readonly"),
            writeonly: common::has_attr(field, "writeonly"),
            resize: common::has_attr(field, "resize"),
            std: common::has_attr(field, "std"),
        })
        .collect()
}

pub fn initialise_property_statements(
    properties: &Vec<Property>,
    buffer_type: TokenStream,
) -> TokenStream {
    let mut initialise_properties = TokenStream::new();
    for property in properties.iter().filter(|p| !p.readonly) {
        let meta_type = &property.meta_type;
        let std_only_attribute = common::std_only_attribute(property.std);

        initialise_properties.extend(quote! {
            #std_only_attribute
            <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::write(
                buffer_ref_mut,
                <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::default(),
            );
        });
    }
    initialise_properties
}

fn property_getter(property: &Property, public: bool) -> TokenStream {
    let meta_type = &property.meta_type;
    let ident = &property.ident;
    let ty = &property.ty;
    let pub_token = if public {
        quote! { pub }
    } else {
        TokenStream::new()
    };
    let std_only_attribute = common::std_only_attribute(property.std);

    quote! {
        #std_only_attribute
        #pub_token fn #ident(&self) -> #ty {
            <#meta_type as crate::detail::property::ReadProperty<B>>::read(self.buffer_access())
        }
    }
}

fn property_setter(property: &Property, public: bool) -> TokenStream {
    let meta_type = &property.meta_type;
    let ident = syn::Ident::new(
        format!("set_{}", &property.ident.to_string()).as_str(),
        proc_macro2::Span::call_site(),
    );
    let ty = &property.ty;
    let pub_token = if public {
        quote! { pub }
    } else {
        TokenStream::new()
    };
    let std_only_attribute = common::std_only_attribute(property.std);

    if property.resize {
        let fallible_ident = syn::Ident::new(
            format!("try_{}", ident.to_string()).as_str(),
            proc_macro2::Span::call_site(),
        );
        quote! {
            #std_only_attribute
            #pub_token fn #ident(&mut self, value: #ty) where B: crate::buffer::BufferMut + crate::buffer::BufferResize {
                <#meta_type as crate::detail::property::ResizeProperty<B>>::resize(self.buffer_access_mut(), &value);
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
            }

            #std_only_attribute
            #pub_token fn #fallible_ident(&mut self, value: #ty) -> core::result::Result<(), crate::error::BufferOverflow>
            where B: crate::buffer::BufferMut + crate::buffer::BufferTryResize {
                <#meta_type as crate::detail::property::ResizeProperty<B>>::try_resize(self.buffer_access_mut(), &value)?;
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
                Ok(())
            }
        }
    } else {
        quote! {
            #std_only_attribute
            #pub_token fn #ident(&mut self, value: #ty) where B: crate::buffer::BufferMut {
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
            }
        }
    }
}

fn imports() -> TokenStream {
    quote! {
        use crate::buffer::UnitPrivate as UnitPrivateGenMessage;
        use crate::buffer::SpecialiseU32 as SpecialiseU32GenMessage;
        use crate::buffer::SpecialiseU8 as SpecialiseU8GenMessage;
        use crate::traits::Size as SizeGenMessage;
        use crate::traits::Data as DataGenMessage;
        use crate::traits::BufferAccess as BufferAccessGenMessage;
    }
}

fn generic_buffer_constraint(args: &GenerateMessageArgs) -> TokenStream {
    match args.representation() {
        Representation::UmpOrBytes => quote! { crate::buffer::Buffer },
        Representation::Bytes => quote! { crate::buffer::Bytes },
        Representation::Ump => quote! { crate::buffer::Ump },
    }
}

fn message(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    attributes: &Vec<syn::Attribute>,
) -> TokenStream {
    let constraint = generic_buffer_constraint(args);

    let mut doc_attributes = TokenStream::new();
    for attribute in attributes.iter() {
        if let syn::Meta::NameValue(syn::MetaNameValue { path, .. }) = &attribute.meta {
            if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                if ident == "doc" {
                    doc_attributes.extend(quote! { #attribute });
                }
            }
        }
    }

    quote! {
        #[derive(PartialEq, Eq, midi2_proc::Debug)]
        #doc_attributes
        pub struct #root_ident<B: #constraint>(B);
    }
}

fn message_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let constraint = generic_buffer_constraint(args);

    let mut methods = TokenStream::new();
    for property in properties
        .iter()
        .filter(|p| !p.constant && !p.implement_via_trait())
    {
        if !property.writeonly {
            methods.extend(property_getter(property, true));
        }
        if !property.readonly {
            methods.extend(property_setter(property, true));
        }
    }

    quote! {
        impl<B: #constraint> #root_ident<B> {
            #methods
        }
    }
}

fn size_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::Size<B> for #root_ident<B> {
            fn size(&self) -> usize {
                <Self as crate::traits::MinSize<B>>::MIN_SIZE
            }
        }
    }
}

fn min_size_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let body = match (&args.min_size_ump, &args.min_size_bytes) {
        (&Some(ump_size), &Some(bytes_size)) => quote! {
            match <B::Unit as UnitPrivateGenMessage>::UNIT_ID {
                crate::buffer::UNIT_ID_U32 => #ump_size,
                crate::buffer::UNIT_ID_U8 => #bytes_size,
                _ => unreachable!(),
            }
        },
        (None, &Some(bytes_size)) => quote! { #bytes_size },
        (&Some(ump_size), None) => quote! { #ump_size },
        (None, None) => panic!("Couldn't deduce message size"),
    };
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::MinSize<B> for #root_ident<B> {
            const MIN_SIZE: usize = #body;
        }
    }
}

fn buffer_access_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::BufferAccess<B> for #root_ident<B> {
            fn buffer_access(&self) -> &B {
                &self.0
            }
            fn buffer_access_mut(&mut self) -> &mut B
            where
                B: crate::buffer::BufferMut
            {
                &mut self.0
            }
        }
    }
}

fn data_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::Data<B> for #root_ident<B> {
            fn data(&self) -> &[B::Unit] {
                &self.buffer_access().buffer()[..self.size()]
            }
        }
    }
}

fn packets_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Ump> crate::Packets for #root_ident<B> {
            fn packets(&self) -> crate::PacketsIterator {
                crate::PacketsIterator(self
                    .data()
                    .chunks_exact(<Self as crate::traits::MinSize<B>>::MIN_SIZE)
                )
            }
        }
    }
}

fn try_from_slice_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut validation_steps = TokenStream::new();
    let generic_unit = match args.representation() {
        Representation::UmpOrBytes => quote! { U: crate::buffer::Unit },
        _ => TokenStream::new(),
    };
    let unit_type = match args.representation() {
        Representation::UmpOrBytes => quote! { U },
        Representation::Ump => quote! { u32 },
        Representation::Bytes => quote! { u8 },
    };
    for property in properties.iter().filter(|p| !p.writeonly) {
        let meta_type = &property.meta_type;
        let std_only_attribute = common::std_only_attribute(property.std);

        validation_steps.extend(quote! {
            #std_only_attribute
            <#meta_type as crate::detail::property::ReadProperty<&[#unit_type]>>::validate(&buffer)?;
        });
    }
    quote! {
        impl<'a, #generic_unit> core::convert::TryFrom<&'a [#unit_type]> for #root_ident<&'a [#unit_type]> {
            type Error = crate::error::InvalidData;
            fn try_from(buffer: &'a [#unit_type]) -> core::result::Result<Self, Self::Error> {
                if buffer.len() < <Self as crate::traits::MinSize<&[#unit_type]>>::MIN_SIZE {
                    return Err(crate::error::InvalidData("Slice is too short"));
                }
                #validation_steps
                Ok(#root_ident(buffer))
            }
        }
    }
}

fn rebuffer_from_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let generics = crate::common::rebuffer_generics(args.representation());
    quote! {
        impl #generics crate::traits::RebufferFrom<#root_ident<A>> for #root_ident<B>
        {
            fn rebuffer_from(other: #root_ident<A>) -> Self {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let message_size = other.data().len();
                buffer.resize(message_size);
                buffer.buffer_mut()[..message_size].copy_from_slice(other.data());
                #root_ident(buffer)
            }
        }
    }
}

fn rebuffer_from_array_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    let buffer_type = quote! { [<A as crate::buffer::Buffer>::Unit; SIZE] };
    quote! {
        impl<const SIZE: usize, A: #constraint> crate::traits::RebufferFrom<#root_ident<A>> for #root_ident<#buffer_type>
        {
            fn rebuffer_from(other: #root_ident<A>) -> Self {
                let _valid = <Self as crate::traits::ArraySizeValid<SIZE, #buffer_type>>::VALID;
                let mut buffer = <#buffer_type as crate::buffer::BufferDefault>::default();
                let message_size = other.data().len();
                buffer[..message_size].copy_from_slice(other.data());
                #root_ident(buffer)
            }
        }
    }
}

fn try_rebuffer_from_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let generics = crate::common::try_rebuffer_generics(args.representation());
    quote! {
        impl #generics crate::traits::TryRebufferFrom<#root_ident<A>> for #root_ident<B>
        {
            fn try_rebuffer_from(other: #root_ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let message_size = other.data().len();
                buffer.try_resize(message_size)?;
                buffer.buffer_mut()[..message_size].copy_from_slice(other.data());
                Ok(#root_ident(buffer))
            }
        }
    }
}

fn new_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    let initialise_properties = initialise_property_statements(properties, quote! {B});
    quote! {
        impl<B: #constraint
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize
        > #root_ident<B>
        {
            /// Create a new message backed by a resizable buffer.
            pub fn new() -> #root_ident<B>
            {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let buffer_ref_mut = &mut buffer;
                buffer_ref_mut.resize(<Self as crate::traits::MinSize<B>>::MIN_SIZE);
                #initialise_properties
                #root_ident::<B>(buffer)
            }
        }
    }
}

fn new_array_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let generics = match args.representation() {
        Representation::UmpOrBytes => quote! { , U: crate::buffer::Unit },
        _ => TokenStream::new(),
    };
    let unit_type = match args.representation() {
        Representation::Ump => quote! { u32 },
        Representation::Bytes => quote! { u8 },
        Representation::UmpOrBytes => quote! { U },
    };
    let buffer_type = quote! { [#unit_type; SIZE] };
    let initialise_properties = initialise_property_statements(properties, quote! { #buffer_type });
    quote! {
        impl<const SIZE: usize #generics> #root_ident<#buffer_type>
        {
            /// Create a new message backed by a simple array type buffer.
            ///
            /// Note: this constructor will fail to compile for `SIZE` values
            /// which are smaller than the minimum representable message size.
            pub fn new() -> #root_ident<#buffer_type>
            {
                let _valid = <Self as crate::traits::ArraySizeValid<SIZE, #buffer_type>>::VALID;
                let mut buffer = [<#unit_type as crate::buffer::Unit>::zero(); SIZE];
                let buffer_ref_mut = &mut buffer;
                #initialise_properties
                #root_ident(buffer)
            }
        }
    }
}

fn try_new_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    let initialise_properties = initialise_property_statements(properties, quote! {B});
    quote! {
        impl<B: #constraint
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize
        > #root_ident<B>
        {
            /// Create a new message backed by a buffer with fallible resize.
            pub fn try_new() -> core::result::Result<#root_ident<B>, crate::error::BufferOverflow>
            {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                buffer.try_resize(<Self as crate::traits::MinSize<B>>::MIN_SIZE)?;
                let buffer_ref_mut = &mut buffer;
                #initialise_properties
                Ok(#root_ident::<B>(buffer))
            }
        }
    }
}

fn clone_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint + core::clone::Clone> core::clone::Clone for #root_ident<B> {
            fn clone(&self) -> Self {
                Self(self.buffer_access().clone())
            }
        }
    }
}

fn grouped_impl(root_ident: &syn::Ident, property: &Property) -> TokenStream {
    let setter = property_setter(property, false);
    let getter = property_getter(property, false);
    quote! {
        impl<B: crate::buffer::Ump> crate::traits::Grouped<B> for #root_ident<B> {
            #getter
            #setter
        }
    }
}

fn channeled_impl(
    root_ident: &syn::Ident,
    property: &Property,
    args: &GenerateMessageArgs,
) -> TokenStream {
    let setter = property_setter(property, false);
    let getter = property_getter(property, false);
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::Channeled<B> for #root_ident<B> {
            #getter
            #setter
        }
    }
}

fn from_bytes_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties, &quote! { B });
    quote! {
        impl<
                A: crate::buffer::Bytes,
                B: crate::buffer::Ump
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            > crate::traits::FromBytes<#root_ident<A>> for #root_ident<B>
        {
            fn from_bytes(other: #root_ident<A>) -> Self {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                buffer.resize(<#root_ident<B> as crate::traits::MinSize<B>>::MIN_SIZE);
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn from_bytes_array_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let array_type = quote! { [u32; SIZE] };
    let convert_properties = convert_properties(properties, &array_type);
    quote! {
        impl<const SIZE: usize, A: crate::buffer::Bytes> crate::traits::FromBytes<#root_ident<A>> for #root_ident<#array_type>
        {
            fn from_bytes(other: #root_ident<A>) -> Self {
                let _valid = <Self as crate::traits::ArraySizeValid<SIZE, #array_type>>::VALID;
                let mut buffer = <#array_type as crate::buffer::BufferDefault>::default();
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn try_from_bytes_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties, &quote! { B });
    quote! {
        impl<
                A: crate::buffer::Bytes,
                B: crate::buffer::Ump
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            > crate::traits::TryFromBytes<#root_ident<A>> for #root_ident<B>
        {
            fn try_from_bytes(other: #root_ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                buffer.try_resize(<#root_ident<B> as crate::traits::MinSize<B>>::MIN_SIZE)?;
                #convert_properties
                Ok(Self(buffer))
            }
        }
    }
}

fn convert_properties(properties: &Vec<Property>, target_buffer_type: &TokenStream) -> TokenStream {
    let mut convert_properties = TokenStream::new();
    for property in properties.iter().filter(|p| !p.readonly && !p.writeonly) {
        let std_only_attribute = common::std_only_attribute(property.std);
        let meta_type = &property.meta_type;

        convert_properties.extend(quote! {
            #std_only_attribute
            <#meta_type as crate::detail::property::WriteProperty<#target_buffer_type>>::write(
                &mut buffer,
                <#meta_type as crate::detail::property::ReadProperty<A>>::read(&other.0)
            );
        });
    }
    convert_properties
}

fn from_ump_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties, &quote! { B });
    quote! {
        impl<
                A: crate::buffer::Ump,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            > crate::traits::FromUmp<#root_ident<A>> for #root_ident<B>
        {
            fn from_ump(other: #root_ident<A>) -> Self {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                buffer.resize(<#root_ident<B> as crate::traits::MinSize<B>>::MIN_SIZE);
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn from_ump_array_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let array_type = quote! { [u8; SIZE] };
    let convert_properties = convert_properties(properties, &array_type);
    quote! {
        impl<const SIZE: usize, A: crate::buffer::Ump> crate::traits::FromUmp<#root_ident<A>> for #root_ident<#array_type>
        {
            fn from_ump(other: #root_ident<A>) -> Self {
                let _valid = <Self as crate::traits::ArraySizeValid<SIZE, #array_type>>::VALID;
                let mut buffer = <#array_type as crate::buffer::BufferDefault>::default();
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn try_from_ump_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties, &quote! { B });
    quote! {
        impl<
                A: crate::buffer::Ump,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            > crate::traits::TryFromUmp<#root_ident<A>> for #root_ident<B>
        {
            fn try_from_ump(other: #root_ident<A>) -> core::result::Result<Self, crate::error::BufferOverflow> {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                buffer.try_resize(<#root_ident<B> as crate::traits::MinSize<B>>::MIN_SIZE)?;
                #convert_properties
                Ok(Self(buffer))
            }
        }
    }
}

fn ump_message_via(root_ident: &syn::Ident, via_type: &syn::Type) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Ump> core::convert::From<#root_ident<B>> for crate::message::UmpMessage<B> {
            fn from(value: #root_ident<B>) -> Self {
                <#via_type<B> as core::convert::From<#root_ident<B>>>::from(value).into()
            }
        }
    }
}

fn bytes_message_via(root_ident: &syn::Ident, via_type: &syn::Type) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Bytes> core::convert::From<#root_ident<B>> for crate::message::BytesMessage<B> {
            fn from(value: #root_ident<B>) -> Self {
                <#via_type<B> as core::convert::From<#root_ident<B>>>::from(value).into()
            }
        }
    }
}

pub fn generate_message(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attrs as GenerateMessageArgs);
    let properties = properties(&input);
    let root_ident = &input.ident;

    let imports = imports();
    let message = message(root_ident, &args, &input.attrs);
    let message_impl = message_impl(root_ident, &args, &properties);
    let data_impl = data_impl(root_ident, &args);
    let min_size_impl = min_size_impl(root_ident, &args);
    let buffer_access_impl = buffer_access_impl(root_ident, &args);
    let try_from_slice_impl = try_from_slice_impl(root_ident, &args, &properties);
    let rebuffer_from_impl = rebuffer_from_impl(root_ident, &args);
    let try_rebuffer_from_impl = try_rebuffer_from_impl(root_ident, &args);
    let new_impl = new_impl(root_ident, &args, &properties);
    let new_array_impl = new_array_impl(root_ident, &args, &properties);
    let try_new_impl = try_new_impl(root_ident, &args, &properties);
    let clone_impl = clone_impl(root_ident, &args);

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        #imports
        #message
        #message_impl
        #data_impl
        #min_size_impl
        #buffer_access_impl
        #try_from_slice_impl
        #rebuffer_from_impl
        #try_rebuffer_from_impl
        #new_impl
        #new_array_impl
        #try_new_impl
        #clone_impl
    });

    if args.fixed_size {
        tokens.extend(size_impl(root_ident, &args));
        tokens.extend(rebuffer_from_array_impl(root_ident, &args));
    }
    if let Some(property) = properties.iter().find(|p| p.is_group()) {
        tokens.extend(grouped_impl(root_ident, property));
    }
    if let Some(property) = properties.iter().find(|p| p.is_channel()) {
        tokens.extend(channeled_impl(root_ident, property, &args));
    }
    if let Representation::UmpOrBytes = args.representation() {
        // we skip generating conversion for sysex7
        // these traits are implemented manually
        if !properties.iter().any(|p| p.is_sysex_payload()) {
            tokens.extend(from_bytes_impl(root_ident, &properties));
            tokens.extend(from_ump_impl(root_ident, &properties));
            tokens.extend(try_from_bytes_impl(root_ident, &properties));
            tokens.extend(try_from_ump_impl(root_ident, &properties));

            if args.fixed_size {
                tokens.extend(from_ump_array_impl(root_ident, &properties));
                tokens.extend(from_bytes_array_impl(root_ident, &properties));
            }
        }
    }
    if matches!(
        args.representation(),
        Representation::Ump | Representation::UmpOrBytes
    ) {
        tokens.extend(packets_impl(root_ident));
    }
    if let Some(via_type) = args.via.as_ref() {
        match args.representation() {
            Representation::Ump => tokens.extend(ump_message_via(root_ident, &via_type)),
            Representation::Bytes => tokens.extend(bytes_message_via(root_ident, &via_type)),
            Representation::UmpOrBytes => {
                tokens.extend(ump_message_via(root_ident, &via_type));
                tokens.extend(bytes_message_via(root_ident, &via_type));
            }
        }
    }

    tokens.into()
}
