extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
pub enum MyEnum { //~ERROR Should be a struct
    One,
    Two,
}
