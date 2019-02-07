#![recursion_limit = "128"]

extern crate proc_macro;

static ERROR_TYPE_ATTRIBUTE: &str = "error_type";

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Fields;
use syn::Lit;
use syn::Meta;
use syn::Type;

#[proc_macro_derive(FromRequest, attributes(error_type))]
pub fn derive_from_request(input: TokenStream) -> TokenStream {
    try_derive_from_request(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn try_derive_from_request(input: TokenStream) -> Result<TokenStream, Error> {
    let ast = syn::parse::<DeriveInput>(input)?;

    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(struct_) => struct_.fields,
        Data::Enum(enum_) => return Err(Error::new_spanned(enum_.enum_token, "Should be a struct")),
        Data::Union(union_) => {
            return Err(Error::new_spanned(union_.union_token, "Should be a struct"));
        }
    };

    if !ast.generics.params.is_empty() {
        return Err(Error::new_spanned(
            ast.generics,
            "Generics are not yet supported",
        ));
    }

    if let Fields::Unnamed(unnamed) = fields {
        return Err(Error::new_spanned(
            unnamed,
            "Should be a struct with named fields",
        ));
    }

    let error_type_declaration = match get_error_type(&ast.attrs)? {
        Some(error_type) => quote! { type Error = #error_type; },
        None => quote! { type Error = (); },
    };

    let mut arms = Vec::new();
    let mut constructor = Vec::new();

    for field in &fields {
        let name = field.ident.as_ref().expect("Unexpected unnamed field");
        let ty = &field.ty;
        arms.push(quote! {
            let #name = match ::rocket::Request::guard::<#ty>(request) {
                ::rocket::Outcome::Success(user) => user,
                ::rocket::Outcome::Failure((status, error)) => return ::rocket::Outcome::Failure((status, ::std::convert::From::from(error))),
                ::rocket::Outcome::Forward(()) => return ::rocket::Outcome::Forward(()),
            };
        });

        constructor.push(quote! { #name: #name })
    }

    let trait_implementation = quote! {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for #name {
            #error_type_declaration

            fn from_request(request: &'a ::rocket::Request<'r>) -> ::rocket::Outcome<Self, (::rocket::http::Status, Self::Error), ()> {
                #(#arms)*
                ::rocket::Outcome::Success(#name { #(#constructor),*})
            }
        }
    };
    Ok(trait_implementation.into())
}

fn get_error_type(attrs: &[Attribute]) -> Result<Option<Type>, Error> {
    let mut error_type_decls = attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok().map(|meta| (attr, meta)))
        .filter(|(_attr, meta)| meta.name() == ERROR_TYPE_ATTRIBUTE)
        .collect::<Vec<_>>();

    let error_type_attr_meta = match (error_type_decls.pop(), error_type_decls.pop()) {
        (None, _) => return Ok(None),
        (Some((_attr, meta)), None) => meta,
        (Some((attr, _meta)), Some(_)) => {
            return Err(Error::new_spanned(
                attr,
                format!("Found more than one `{}` declaration", ERROR_TYPE_ATTRIBUTE),
            ));
        }
    };

    let name_value = if let Meta::NameValue(name_value) = error_type_attr_meta {
        name_value
    } else {
        return Err(Error::new_spanned(
            error_type_attr_meta,
            format!(
                "Expected a name-value attribute, e.g. `#[{} = \"MyType\"]`",
                ERROR_TYPE_ATTRIBUTE
            ),
        ));
    };

    match name_value.lit {
        Lit::Str(lit_str) => match lit_str.parse() {
            Ok(type_spec) => Ok(Some(type_spec)),
            Err(_) => Err(Error::new_spanned(lit_str, "Invalid type specifier")),
        },
        other => Err(Error::new_spanned(other, "Invalid string literal")),
    }
}
