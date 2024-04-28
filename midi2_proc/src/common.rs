use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum Representation {
    Ump,
    Bytes,
    UmpOrBytes,
}

pub enum BufferGeneric {
    UmpOrBytes(syn::TypeParam),
    Ump(syn::TypeParam),
    Bytes(syn::TypeParam),
}

impl BufferGeneric {
    pub fn ident(&self) -> syn::Ident {
        match self {
            Self::UmpOrBytes(param) => param.ident.clone(),
            Self::Ump(param) => param.ident.clone(),
            Self::Bytes(param) => param.ident.clone(),
        }
    }
}

pub fn buffer_generic(generics: &syn::Generics) -> Option<BufferGeneric> {
    let type_param = |param: &syn::GenericParam| {
        if let syn::GenericParam::Type(type_param) = param {
            Some(type_param.clone())
        } else {
            None
        }
    };
    let trait_bound = |bound: &syn::TypeParamBound| {
        if let syn::TypeParamBound::Trait(trait_bound) = bound {
            Some(trait_bound.clone())
        } else {
            None
        }
    };
    let buffer_bound = |id: &'static str| {
        move |bound: &syn::TraitBound| match bound.path.segments.last().as_ref() {
            Some(segment) => segment.ident == id,
            None => false,
        }
    };
    for param in generics.params.iter().filter_map(type_param) {
        if let Some(_) = param
            .bounds
            .iter()
            .filter_map(trait_bound)
            .find(buffer_bound("Ump"))
        {
            return Some(BufferGeneric::Ump(param.clone()));
        };
        if let Some(_) = param
            .bounds
            .iter()
            .filter_map(trait_bound)
            .find(buffer_bound("Bytes"))
        {
            return Some(BufferGeneric::Bytes(param.clone()));
        };
        if let Some(_) = param
            .bounds
            .iter()
            .filter_map(trait_bound)
            .find(buffer_bound("Buffer"))
        {
            return Some(BufferGeneric::UmpOrBytes(param.clone()));
        };
    }
    None
}

pub fn rebuffer_generics(repr: Representation) -> TokenStream {
    match repr {
        Representation::Ump => quote! {
            <
                A: crate::buffer::Ump,
                B: crate::buffer::Ump
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            >
        },
        Representation::Bytes => quote! {
            <
                A: crate::buffer::Bytes,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            >
        },
        Representation::UmpOrBytes => quote! {
            <
                U: crate::buffer::Unit,
                A: crate::buffer::Buffer<Unit = U>,
                B: crate::buffer::Buffer<Unit = U>
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize,
            >
        },
    }
}

pub fn try_rebuffer_generics(repr: Representation) -> TokenStream {
    match repr {
        Representation::Ump => quote! {
            <
                A: crate::buffer::Ump,
                B: crate::buffer::Ump
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            >
        },
        Representation::Bytes => quote! {
            <
                A: crate::buffer::Bytes,
                B: crate::buffer::Bytes
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            >
        },
        Representation::UmpOrBytes => quote! {
            <
                U: crate::buffer::Unit,
                A: crate::buffer::Buffer<Unit = U>,
                B: crate::buffer::Buffer<Unit = U>
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferTryResize,
            >
        },
    }
}
