extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
#[error_type = "Defect Type"] //~ERROR Invalid type specifier
pub struct MyStruct {
    field: String,
}
