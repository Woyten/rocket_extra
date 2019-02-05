#![recursion_limit = "128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Data;
use syn::Error;

#[proc_macro_derive(FromRequest)]
pub fn dervive_from_request(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    try_derive_from_request(&ast)
}

fn try_derive_from_request(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(struct_) => &struct_.fields,
        Data::Enum(enum_) => {
            return Error::new_spanned(enum_.enum_token, "Should be a struct")
                .to_compile_error()
                .into();
        }
        Data::Union(union_) => {
            return Error::new_spanned(union_.union_token, "Should be a struct")
                .to_compile_error()
                .into();
        }
    };

    let mut arms = Vec::new();
    let mut constructor = Vec::new();

    for field in fields.into_iter() {
        let name = field.clone().ident.unwrap();
        let ty = &field.ty;
        arms.push(quote! {
            let #name = match ::rocket::Request::guard::<#ty>(request) {
                ::rocket::Outcome::Success(user) => user,
                ::rocket::Outcome::Failure(error) => return ::rocket::Outcome::Failure(error),
                ::rocket::Outcome::Forward(()) => return ::rocket::Outcome::Forward(()),
            };
        });

        constructor.push(quote! { #name: #name })
    }

    let trait_implementation = quote! {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for #name {
            type Error = ();

            fn from_request(request: &'a ::rocket::Request<'r>) -> ::rocket::Outcome<Self, (::rocket::http::Status, Self::Error), ()> {
                #(#arms)*
                ::rocket::Outcome::Success(#name { #(#constructor),*})
            }
        }
    };
    trait_implementation.into()
}
