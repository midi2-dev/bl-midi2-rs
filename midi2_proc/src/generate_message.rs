use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

struct Property {
    ident: syn::Ident,
    meta_type: TokenStream,
    ty: syn::Type,
    constant: bool,
}

impl Property {
    fn implement_via_trait(&self) -> bool {
        self.is_group() || self.is_channel()
    }
    fn is_group(&self) -> bool {
        self.ident == "group"
    }
    fn is_channel(&self) -> bool {
        self.ident == "channel"
    }
}

#[allow(dead_code)]
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
        })
        .collect()
}

fn is_unit_tuple(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Tuple(tup) => tup.elems.len() == 0,
        _ => false,
    }
}

#[derive(Debug, Default)]
struct GenerateMessageArgs {
    fixed_size: bool,
    min_size_ump: Option<usize>,
    min_size_bytes: Option<usize>,
}

enum Representation {
    Ump,
    Bytes,
    UmpOrBytes,
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

fn imports() -> TokenStream {
    quote! {
        use crate::buffer::UnitPrivate as UnitPrivateGenMessage;
        use crate::util::property::Property as PropertyGenMessage;
    }
}

fn generic_buffer_constraint(args: &GenerateMessageArgs) -> TokenStream {
    match args.representation() {
        Representation::UmpOrBytes => quote! { crate::buffer::Buffer },
        Representation::Bytes => quote! { crate::buffer::Bytes },
        Representation::Ump => quote! { crate::buffer::Ump },
    }
}

fn message(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        #[derive(PartialEq, Eq)]
        pub struct #root_ident<B: crate::buffer::Buffer>(B);
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
            <#meta_type as crate::util::property::Property<B>>::read(&self.0).unwrap()
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
            <#meta_type as crate::util::property::Property<B>>::write(&mut self.0, value).unwrap();
        }
    }
}

fn message_new_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let owned_type = match args.representation() {
        Representation::Bytes => owned_type_bytes(args),
        Representation::Ump => owned_type_ump(args),
        Representation::UmpOrBytes => owned_type_ump(args),
    };
    let mut set_defaults = TokenStream::new();
    for property in properties {
        let meta_type = &property.meta_type;
        set_defaults.extend(quote! {
            <#meta_type as crate::util::property::Property<#owned_type>>::write(
                &mut buffer,
                <#meta_type as crate::util::property::Property<#owned_type>>::default(),
            ).unwrap();
        });
    }
    quote! {
        impl #root_ident<#owned_type> {
            pub fn new() -> Self {
                let mut buffer: #owned_type = core::default::Default::default();
                #set_defaults
                #root_ident(buffer)
            }
        }
    }
}

fn secondary_new_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let owned_type = owned_type_bytes(args);
    let mut set_defaults = TokenStream::new();
    for property in properties.iter().filter(|p| !p.is_group()) {
        let meta_type = &property.meta_type;
        set_defaults.extend(quote! {
            <#meta_type as crate::util::property::Property<#owned_type>>::write(
                &mut buffer,
                <#meta_type as crate::util::property::Property<#owned_type>>::default(),
            ).unwrap();
        });
    }
    quote! {
        impl #root_ident<#owned_type> {
            pub fn new_bytes() -> Self {
                let mut buffer: #owned_type = core::default::Default::default();
                #set_defaults
                #root_ident(buffer)
            }
        }
    }
}

fn owned_type_ump(args: &GenerateMessageArgs) -> TokenStream {
    match args.min_size_ump {
        Some(size) => quote! { [u32; #size] },
        None => quote! { std::vec::Vec<u32> },
    }
}

fn owned_type_bytes(args: &GenerateMessageArgs) -> TokenStream {
    match args.min_size_bytes {
        Some(size) => quote! { [u8; #size] },
        None => quote! { std::vec::Vec<u8> },
    }
}

fn size_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
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
    quote! {
        impl<B: crate::buffer::Buffer> crate::traits::Size<B> for #root_ident<B> {
            fn size(&self) -> usize {
                #body
            }
        }
    }
}

fn debug_impl(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        impl<B: #constraint> core::fmt::Debug for #root_ident<B> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}([", stringify!(#root_ident)))?;
                match <<B as crate::buffer::Buffer>::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                    crate::buffer::UNIT_ID_U8 => {
                        let mut iter = self.0.buffer().iter().peekable();
                        while let Some(v) = iter.next() {
                            fmt.write_fmt(format_args!("{:#04X}", v.specialise_u8()))?;
                            if iter.peek().is_some() {
                                fmt.write_str(", ")?;
                            }
                        }
                    }
                    crate::buffer::UNIT_ID_U32 => {
                        let mut iter = self.0.buffer().iter().peekable();
                        while let Some(v) = iter.next() {
                            fmt.write_fmt(format_args!("{:#010X}", v.specialise_u32()))?;
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
}

fn data_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Buffer> crate::traits::Data<B> for #root_ident<B> {
            fn data(&self) -> &[B::Unit] {
                self.0.buffer()
            }
        }
    }
}

fn with_buffer_impl(root_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<B: crate::buffer::Buffer> crate::traits::WithBuffer<B> for #root_ident<B> {
            fn with_buffer(buffer: B) -> Self {
                Self(buffer)
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
    for property in properties.iter().filter(|p| !p.implement_via_trait()) {
        let meta_type = &property.meta_type;
        validation_steps.extend(quote! {
            <#meta_type as PropertyGenMessage<&[#unit_type]>>::read(&buffer)?;
        });
    }
    quote! {
        impl<'a, #generic_unit> core::convert::TryFrom<&'a [#unit_type]> for #root_ident<&'a [#unit_type]> {
            type Error = crate::error::Error;
            fn try_from(buffer: &'a [#unit_type]) -> Result<Self, Self::Error> {
                #validation_steps
                Ok(#root_ident(buffer))
            }
        }
    }
}

fn from_message_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut copy_properties = TokenStream::new();
    for property in properties.iter().filter(|p| !p.is_group()) {
        let meta_type = &property.meta_type;
        copy_properties.extend(quote! {
            <#meta_type as crate::util::property::Property<C>>::write(
                &mut buffer,
                <#meta_type as crate::util::property::Property<B>>::read(&value.0).unwrap(),
            ).unwrap();
        });
    }
    quote! {
        impl<
                U: crate::buffer::Unit,
                B: crate::buffer::Buffer<Unit = U>,
                C: crate::buffer::Buffer<Unit = U> + crate::buffer::BufferMut + crate::buffer::BufferDefault,
            > crate::traits::FromMessage<#root_ident<B>> for #root_ident<C>
        {
            fn from_message(value: #root_ident<B>) -> Self {
                let mut buffer = <C as crate::buffer::BufferDefault>::default();
                #copy_properties
                Self(buffer)
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

pub fn generate_message(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attrs as GenerateMessageArgs);
    let properties = properties(&input);
    let root_ident = &input.ident;

    let imports = imports();
    let message = message(root_ident);
    let message_impl = message_impl(root_ident, &args, &properties);
    let message_new_impl = message_new_impl(root_ident, &args, &properties);
    let debug_impl = debug_impl(root_ident, &args);
    let data_impl = data_impl(root_ident);
    let with_buffer_impl = with_buffer_impl(root_ident);
    let try_from_slice_impl = try_from_slice_impl(root_ident, &args, &properties);
    let from_message_impl = from_message_impl(root_ident, &args, &properties);

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        #imports
        #message
        #message_impl
        #message_new_impl
        #debug_impl
        #data_impl
        #with_buffer_impl
        #try_from_slice_impl
        // #from_message_impl
    });

    if let Representation::UmpOrBytes = args.representation() {
        tokens.extend(secondary_new_impl(root_ident, &args, &properties))
    }
    if args.fixed_size {
        tokens.extend(size_impl(root_ident, &args))
    }
    if let Some(property) = properties.iter().find(|p| p.is_group()) {
        tokens.extend(grouped_impl(root_ident, property));
    }
    if let Some(property) = properties.iter().find(|p| p.is_channel()) {
        tokens.extend(channeled_impl(root_ident, property));
    }

    tokens.into()
}
