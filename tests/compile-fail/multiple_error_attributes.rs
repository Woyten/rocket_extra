extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
#[error_type = "OneError"]
#[error_type = "AnotherError"] //~ERROR Found more than one `error_type` declaration
pub struct MyStruct {
    field: String,
}
