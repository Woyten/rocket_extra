extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
pub struct MyStruct<T>{ //~ERROR Generics are not yet supported
    field: T
}