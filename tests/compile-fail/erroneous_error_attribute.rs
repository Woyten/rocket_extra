extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
#[error_type = 2] //~ERROR Invalid string literal
pub struct MyStruct {
    field: String,
}
