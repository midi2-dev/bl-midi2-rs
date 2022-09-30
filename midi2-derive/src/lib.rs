use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident_original = input.ident;

    let original_struct_impl = impl_struct(&struct_ident_original, &input.data);
    let builder_struct = builder_struct(&input.data);
    let builder_impl = builder_impl(&struct_ident_original, &input.data);

    quote!(
        #original_struct_impl
        #builder_struct
        #builder_impl
    )
    .into()
}

fn impl_struct(struct_ident: &syn::Ident, data: &syn::Data) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data
    {
        let builder_struct_inits = named.iter().map(initialize_field);
        quote!(
            impl #struct_ident {
                pub fn builder() -> Builder {
                    Builder {
                        #(#builder_struct_inits),*
                    }
                }
            }
        )
    } else {
        unreachable!()
    }
}

fn builder_struct(data: &syn::Data) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data
    {
        let builder_struct_fields = named.iter().map(optionize_field);
        quote!(
            pub struct Builder {
                #(#builder_struct_fields),*
            }
        )
    } else {
        unreachable!()
    }
}

fn builder_impl(
    struct_ident: &syn::Ident, 
    data: &syn::Data,
) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data
    {
        let builder_impl_functions = named.iter().map(functionize_field);
        let assigned_fields = named.iter().map(assign_field);
        quote!(
            impl Builder {
                pub fn build(&mut self) -> core::result::Result<#struct_ident, Error> {
                    Ok(#struct_ident {
                        #(#assigned_fields),*
                    })
                }
                #(#builder_impl_functions)*
            }
        )
    } else {
        unreachable!()
    }
}

fn initialize_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let ref field_name = field.ident;
    quote!(#field_name: core::option::Option::None)
}

fn assign_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let ref field_name = field.ident;
    let ref field_type = field.ty;
    if extract_inner_type(field_type, "Option").is_some() {
        quote!(#field_name: self.#field_name.clone())
    } else {
        if let Some(default) = field.attrs.iter().find_map(default_field_value) {
            quote!(
                #field_name: match self.#field_name {
                    Some(v) => v,
                    None => #default,
                }
            )
        } else {
            quote!(#field_name: self.#field_name.clone().ok_or(Error::MissingFields)?)
        }
    }
}

fn default_field_value(attr: &syn::Attribute) -> core::option::Option<syn::Ident> {
    if attr.path.is_ident("builder") {
        if let std::result::Result::Ok(syn::Meta::List(list)) = attr.parse_meta() {
            for nested_meta in list.nested {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested_meta {
                    if name_value.path.is_ident("default") {
                        if let syn::Lit::Str(s) = name_value.lit {
                            return Some(format_ident!("{}", s.value()))
                        }
                    }
                }
            }
        }
    }
    None
}

fn functionize_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let mut field_type = &field.ty;
    if let core::option::Option::Some(inner_ty) = extract_inner_type(field_type, "Option") {
        field_type = inner_ty;
    }
    let setter_function_def = setter(field_name, field_type);
    quote!(
       #setter_function_def
    )
}

fn setter(field_name: &syn::Ident, field_type: &syn::Type) -> proc_macro2::TokenStream {
    quote!(fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
        self.#field_name = core::option::Option::Some(#field_name);
        self
    })
}

fn optionize_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let ref field_name = field.ident;
    let ref field_type = field.ty;
    if extract_inner_type(field_type, "Option").is_some() {
        quote!(#field_name: #field_type)
    } else {
        quote!(#field_name: core::option::Option<#field_type>)
    }
}

fn extract_inner_type<'t>(ty: &'t syn::Type, expected_ident: &str) -> Option<&'t syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if let core::option::Option::Some(syn::PathSegment {
            ident,
            arguments:
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }),
        }) = segments.last()
        {
            if ident == expected_ident {
                if let core::option::Option::Some(syn::GenericArgument::Type(ty)) = args.last() {
                    return core::option::Option::Some(ty);
                }
            }
        }
    }
    None
}
