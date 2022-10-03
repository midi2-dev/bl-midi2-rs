use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Getters, attributes(getters))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_impl = impl_struct(&input.ident, &input.data);
    quote!(#struct_impl).into()
}

fn impl_struct(struct_ident: &syn::Ident, data: &syn::Data) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data
    {
        let getters = named.iter().map(getter);
        quote!(
            impl #struct_ident {
                #(#getters)*
            }
        )
    } else {
        unreachable!()
    }
}

fn getter(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    if has_attr(field, "ref") {
        quote!(
            fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        )
    } else if let Some(ty) = slice_data_type(&field.ty) {
        quote!(
            fn #field_name(&self) -> &[#ty] {
                self.#field_name.data()
            }
        )
    } else {
        quote!(
            fn #field_name(&self) -> #field_type {
                self.#field_name
            }
        )
    }
}

fn slice_data_type<'t>(ty: &'t syn::Type) -> Option<&'t syn::Type> {
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
            if ident == "SliceData" {
                if let core::option::Option::Some(syn::GenericArgument::Type(ty)) = args.first() {
                    return core::option::Option::Some(ty);
                }
            }
        }
    }
    None
}

fn has_attr(field: &syn::Field, attr_name: &str) -> bool {
    for attr in &field.attrs {
        if attr.path.is_ident("getters") {
            if let std::result::Result::Ok(syn::Meta::List(list)) = attr.parse_meta() {
                for nested_meta in list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested_meta {
                        let attr_ident = &syn::Ident::new(attr_name, proc_macro2::Span::call_site());
                        if path.is_ident(attr_ident) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
