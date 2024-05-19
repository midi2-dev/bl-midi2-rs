use crate::common;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
struct GenerateCiArgs {
    fixed_size: bool,
    min_size: Vec<usize>,
}

impl syn::parse::Parse for GenerateCiArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args: GenerateCiArgs = Default::default();
        loop {
            let Ok(ident) = input.parse::<syn::Ident>() else {
                assert!(input.is_empty());
                break;
            };

            if ident == "FixedSize" {
                args.fixed_size = true;
            }
            if ident == "MinSize" {
                args.min_size = parse_fixed_size(input);
            }

            if let Err(_) = input.parse::<syn::Token![,]>() {
                assert!(input.is_empty());
                break;
            }
        }

        Ok(args)
    }
}

pub fn parse_fixed_size(input: syn::parse::ParseStream) -> Vec<usize> {
    let syn::ExprTuple { elems, .. } = input
        .parse()
        .expect("Bracketed expression should follow size arg");

    let mut ret = Vec::new();

    for elem in &elems {
        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int_lit),
            ..
        }) = elem.clone()
        else {
            panic!("Size expressions should contain int literal");
        };

        ret.push(
            int_lit
                .base10_parse::<usize>()
                .expect("Valid base 10 literal size"),
        );
    }

    ret
}

pub struct Property {
    pub ident: syn::Ident,
    pub meta_type: syn::Type,
    pub version: u8,
    pub ty: syn::Type,
    pub constant: bool,
    pub readonly: bool,
    pub writeonly: bool,
    pub resize: bool,
    pub std: bool,
}

impl Property {
    pub fn implement_getter_via_trait(&self) -> bool {
        self.ident == "device_id" || self.ident == "source" || self.ident == "destination"
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
            version: parse_version(field),
            constant: common::is_unit_tuple(&field.ty),
            readonly: common::has_attr(field, "readonly"),
            writeonly: common::has_attr(field, "writeonly"),
            resize: common::has_attr(field, "resize"),
            std: common::has_attr(field, "std"),
        })
        .collect()
}

fn parse_version(field: &syn::Field) -> u8 {
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
                .any(|&segment| segment.ident.to_string() == "version")
        })
        .map(|list| {
            list.parse_args::<syn::LitInt>()
                .expect("Arguments to version attribute should be a valid int literal")
                .base10_parse::<u8>()
                .expect("Version literal should represent a valid u8")
        })
        .expect("fields must be annotated with the version attribute")
}

fn initialise_property_statements(
    properties: &Vec<Property>,
    buffer_type: TokenStream,
) -> TokenStream {
    let mut initialise_properties = TokenStream::new();
    for property in properties.iter().filter(|p| !p.readonly) {
        let meta_type = &property.meta_type;
        let std_only_attribute = common::std_only_attribute(property.std);
        let property_version = property.version;

        initialise_properties.extend(quote! {
            #std_only_attribute
            // this check should be optimised away by the compiler
            if #property_version <= VERSION {
                <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::write(
                    buffer_ref_mut,
                    <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::default(),
                );
            }
        });
    }
    initialise_properties
}

fn property_getter(property: &Property, public: bool) -> TokenStream {
    let meta_type = &property.meta_type;
    let ident = &property.ident;
    let version = property.version;
    let ty = &property.ty;
    let pub_token = if public {
        quote! { pub }
    } else {
        TokenStream::new()
    };
    let std_only_attribute = common::std_only_attribute(property.std);

    quote! {
        #std_only_attribute
        #pub_token fn #ident(&self) -> #ty where Self: crate::ci::version::CiVersion<#version> {
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
    let version = property.version;
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
            #pub_token fn #ident(&mut self, value: #ty)
            where
                B: crate::buffer::BufferMut + crate::buffer::BufferResize,
                Self: crate::ci::version::CiVersion<#version>,
            {
                <#meta_type as crate::detail::property::ResizeProperty<B>>::resize(self.buffer_access_mut(), &value);
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
            }

            #std_only_attribute
            #pub_token fn #fallible_ident(&mut self, value: #ty) -> core::result::Result<(), crate::error::BufferOverflow>
            where
                B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
                Self: crate::ci::version::CiVersion<#version>,
            {
                <#meta_type as crate::detail::property::ResizeProperty<B>>::try_resize(self.buffer_access_mut(), &value)?;
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
                Ok(())
            }
        }
    } else {
        quote! {
            #std_only_attribute
            #pub_token fn #ident(&mut self, value: #ty)
            where
                B: crate::buffer::BufferMut,
                Self: crate::ci::version::CiVersion<#version>,
            {
                <#meta_type as crate::detail::property::WriteProperty<B>>::write(self.buffer_access_mut(), value);
            }
        }
    }
}

fn imports() -> TokenStream {
    quote! {
        use crate::traits::BufferAccess as BufferAccessGenCi;
    }
}

fn message(root_ident: &syn::Ident, attributes: &Vec<syn::Attribute>) -> TokenStream {
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
        #[derive(Clone, PartialEq, Eq, midi2_proc::Debug)]
        #doc_attributes
        pub struct #root_ident<const VERSION: u8, B: crate::buffer::Bytes>(crate::sysex7::Sysex7<B>);
    }
}

fn buffer_access_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<const VERSION: u8, B: crate::buffer::Bytes> crate::traits::BufferAccess<B> for #root_ident<VERSION, B> {
            fn buffer_access(&self) -> &B {
                self.0.buffer_access()
            }
            fn buffer_access_mut(&mut self) -> &mut B
            where
                B: crate::buffer::BufferMut
            {
                self.0.buffer_access_mut()
            }
        }
    }
}

fn new_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let initialise_properties = initialise_property_statements(properties, quote! {B});
    quote! {
        impl<const VERSION: u8,
            B: crate::buffer::Bytes
                + crate::buffer::BufferMut
                + crate::buffer::BufferDefault
                + crate::buffer::BufferResize
        > #root_ident<VERSION, B>
        {
            /// Create a new message backed by a resizable buffer.
            pub fn new() -> Self
            {
                let _ = crate::ci::version::ValidCiVersion::<VERSION>::VALID;

                let mut sysex7 = crate::sysex7::Sysex7::<B>::new();
                let payload_size = <Self as crate::traits::MinSize<B>>::MIN_SIZE - 2;
                <crate::sysex7::Sysex7<B> as crate::SysexInternal<B>>::resize(&mut sysex7, payload_size);
                let buffer_ref_mut = <crate::sysex7::Sysex7<B> as crate::BufferAccess<B>>::buffer_access_mut(&mut sysex7);
                if buffer_ref_mut.buffer().len() > 5 {
                    // write the version
                    buffer_ref_mut.buffer_mut()[5] = VERSION;
                }
                #initialise_properties
                #root_ident::<VERSION, B>(sysex7)
            }
        }
    }
}

fn try_new_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let initialise_properties = initialise_property_statements(properties, quote! {B});
    quote! {
        impl<const VERSION: u8,
            B: crate::buffer::Bytes
                + crate::buffer::BufferMut
                + crate::buffer::BufferDefault
                + crate::buffer::BufferTryResize
        > #root_ident<VERSION, B>
        {
            /// Create a new message backed by a buffer with fallible resize.
            pub fn try_new() -> Result<Self, crate::error::BufferOverflow>
            {
                let _ = crate::ci::version::ValidCiVersion::<VERSION>::VALID;

                let mut sysex7 = crate::sysex7::Sysex7::<B>::try_new()?;
                let payload_size = <Self as crate::traits::MinSize<B>>::MIN_SIZE - 2;
                <crate::sysex7::Sysex7<B> as crate::SysexInternal<B>>::try_resize(&mut sysex7, payload_size)?;
                let buffer_ref_mut = <crate::sysex7::Sysex7<B> as crate::BufferAccess<B>>::buffer_access_mut(&mut sysex7);
                // write the version
                buffer_ref_mut.buffer_mut()[5] = VERSION;
                #initialise_properties
                Ok(#root_ident::<VERSION, B>(sysex7))
            }
        }
    }
}

fn ci_version_impls(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Bytes> crate::ci::version::CiVersion<0x1> for #root_ident<0x1, B> {}
        impl<B: crate::buffer::Bytes> crate::ci::version::CiVersion<0x2> for #root_ident<0x2, B> {}
    }
}

fn min_size_impl(root_ident: &syn::Ident, args: &GenerateCiArgs) -> TokenStream {
    if args.min_size.len() != 2 {
        panic!("Expected a min size for each CI version");
    }
    let size_v1_1 = args.min_size[0];
    let size_v1_2 = args.min_size[1];
    quote! {
        impl<const VERSION: u8, B: crate::buffer::Bytes> crate::traits::MinSize<B> for #root_ident<VERSION, B> {
            const MIN_SIZE: usize = match VERSION {
                0x1 => #size_v1_1,
                0x2 => #size_v1_2,
                _ => <crate::sysex7::Sysex7<B> as crate::traits::MinSize<B>>::MIN_SIZE,
            };
        }
    }
}

fn deref_sysex7_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<const VERSION: u8, B: crate::buffer::Bytes> core::ops::Deref for #root_ident<VERSION, B> {
            type Target = crate::sysex7::Sysex7<B>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}

fn message_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let mut methods = TokenStream::new();
    for property in properties.iter().filter(|p| !p.constant) {
        if !property.writeonly && !property.implement_getter_via_trait() {
            methods.extend(property_getter(property, true));
        }
        if !property.readonly {
            methods.extend(property_setter(property, true));
        }
    }

    quote! {
        impl<const VERSION: u8, B: crate::buffer::Bytes> #root_ident<VERSION, B> {
            #methods
        }
    }
}

fn ci_impl(root_ident: &syn::Ident) -> TokenStream {
    let mut methods = TokenStream::new();
    let make_property = |ident: syn::Ident, meta_type: syn::Type, ty: syn::Type| Property {
        ident,
        meta_type,
        ty,
        version: 0x1,
        constant: false,
        readonly: false,
        writeonly: false,
        resize: false,
        std: false,
    };
    methods.extend(property_getter(
        &make_property(
            syn::Ident::new("device_id", proc_macro2::Span::call_site()),
            syn::parse_quote! { crate::ci::common_properties::DeviceIdProperty },
            syn::parse_quote! {crate::ci::device_id::DeviceId},
        ),
        false,
    ));
    methods.extend(property_getter(
        &make_property(
            syn::Ident::new("source", proc_macro2::Span::call_site()),
            syn::parse_quote! { crate::ci::common_properties::SourceProperty },
            syn::parse_quote! { ux::u28 },
        ),
        false,
    ));
    methods.extend(property_getter(
        &make_property(
            syn::Ident::new("destination", proc_macro2::Span::call_site()),
            syn::parse_quote! { crate::ci::common_properties::DestinationProperty },
            syn::parse_quote! { ux::u28 },
        ),
        false,
    ));

    quote! {
        impl<const VERSION: u8, B: crate::buffer::Bytes> crate::ci::Ci<B> for #root_ident<VERSION, B> {
            #methods
        }
    }
}

fn try_from_slice_impl(root_ident: &syn::Ident, properties: &Vec<Property>) -> TokenStream {
    let mut validation_steps = TokenStream::new();
    for property in properties.iter().filter(|p| !p.writeonly) {
        let meta_type = &property.meta_type;
        let std_only_attribute = common::std_only_attribute(property.std);
        let property_version = property.version;

        validation_steps.extend(quote! {
            #std_only_attribute
            // this check should be optimised away by the compiler
            if #property_version <= VERSION {
                <#meta_type as crate::detail::property::ReadProperty<&[u8]>>::validate(buffer)?;
            }
        });
    }
    quote! {
        impl<'a, const VERSION: u8> core::convert::TryFrom<&'a [u8]> for #root_ident<VERSION, &'a [u8]> {
            type Error = crate::error::InvalidData;
            fn try_from(buffer: &'a [u8]) -> core::result::Result<Self, Self::Error> {
                if buffer.len() < <Self as crate::traits::MinSize<&[u8]>>::MIN_SIZE {
                    return Err(crate::error::InvalidData("Slice is too short"));
                }
                if buffer[5] != VERSION {
                    return Err(crate::error::InvalidData("Incorrect CI version"));
                }
                let sysex7 = crate::sysex7::Sysex7::try_from(buffer)?;
                let buffer = sysex7.buffer_access();
                #validation_steps
                Ok(#root_ident(sysex7))
            }
        }
    }
}

fn rebuffer_from_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<
                const VERSION: u8,
                A: crate::buffer::Bytes,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            > crate::traits::RebufferFrom<#root_ident<VERSION, A>> for #root_ident<VERSION, B> {
            fn rebuffer_from(other: #root_ident<VERSION, A>) -> Self {
                #root_ident(crate::sysex7::Sysex7::rebuffer_from(other.0))
            }
        }
    }
}

fn try_rebuffer_from_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<
                const VERSION: u8,
                A: crate::buffer::Bytes,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            > crate::traits::TryRebufferFrom<#root_ident<VERSION, A>> for #root_ident<VERSION, B> {
            fn try_rebuffer_from(other: #root_ident<VERSION, A>) -> Result<Self, crate::error::BufferOverflow> {
                Ok(#root_ident(crate::sysex7::Sysex7::try_rebuffer_from(other.0)?))
            }
        }
    }
}

pub fn generate_ci(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attrs as GenerateCiArgs);
    let properties = properties(&input);
    let root_ident = &input.ident;

    let imports = imports();
    let message = message(root_ident, &input.attrs);
    let buffer_access_impl = buffer_access_impl(root_ident);
    let min_size_impl = min_size_impl(root_ident, &args);
    let new_impl = new_impl(root_ident, &properties);
    let try_new_impl = try_new_impl(root_ident, &properties);
    let ci_version_impls = ci_version_impls(root_ident);
    let deref_sysex7_impl = deref_sysex7_impl(root_ident);
    let message_impl = message_impl(root_ident, &properties);
    let ci_impl = ci_impl(root_ident);
    let try_from_slice_impl = try_from_slice_impl(root_ident, &properties);
    let rebuffer_from_impl = rebuffer_from_impl(root_ident);
    let try_rebuffer_from_impl = try_rebuffer_from_impl(root_ident);

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        #imports
        #message
        #buffer_access_impl
        #min_size_impl
        #new_impl
        #try_new_impl
        #deref_sysex7_impl
        #message_impl
        #ci_impl
        #try_from_slice_impl
        #ci_version_impls
        #rebuffer_from_impl
        #try_rebuffer_from_impl
    });

    tokens.into()
}
