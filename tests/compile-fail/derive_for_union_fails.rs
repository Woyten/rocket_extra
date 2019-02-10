extern crate rocket;
extern crate rocket_extra_codegen;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
pub union MyUnion {//~ERROR Should be a struct
    one: u32,
    two: usize,
}
