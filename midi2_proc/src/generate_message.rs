use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

struct Property {
    ident: syn::Ident,
    meta_type: TokenStream,
    ty: syn::Type,
    constant: bool,
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

fn message(root_ident: &syn::Ident, args: &GenerateMessageArgs) -> TokenStream {
    let constraint = generic_buffer_constraint(args);
    quote! {
        #[derive(PartialEq, Eq)]
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
    for property in properties.iter().filter(|p| !p.constant) {
        methods.extend(property_getter(property));
        methods.extend(property_setter(property));
    }

    quote! {
        impl<B: #constraint> #root_ident<B> {
            #methods
        }
    }
}

fn property_getter(property: &Property) -> TokenStream {
    let meta_type = &property.meta_type;
    let ident = &property.ident;
    let ty = &property.ty;
    quote! {
        pub fn #ident(&self) -> #ty {
            <#meta_type as crate::util::property::Property<B>>::read(&self.0).unwrap()
        }
    }
}

fn property_setter(property: &Property) -> TokenStream {
    let meta_type = &property.meta_type;
    let ident = syn::Ident::new(
        format!("set_{}", &property.ident.to_string()).as_str(),
        proc_macro2::Span::call_site(),
    );
    let ty = &property.ty;
    quote! {
        pub fn #ident(&mut self, value: #ty) -> #ty where B: crate::buffer::BufferMut {
            <#meta_type as crate::util::property::Property<B>>::write(&mut self.0, value).unwrap();
        }
    }
}

fn message_owned_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let owned_type = owned_type_ump(args);
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

fn owned_type_ump(args: &GenerateMessageArgs) -> TokenStream {
    match args.min_size_ump {
        Some(size) => quote! { [u32; #size] },
        None => quote! { std::vec::Vec<u32> },
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

fn try_from_slice_impl(
    root_ident: &syn::Ident,
    args: &GenerateMessageArgs,
    properties: &Vec<Property>,
) -> TokenStream {
    let mut validation_steps = TokenStream::new();
    let generic_unit = match args.representation() {
        Representation::UmpOrBytes => quote! { U: crate::buffer::Buffer },
        _ => TokenStream::new(),
    };
    let unit_type = match args.representation() {
        Representation::UmpOrBytes => quote! { U },
        Representation::Ump => quote! { u32 },
        Representation::Bytes => quote! { u8 },
    };
    for property in properties {
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

pub fn generate_message(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attrs as GenerateMessageArgs);
    let properties = properties(&input);
    let root_ident = &input.ident;

    let imports = imports();
    let message = message(root_ident, &args);
    let message_impl = message_impl(root_ident, &args, &properties);
    let message_owned_impl = message_owned_impl(root_ident, &args, &properties);
    let debug_impl = debug_impl(root_ident, &args);
    let try_from_slice_impl = try_from_slice_impl(root_ident, &args, &properties);

    quote! {
        #imports
        #message
        #message_impl
        #message_owned_impl
        #debug_impl
        #try_from_slice_impl
    }
    .into()
}
