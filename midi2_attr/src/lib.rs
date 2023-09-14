use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    token::{Colon, Comma, Gt, Lt, PathSep, Plus},
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, Ident, ItemStruct, Path,
    PathArguments, PathSegment, TraitBound, TraitBoundModifier, Type, TypeParam, TypeParamBound,
    TypePath, TypeTuple,
};

struct Property {
    name: Ident,
    constant: bool,
    ty: Type,
    ump_representation: Type,
    bytes_representation: Type,
}

#[derive(Clone, Copy)]
enum MessageRepresentation {
    Ump,
    UmpAndBytes,
}

fn deduce_message_representation(properties: &Vec<Property>) -> MessageRepresentation {
    let is_non_unit_type = |ty: &Type| -> bool {
        if let Type::Tuple(TypeTuple { elems, .. }) = ty {
            if elems.is_empty() {
                return false;
            }
        }
        true
    };
    use MessageRepresentation::*;
    if properties
        .iter()
        .map(|property| &property.bytes_representation)
        .any(is_non_unit_type)
    {
        UmpAndBytes
    } else {
        Ump
    }
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

fn message_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Message", root_ident), Span::call_site())
}

#[derive(Clone, Copy)]
enum StructType {
    Message,
    Builder,
}

fn generate_type(
    root_ident: &Ident,
    representation: MessageRepresentation,
    repr: Repr,
    struct_type: StructType,
) -> TokenStream {
    let ident = match struct_type {
        StructType::Message => message_ident(root_ident),
        StructType::Builder => builder_ident(root_ident),
    };
    use MessageRepresentation::*;
    match representation {
        Ump => quote! {
            #ident<'a>
        },
        UmpAndBytes => {
            let buffer_type: TokenStream = match repr {
                Repr::Ump => parse_str("Ump").unwrap(),
                Repr::Bytes => parse_str("Bytes").unwrap(),
                Repr::Generic => parse_str("B").unwrap(),
            };
            quote! {
                #ident<'a, #buffer_type>
            }
        }
    }
}

fn builder_ident(root_ident: &Ident) -> Ident {
    Ident::new(&format!("{}Builder", root_ident), Span::call_site())
}

fn imports() -> TokenStream {
    quote! {
        use crate::{
            message::helpers as message_helpers,
            traits::*,
            buffer::*,
            util::{schema::*, BitOps},
            *,
        };
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

fn message(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let ident = message_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => quote! {
            #[derive(Clone, PartialEq, Eq)]
            pub struct #ident <'a>(&'a [u32]);
        },
        UmpAndBytes => {
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                #[derive(Clone, PartialEq, Eq)]
                pub struct #ident<'a, #buffer_type>(&'a B::Data);
            }
        }
    }
}

fn builder(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let ident = builder_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => quote! {
            pub struct #ident<'a>(Option<&'a mut [u32]>);
        },
        UmpAndBytes => {
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                pub struct #ident<'a, #buffer_type>(Option<&'a mut B::Data>);
            }
        }
    }
}

fn message_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let ident = message_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => {
            let mut methods = TokenStream::new();
            for property in properties.iter().filter(|p| !p.constant) {
                methods.extend(message_impl_method(property, Repr::Ump, true));
            }
            quote! {
                impl<'a> #ident<'a> {
                    #methods
                }
            }
        }
        UmpAndBytes => {
            let mut methods = TokenStream::new();
            for property in properties.iter().filter(|p| !p.constant) {
                methods.extend(message_impl_method(property, Repr::Generic, true));
            }
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                impl<'a, #buffer_type> #ident<'a, B> {
                    #methods
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Repr {
    Ump,
    Bytes,
    Generic,
}

fn message_impl_method(property: &Property, repr: Repr, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let ump_schema = &property.ump_representation;
    let bytes_schema = &property.bytes_representation;
    let buffer_type: Type = match repr {
        Repr::Ump => parse_str("Ump").unwrap(),
        Repr::Bytes => parse_str("Bytes").unwrap(),
        Repr::Generic => parse_str("B").unwrap(),
    };
    let visibility = {
        let mut ret = TokenStream::new();
        if public {
            ret.extend(parse_str::<TokenStream>("pub").unwrap());
        }
        ret
    };
    quote! {
        #visibility fn #name(&self) -> #ty {
            <#buffer_type as Property<#ty, #ump_schema, #bytes_schema>>::get(self.0)
        }
    }
}

fn builder_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let ident = builder_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => {
            let mut methods = TokenStream::new();
            for property in properties.iter().filter(|p| !p.constant) {
                methods.extend(builder_impl_method(property, Repr::Ump, true));
            }
            quote! {
                impl<'a> #ident<'a> {
                    #methods
                }
            }
        }
        UmpAndBytes => {
            let mut methods = TokenStream::new();
            for property in properties.iter().filter(|p| !p.constant) {
                methods.extend(builder_impl_method(property, Repr::Generic, true));
            }
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                impl<'a, #buffer_type> #ident<'a, B> {
                    #methods
                }
            }
        }
    }
}

fn builder_impl_method(property: &Property, repr: Repr, public: bool) -> TokenStream {
    let name = &property.name;
    let ty = &property.ty;
    let ump_schema = &property.ump_representation;
    let bytes_schema = &property.bytes_representation;
    let buffer_type: Type = match repr {
        Repr::Ump => parse_str("Ump").unwrap(),
        Repr::Bytes => parse_str("Bytes").unwrap(),
        Repr::Generic => parse_str("B").unwrap(),
    };
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
                <#buffer_type as Property<#ty, #ump_schema, #bytes_schema>>::write(buffer, v);
            }
            self
        }
    }
}

fn message_trait_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let message_ident = message_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => {
            let validation_steps = validation_steps(properties, Repr::Ump);
            quote! {
                impl<'a> Message<'a, Ump> for #message_ident<'a> {
                    fn from_data_unchecked(data: &'a [u32]) -> Self {
                        #message_ident(data)
                    }
                    fn data(&self) -> &'a [u32] {
                        self.0
                    }
                    fn validate_data(buffer: &'a [u32]) -> Result<()> {
                        #validation_steps
                        if buffer.len() != <Ump as Buffer>::SIZE {
                            return Err(Error::InvalidData);
                        }
                        Ok(())
                    }
                }
            }
        }
        UmpAndBytes => {
            let validation_steps = validation_steps(properties, Repr::Generic);
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                impl<'a, #buffer_type> Message<'a, B> for #message_ident<'a, B> {
                    fn from_data_unchecked(data: &'a <B as Buffer>::Data) -> Self {
                        #message_ident(data)
                    }
                    fn data(&self) -> &'a <B as Buffer>::Data {
                        self.0
                    }
                    fn validate_data(buffer: &'a <B as Buffer>::Data) -> Result<()> {
                        #validation_steps
                        if <B as Buffer>::len(buffer) != <B as Buffer>::SIZE {
                            return Err(Error::InvalidData);
                        }
                        Ok(())
                    }
                }
            }
        }
    }
}

fn validation_steps(properties: &Vec<Property>, repr: Repr) -> TokenStream {
    let mut ret = TokenStream::new();
    for property in properties {
        let ty = &property.ty;
        let ump_schema = &property.ump_representation;
        let bytes_schema = &property.bytes_representation;
        let buffer_type: Type = match repr {
            Repr::Ump => parse_str("Ump").unwrap(),
            Repr::Bytes => parse_str("Bytes").unwrap(),
            Repr::Generic => parse_str("B").unwrap(),
        };
        ret.extend(quote! {
            <#buffer_type as Property<#ty, #ump_schema, #bytes_schema>>::validate(buffer)?;
        })
    }
    ret
}

fn builder_trait_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let repr = match representation {
        MessageRepresentation::Ump => Repr::Ump,
        MessageRepresentation::UmpAndBytes => Repr::Generic,
    };
    let repr_type: Type = match representation {
        MessageRepresentation::Ump => parse_str("Ump").unwrap(),
        MessageRepresentation::UmpAndBytes => parse_str("B").unwrap(),
    };
    let impl_declaration: TokenStream = match representation {
        MessageRepresentation::Ump => parse_str("impl<'a>").unwrap(),
        MessageRepresentation::UmpAndBytes => {
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                impl<'a, #buffer_type>
            }
        }
    };
    let builder_type = generate_type(root_ident, representation, repr, StructType::Builder);
    let message_type = generate_type(root_ident, representation, repr, StructType::Message);
    let message_ident = message_ident(root_ident);
    let write_const_data = builder_new_write_const_data(properties, repr);
    quote! {
        #impl_declaration Builder<'a, #repr_type> for #builder_type {
            type Message = #message_type;
            fn new(buffer: &'a mut <#repr_type as Buffer>::Data) -> Self {
                if <#repr_type as Buffer>::len(buffer) >= <#repr_type as Buffer>::SIZE {
                    <#repr_type as Buffer>::clear(buffer);
                    #write_const_data
                    Self(Some(<#repr_type as Buffer>::crop(buffer)))
                } else {
                    Self(None)
                }
            }
            fn build(self) -> Result<#message_type> {
                if let Some(buffer) = self.0 {
                    Ok(#message_ident(buffer))
                } else {
                    Err(Error::BufferOverflow)
                }
            }
        }
    }
}

fn builder_new_write_const_data(properties: &Vec<Property>, repr: Repr) -> TokenStream {
    let mut ret = TokenStream::new();
    let repr_type: Type = match repr {
        Repr::Ump => parse_str("Ump").unwrap(),
        Repr::Bytes => parse_str("Bytes").unwrap(),
        Repr::Generic => parse_str("B").unwrap(),
    };
    for property in properties.iter().filter(|property| property.constant) {
        let ump_schema = &property.ump_representation;
        let bytes_schema = &property.bytes_representation;
        let ty = &property.ty;
        ret.extend(quote! {
            <#repr_type as Property::<#ty, #ump_schema, #bytes_schema>>::write(buffer, Default::default());
        })
    }
    ret
}

fn buildable_trait_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
    properties: &Vec<Property>,
) -> TokenStream {
    let builder_ident = builder_ident(root_ident);
    let message_ident = message_ident(root_ident);
    use MessageRepresentation::*;
    match representation {
        Ump => quote! {
            impl<'a> Buildable<'a, Ump> for #message_ident<'a> {
                type Builder = #builder_ident<'a>;
            }
        },
        UmpAndBytes => {
            let buffer_type = buffer_generic_with_constraints(properties);
            quote! {
                impl<'a, #buffer_type> Buildable<'a, B> for #message_ident<'a, B> {
                    type Builder = #builder_ident<'a, B>;
                }
            }
        }
    }
}

fn grouped_message_trait_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
) -> TokenStream {
    let message_type = generate_type(root_ident, representation, Repr::Ump, StructType::Message);
    quote! {
        impl<'a> GroupedMessage<'a> for #message_type {
            fn group(&self) -> u4 {
                self.0[0].nibble(1)
            }
        }
    }
}

fn grouped_builder_trait_impl(
    root_ident: &Ident,
    representation: MessageRepresentation,
) -> TokenStream {
    let builder_type = generate_type(root_ident, representation, Repr::Ump, StructType::Builder);
    let ty: Type = parse_str("u4").unwrap();
    let function = builder_impl_method(
        &Property {
            name: Ident::new("group", Span::call_site()),
            constant: false,
            ty,
            ump_representation: parse_str("UmpSchema<0x0F00_0000, 0x0, 0x0, 0x0>").unwrap(),
            bytes_representation: parse_str("()").unwrap(),
        },
        Repr::Ump,
        false,
    );
    quote! {
        impl<'a> GroupedBuilder<'a> for #builder_type {
            #function
        }
    }
}

fn debug_impl(root_ident: &Ident, representation: MessageRepresentation) -> TokenStream {
    let ump_message_type =
        generate_type(root_ident, representation, Repr::Ump, StructType::Message);
    let message_ident = message_ident(root_ident);
    let mut ret = quote! {
        impl<'a> core::fmt::Debug for #ump_message_type {
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
    };
    if let MessageRepresentation::UmpAndBytes = representation {
        let bytes_message_type =
            generate_type(root_ident, representation, Repr::Bytes, StructType::Message);
        ret.extend(quote! {
            impl<'a> core::fmt::Debug for #bytes_message_type {
                fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                    fmt.write_fmt(format_args!("{}(", stringify!(#message_ident)))?;
                    let mut iter = self.0.iter().peekable();
                    while let Some(v) = iter.next() {
                        fmt.write_fmt(format_args!("{v:#04X}"))?;
                        if iter.peek().is_some() {
                            fmt.write_str(",")?;
                        }
                    }
                    fmt.write_str(")")
                }
            }
        })
    }
    ret
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
    let representation = deduce_message_representation(&properties);

    let imports = imports();
    let message = message(&root_ident, representation, &properties);
    let message_impl = message_impl(&root_ident, representation, &properties);
    let message_trait_impl = message_trait_impl(&root_ident, representation, &properties);
    let builder = builder(&root_ident, representation, &properties);
    let builder_impl = builder_impl(&root_ident, representation, &properties);
    let buildable_trait_impl = buildable_trait_impl(&root_ident, representation, &properties);
    let builder_trait_impl = builder_trait_impl(&root_ident, representation, &properties);
    let grouped_message_impl = grouped_message_trait_impl(&root_ident, representation);
    let grouped_builder_impl = grouped_builder_trait_impl(&root_ident, representation);
    let debug_impl = debug_impl(&root_ident, representation);

    quote! {
        #imports
        #message
        #message_impl
        #message_trait_impl
        #builder
        #builder_impl
        #buildable_trait_impl
        #builder_trait_impl
        #grouped_message_impl
        #grouped_builder_impl
        #debug_impl
    }
    .into()
}
