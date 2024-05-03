use crate::common::Representation;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

struct Property {
    ident: syn::Ident,
    meta_type: TokenStream,
    ty: syn::Type,
    constant: bool,
    #[allow(dead_code)]
    readonly: bool,
}

impl Property {
    fn implement_via_trait(&self) -> bool {
        self.is_group()
            || self.is_channel()
            || self.is_sysex_payload()
            || self.is_jitter_reduction()
    }
    fn is_placeholder(&self) -> bool {
        self.is_sysex_payload()
    }
    fn is_group(&self) -> bool {
        self.ident == "group"
    }
    fn is_channel(&self) -> bool {
        self.ident == "channel"
    }
    fn is_sysex_payload(&self) -> bool {
        self.ident == "sysex_payload"
    }
    fn is_jitter_reduction(&self) -> bool {
        self.ident == "jitter_reduction"
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
                .any(|&segment| segment.ident.to_string() == "property")
        })
        .map(|list| list.tokens.clone())
        .expect("fields must be annotated with the property attribute")
}

fn properties(input: &syn::ItemStruct) -> Vec<Property> {
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
            constant: is_unit_tuple(&field.ty),
            readonly: has_attr(field, "readonly"),
        })
        .collect()
}

fn is_unit_tuple(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Tuple(tup) => tup.elems.len() == 0,
        _ => false,
    }
}

#[derive(Default)]
struct GenerateMessageArgs {
    fixed_size: bool,
    min_size_ump: Option<usize>,
    min_size_bytes: Option<usize>,
    type_doc: Option<syn::LitStr>,
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

            let ident = ident.to_string();
            if ident == "FixedSize" {
                args.fixed_size = true;
            }
            if ident == "MinSizeUmp" {
                args.min_size_ump = Some(parse_fixed_size(input));
            }
            if ident == "MinSizeBytes" {
                args.min_size_bytes = Some(parse_fixed_size(input));
            }
            if ident == "Doc" {
                args.type_doc = Some(parse_string(input));
            }

            if let Err(_) = input.parse::<syn::Token![,]>() {
                assert!(input.is_empty());
                break;
            }
        }

        Ok(args)
    }
}

fn parse_fixed_size(input: syn::parse::ParseStream) -> usize {
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

fn parse_string(input: syn::parse::ParseStream) -> syn::LitStr {
    let syn::ExprParen { expr, .. } = input.parse().expect("Expected bracketed expression");

    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(str_lit),
        ..
    }) = *expr
    else {
        panic!("Expected a string literal");
    };
    str_lit
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

fn message(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    let mut doc = TokenStream::new();
    if let Some(doc_file) = &args.type_doc {
        doc.extend(quote! {
            #[doc = include_str!(#doc_file)]
        });
    }
    quote! {
        #[derive(PartialEq, Eq, midi2_proc::Debug)]
        #doc
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
        methods.extend(property_getter(property, true));
        methods.extend(property_setter(property, true));
    }

    quote! {
        impl<B: #constraint> #root_ident<B> {
            #methods
        }
    }
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
    quote! {
        #pub_token fn #ident(&self) -> #ty {
            <#meta_type as crate::util::property::ReadProperty<B>>::read(self.buffer_access())
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
    quote! {
        #pub_token fn #ident(&mut self, value: #ty) where B: crate::buffer::BufferMut {
            <#meta_type as crate::util::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
        }
    }
}

fn message_new_arr_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let arr_type = match args.representation() {
        Representation::Bytes => arr_type_bytes(),
        Representation::Ump => arr_type_ump(),
        Representation::UmpOrBytes => arr_type_ump(),
    };
    let set_defaults = initialise_property_statements(properties);
    quote! {
        impl #root_ident<#arr_type> {
            pub fn new_arr() -> Self {
                let mut buffer: #arr_type = core::default::Default::default();
                #set_defaults
                #root_ident(buffer)
            }
        }
    }
}

fn secondary_new_arr_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let arr_type = arr_type_bytes();
    let set_defaults = initialise_property_statements(properties);
    quote! {
        impl #root_ident<#arr_type> {
            pub fn new_arr_bytes() -> Self {
                let mut buffer: #arr_type = core::default::Default::default();
                #set_defaults
                #root_ident(buffer)
            }
        }
    }
}

fn arr_type_ump() -> TokenStream {
    quote! { [u32; 5] }
}

fn arr_type_bytes() -> TokenStream {
    quote! { [u8; 3] }
}

fn size_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> crate::traits::Size<B> for #root_ident<B> {
            fn size(&self) -> usize {
                match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U32 => {
                        // account for jitter reduction header
                        use crate::buffer::UmpPrivate;
                        <Self as crate::traits::MinSize<B>>::min_size()
                            + self.buffer_access().specialise_u32().jitter_reduction().len()
                    }
                    crate::buffer::UNIT_ID_U8 => {
                        // simple case
                        // no jitter reduction logic here
                        <Self as crate::traits::MinSize<B>>::min_size()
                    }
                    _ => unreachable!(),
                }
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
            fn min_size() -> usize {
                #body
            }
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
                match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U32 => {
                        use crate::buffer::UmpPrivate;
                        use crate::util::BitOps;

                        // account for jitter reduction header
                        let buffer = self.buffer_access().specialise_u32();
                        let jr_slice = buffer.jitter_reduction();
                        let jr_offset = match jr_slice.len() {
                            0 => 0,
                            _ => {
                                match u8::from(jr_slice[0].nibble(2)) {
                                    0 => 1,
                                    _ => 0,
                                }
                            }
                        };
                        &self.buffer_access().buffer()[jr_offset..self.size()]
                    }
                    crate::buffer::UNIT_ID_U8 => {
                        // simple case
                        // no jitter reduction logic here
                        &self.buffer_access().buffer()[..self.size()]
                    }
                    _ => unreachable!(),
                }
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
    for property in properties.iter().filter(|p| !p.is_placeholder()) {
        let meta_type = &property.meta_type;
        validation_steps.extend(quote! {
            <#meta_type as crate::util::property::ReadProperty<&[#unit_type]>>::validate(&buffer)?;
        });
    }
    quote! {
        impl<'a, #generic_unit> core::convert::TryFrom<&'a [#unit_type]> for #root_ident<&'a [#unit_type]> {
            type Error = crate::error::Error;
            fn try_from(buffer: &'a [#unit_type]) -> core::result::Result<Self, Self::Error> {
                if buffer.len() < <Self as crate::traits::MinSize<&[#unit_type]>>::min_size() {
                    return Err(crate::error::Error::InvalidData("Slice is too short"));
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
                match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U32 => {
                        // account for jitter reduction header
                        use crate::buffer::UmpPrivate;
                        let message_size = other.data().len();
                        let jr_offset: usize = match other.data()
                            .specialise_u32()
                            .jitter_reduction()
                            .len() {
                            0 => {
                                // other message had no jitter reduction header
                                // -> we add it in on our side
                                1
                            },
                            _ => 0,
                        };
                        buffer.resize(message_size + jr_offset);
                        buffer.buffer_mut()[jr_offset..(message_size + jr_offset)].copy_from_slice(other.data());
                    }
                    crate::buffer::UNIT_ID_U8 => {
                        // simple case
                        // no jitter reduction logic here
                        let message_size = other.data().len();
                        buffer.resize(message_size);
                        buffer.buffer_mut()[..message_size].copy_from_slice(other.data());
                    }
                    _ => unreachable!(),
                }
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
                match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U32 => {
                        // account for jitter reduction header
                        use crate::buffer::UmpPrivate;
                        let message_size = other.data().len();
                        let jr_offset: usize = match other.data()
                            .specialise_u32()
                            .jitter_reduction()
                            .len() {
                            0 => {
                                // other message had no jitter reduction header
                                // -> we add it in on our side
                                1
                            },
                            _ => 0,
                        };
                        buffer.try_resize(message_size + jr_offset)?;
                        buffer.buffer_mut()[jr_offset..(message_size + jr_offset)].copy_from_slice(other.data());
                    }
                    crate::buffer::UNIT_ID_U8 => {
                        // simple case
                        // no jitter reduction logic here
                        let message_size = other.data().len();
                        buffer.try_resize(message_size)?;
                        buffer.buffer_mut()[..message_size].copy_from_slice(other.data());
                    }
                    _ => unreachable!(),
                }
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
    let initialise_properties = initialise_property_statements(properties);
    quote! {
        impl<B: #constraint
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize
        > #root_ident<B>
        {
            pub fn new() -> #root_ident<B>
            {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let jr_offset = match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    // account for jitter reduction header
                    crate::buffer::UNIT_ID_U32 => 1,
                    crate::buffer::UNIT_ID_U8 => 0,
                    _ => unreachable!(),
                };
                buffer.resize(<Self as crate::traits::MinSize<B>>::min_size() + jr_offset);
                #initialise_properties
                #root_ident::<B>(buffer)
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
    let initialise_properties = initialise_property_statements(properties);
    quote! {
        impl<B: #constraint
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize
        > #root_ident<B>
        {
            pub fn try_new() -> core::result::Result<#root_ident<B>, crate::error::BufferOverflow>
            {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let jr_offset = match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    // account for jitter reduction header
                    crate::buffer::UNIT_ID_U32 => 1,
                    crate::buffer::UNIT_ID_U8 => 0,
                    _ => unreachable!(),
                };
                buffer.try_resize(<Self as crate::traits::MinSize<B>>::min_size() + jr_offset)?;
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

fn initialise_property_statements(properties: &Vec<Property>) -> TokenStream {
    let mut initialise_properties = TokenStream::new();
    for property in properties.iter().filter(|p| !p.is_placeholder()) {
        let meta_type = &property.meta_type;
        initialise_properties.extend(quote! {
            <#meta_type as crate::util::property::WriteProperty<B>>::write(
                &mut buffer,
                <#meta_type as crate::util::property::WriteProperty<B>>::default(),
            );
        });
    }
    initialise_properties
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

fn jitter_reduction_impl(root_ident: &syn::Ident, property: &Property) -> TokenStream {
    let setter = property_setter(property, false);
    let getter = property_getter(property, false);
    quote! {
        impl<B: crate::buffer::Ump> crate::traits::JitterReduced<B> for #root_ident<B> {
            #getter
            #setter
        }
    }
}

fn channeled_impl(root_ident: &syn::Ident, property: &Property) -> TokenStream {
    let setter = property_setter(property, false);
    let getter = property_getter(property, false);
    quote! {
        impl<B: crate::buffer::Buffer> crate::traits::Channeled<B> for #root_ident<B> {
            #getter
            #setter
        }
    }
}

fn from_bytes_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties);
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
                buffer.resize(
                    <#root_ident<B> as crate::traits::MinSize<B>>::min_size()
                    + crate::buffer::OFFSET_FOR_JITTER_REDUCTION
                );
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn try_from_bytes_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties);
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
                buffer.try_resize(
                    <#root_ident<B> as crate::traits::MinSize<B>>::min_size()
                    + crate::buffer::OFFSET_FOR_JITTER_REDUCTION
                )?;
                #convert_properties
                Ok(Self(buffer))
            }
        }
    }
}

fn convert_properties(properties: &Vec<Property>) -> TokenStream {
    let mut convert_properties = TokenStream::new();
    for property in properties {
        let meta_type = &property.meta_type;
        convert_properties.extend(quote! {
            <#meta_type as crate::util::property::WriteProperty<B>>::write(
                &mut buffer,
                <#meta_type as crate::util::property::ReadProperty<A>>::read(&other.0)
            );
        });
    }
    convert_properties
}

fn from_ump_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties);
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
                buffer.resize(<#root_ident<B> as crate::traits::MinSize<B>>::min_size());
                #convert_properties
                Self(buffer)
            }
        }
    }
}

fn try_from_ump_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let convert_properties = convert_properties(properties);
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
                buffer.try_resize(<#root_ident<B> as crate::traits::MinSize<B>>::min_size())?;
                #convert_properties
                Ok(Self(buffer))
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
    let message = message(root_ident, &args);
    let message_impl = message_impl(root_ident, &args, &properties);
    let data_impl = data_impl(root_ident, &args);
    let min_size_impl = min_size_impl(root_ident, &args);
    let buffer_access_impl = buffer_access_impl(root_ident, &args);
    let try_from_slice_impl = try_from_slice_impl(root_ident, &args, &properties);
    let rebuffer_from_impl = rebuffer_from_impl(root_ident, &args);
    let try_rebuffer_from_impl = try_rebuffer_from_impl(root_ident, &args);
    let new_impl = new_impl(root_ident, &args, &properties);
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
        #try_new_impl
        #clone_impl
    });

    if args.fixed_size {
        tokens.extend(message_new_arr_impl(root_ident, &args, &properties));
        if let Representation::UmpOrBytes = args.representation() {
            tokens.extend(secondary_new_arr_impl(root_ident, &properties));
        }
    }
    if args.fixed_size {
        tokens.extend(size_impl(root_ident, &args))
    }
    if let Some(property) = properties.iter().find(|p| p.is_jitter_reduction()) {
        tokens.extend(jitter_reduction_impl(root_ident, property));
    }
    if let Some(property) = properties.iter().find(|p| p.is_group()) {
        tokens.extend(grouped_impl(root_ident, property));
    }
    if let Some(property) = properties.iter().find(|p| p.is_channel()) {
        tokens.extend(channeled_impl(root_ident, property));
    }
    if let Representation::UmpOrBytes = args.representation() {
        if !properties.iter().any(|p| p.is_sysex_payload()) {
            tokens.extend(from_bytes_impl(root_ident, &properties));
            tokens.extend(from_ump_impl(root_ident, &properties));
            tokens.extend(try_from_bytes_impl(root_ident, &properties));
            tokens.extend(try_from_ump_impl(root_ident, &properties));
        }
    }

    tokens.into()
}
