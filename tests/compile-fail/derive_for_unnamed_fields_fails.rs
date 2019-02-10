extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
pub struct MyStruct(usize,usize); //~ERROR Should be a struct with named fields
