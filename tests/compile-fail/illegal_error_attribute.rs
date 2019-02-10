extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
#[error_type] //~ERROR Expected a name-value attribute, e.g. `#[error_type = "MyType"]`
pub struct MyStruct {
    field: String,
}
